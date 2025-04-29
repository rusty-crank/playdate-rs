use alloc::string::String;
use alloc::string::ToString;
use alloc::{format, vec, vec::Vec};
use no_std_io::io::ErrorKind;
use rand::Rng;
use url::Url;

use crate::error::Error;
use crate::PLAYDATE;

use super::http::Headers;
use super::tcp::TCPConnection;

pub struct WebSocket {
    url: Url,
    conn: TCPConnection,
}

impl WebSocket {
    pub async fn connect(url: impl TryInto<Url>, headers: &Headers) -> Result<Self, Error> {
        let url: Url = url.try_into().map_err(|_| ErrorKind::InvalidInput)?;
        let mut host = url.clone();
        host.set_path("/");
        host.set_query(None);
        let ssl = url.scheme() == "wss" || url.scheme() == "https";
        let mut conn = TCPConnection::new(host, ssl)?;
        conn.set_timeout(5000);
        conn.open().await?;
        let mut ws = Self { url, conn };
        ws.handshake(headers).await?;
        Ok(ws)
    }

    fn create_ws_key() -> String {
        use base64::prelude::*;
        let mut rng = crate::util::rand::rng();
        let mut key = [0u8; 16];
        rng.fill(&mut key);
        let key = BASE64_STANDARD.encode(&key);
        key
    }

    fn mask() -> [u8; 4] {
        let mut rng = crate::util::rand::rng();
        let mut key = [0u8; 4];
        rng.fill(&mut key);
        key
    }

    async fn handshake(&mut self, headers: &Headers) -> Result<(), Error> {
        let key = Self::create_ws_key(); // Example key, should be generated
        let mut path_and_query = self.url.path().to_string();
        if let Some(query) = self.url.query() {
            path_and_query.push('?');
            path_and_query.push_str(query);
        }
        let mut msg = format!(
            "GET {} HTTP/1.1\r\nHost: {}\r\nOrigin: {}\r\nUpgrade: websocket\r\nConnection: Upgrade\r\nSec-WebSocket-Key: {}\r\nSec-WebSocket-Version: 13\r\n",
            path_and_query,
            self.url.host_str().unwrap_or(""),
            self.url.origin().ascii_serialization(),
            key
        );
        msg.push_str(&headers.build());
        msg.push_str("\r\n");
        self.conn.send(msg.as_bytes()).await.unwrap();
        self.conn.wait_for_data().await;
        // Receive the response
        let buf = self.conn.recv_all().await?;
        let Some(pos) = buf.windows(4).position(|window| window == b"\r\n\r\n") else {
            return Err(ErrorKind::InvalidData.into());
        };
        let headers = &buf[..pos + 4];
        let headers_str = core::str::from_utf8(headers).map_err(|_| ErrorKind::InvalidData)?;
        if !headers_str.starts_with("HTTP/1.1 101") {
            return Err(ErrorKind::InvalidData.into());
        }
        // TODO: Validate Sec-WebSocket-Accept
        Ok(())
    }

    fn create_frame(data: &[u8]) -> Vec<u8> {
        // Create a websocket frame
        let mask = Self::mask();
        let len = data.len();
        let mut header = if len < 126 {
            vec![0x81, 0x80 | len as u8]
        } else if len < 65536 {
            vec![0x81, 0x80 | 126, (len >> 8) as u8, len as u8]
        } else {
            let mut frame = vec![0x81, 0x80 | 127];
            frame.extend_from_slice(&(len as u64).to_be_bytes());
            frame
        };
        let masked = data
            .iter()
            .enumerate()
            .map(|(i, &b)| b ^ mask[i % 4])
            .collect::<Vec<u8>>();
        header.extend(mask);
        header.extend(masked);
        header
    }

    async fn receive_exact(&mut self, buf: &mut [u8]) -> Result<(), Error> {
        let mut len = 0;
        let cap = buf.len();
        while len < cap {
            let read_size = usize::min(cap - len, self.conn.get_bytes_available());
            let mut x = vec![0; read_size];
            self.conn.recv(&mut x).await?;
            buf[len..len + read_size].copy_from_slice(&x);
            len += read_size;
            PLAYDATE.yield_now().await;
        }
        Ok(())
    }

    async fn receive_exact_vec(&mut self, buf: &mut Vec<u8>) -> Result<(), Error> {
        assert_eq!(buf.len(), 0);
        let cap = buf.capacity();
        while buf.len() < cap {
            let read_size = usize::min(cap - buf.len(), self.conn.get_bytes_available());
            let mut x = vec![0; read_size];
            self.conn.recv(&mut x).await?;
            buf.extend_from_slice(&x);
            PLAYDATE.yield_now().await;
        }
        Ok(())
    }

    async fn parse_frame(&mut self) -> Result<(usize, Vec<u8>), Error> {
        // Parse a websocket frame, returning the opcode and payload
        let mut x = [0u8; 2];
        self.receive_exact(&mut x).await?;
        let (b1, b2) = (x[0], x[1]);
        // let fin = (b1 & 0x80) != 0;
        let opcode = (b1 & 0x0F) as usize;
        let masked = (b2 & 0x80) != 0;
        let mut len = (b2 & 0x7F) as usize;
        if len == 126 {
            let mut x = [0u8; 2];
            self.receive_exact(&mut x).await?;
            len = u16::from_be_bytes(x) as usize;
        } else if len == 127 {
            let mut x = [0u8; 8];
            self.receive_exact(&mut x).await?;
            len = u64::from_be_bytes(x) as usize;
        }
        let mask = if masked {
            let mut x = [0u8; 4];
            self.receive_exact(&mut x).await?;
            Some(x)
        } else {
            None
        };
        let mut payload = Vec::with_capacity(len);
        self.receive_exact_vec(&mut payload).await?;
        if let Some(mask) = mask {
            payload = payload
                .iter()
                .enumerate()
                .map(|(i, &b)| b ^ mask[i % 4])
                .collect();
        }
        return Ok((opcode, payload));
    }

    pub async fn send(&mut self, data: &[u8]) -> Result<(), Error> {
        let frame = Self::create_frame(data);
        self.conn.send(&frame).await.unwrap();
        Ok(())
    }

    pub async fn send_string(&mut self, data: impl AsRef<str>) -> Result<(), Error> {
        self.send(data.as_ref().as_bytes()).await
    }

    pub async fn send_json<T: serde::Serialize>(&mut self, data: &T) -> Result<(), Error> {
        let data = serde_json::to_string(data).map_err(|_| ErrorKind::InvalidData)?;
        self.send_string(data).await
    }

    pub async fn recv(&mut self) -> Result<Vec<u8>, Error> {
        self.conn.wait_for_data().await;
        let (_op, data) = self.parse_frame().await.unwrap();
        Ok(data)
    }

    pub async fn recv_string(&mut self) -> Result<String, Error> {
        let data = self.recv().await?;
        let s = core::str::from_utf8(&data).map_err(|_| ErrorKind::InvalidData)?;
        Ok(s.to_string())
    }

    pub async fn recv_json<T: serde::de::DeserializeOwned>(&mut self) -> Result<T, Error> {
        let data = self.recv_string().await?;
        let result = serde_json::from_str(data.as_str());
        match result {
            Ok(value) => Ok(value),
            Err(_e) => Err(ErrorKind::InvalidData.into()),
        }
    }

    pub fn close(&mut self) {
        self.conn.close()
    }

    pub fn is_closed(&self) -> bool {
        self.conn.is_closed()
    }
}
