use alloc::string::String;
use alloc::string::ToString;
use alloc::{format, vec, vec::Vec};
use no_std_io::io::ErrorKind;
use rand::Rng;
use url::Url;

use crate::error::Error;

use super::tcp::TCPConnection;

pub struct WebSocket {
    url: Url,
    conn: TCPConnection,
}

impl WebSocket {
    pub async fn connect(url: impl TryInto<Url>) -> Result<Self, Error> {
        let url: Url = url.try_into().map_err(|_| ErrorKind::InvalidInput)?;
        let mut host = url.clone();
        host.set_path("/");
        host.set_query(None);
        let ssl = url.scheme() == "wss" || url.scheme() == "https";
        let mut conn = TCPConnection::new(host, ssl)?;
        conn.set_timeout(5000);
        conn.open().await?;
        let mut ws = Self { url, conn };
        ws.handshake().await?;
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

    async fn handshake(&mut self) -> Result<(), Error> {
        let key = Self::create_ws_key(); // Example key, should be generated
        let msg = format!(
            "GET {} HTTP/1.1\r\nHost: {}\r\nOrigin: {}\r\nUpgrade: websocket\r\nConnection: Upgrade\r\nSec-WebSocket-Key: {}\r\nSec-WebSocket-Version: 13\r\n\r\n",
            self.url.path(),
            self.url.host_str().unwrap_or(""),
            self.url.origin().ascii_serialization(),
            key
        );
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

    fn parse_frame(data: &[u8]) -> Result<(usize, Vec<u8>), Error> {
        // Parse a websocket frame, returning the opcode and payload
        if data.len() < 2 {
            return Err(ErrorKind::InvalidData.into());
        }
        let (b1, b2) = (data[0], data[1]);
        // let fin = (b1 & 0x80) != 0;
        let opcode = (b1 & 0x0F) as usize;
        let masked = (b2 & 0x80) != 0;
        let mut len = (b2 & 0x7F) as usize;
        let mut cursor = 2;
        if len == 126 {
            len = ((data[2] as usize) << 8) | (data[3] as usize);
            cursor += 2;
        } else if len == 127 {
            len = u64::from_be_bytes(data[2..10].try_into().unwrap()) as usize;
            cursor += 8;
        }
        let mask = if masked {
            let x = &data[cursor..cursor + 4];
            cursor += 4;
            Some(x)
        } else {
            None
        };
        let mut payload = data[cursor..].to_vec();
        if payload.len() != len {
            return Err(ErrorKind::InvalidData.into());
        }
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
        // let t = crate::PLAYDATE.system.get_current_time_milliseconds();
        self.conn.wait_for_data().await;
        // let t2 = crate::PLAYDATE.system.get_current_time_milliseconds();
        // println!("waited for data: {}ms", t2 - t);
        let buf = self.conn.recv_all().await.unwrap();
        let (_op, data) = Self::parse_frame(&buf).unwrap();
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
}
