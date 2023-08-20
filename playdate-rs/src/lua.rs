use alloc::{borrow::ToOwned, ffi::CString};

use crate::error::Error;

pub struct Lua {
    #[allow(unused)]
    handle: *const sys::playdate_lua,
}

impl Lua {
    pub(crate) fn new(handle: *const sys::playdate_lua) -> Self {
        Self { handle }
    }

    pub fn add_function(&self, f: sys::lua_CFunction, name: impl AsRef<str>) -> Result<(), Error> {
        let c_string = CString::new(name.as_ref()).unwrap();
        unsafe {
            let mut err = core::ptr::null();
            (*self.handle).addFunction.unwrap()(f, c_string.as_ptr(), &mut err);
            if !err.is_null() {
                let c_str = ::core::ffi::CStr::from_ptr(err);
                let err_str = c_str.to_str().unwrap();
                Err(Error::Lua(err_str.to_owned()))
            } else {
                Ok(())
            }
        }
    }

    // pub addFunction: ::core::option::Option<
    //     unsafe extern "C" fn(
    //         f: lua_CFunction,
    //         name: *const ::core::ffi::c_char,
    //         outErr: *mut *const ::core::ffi::c_char,
    //     ) -> ::core::ffi::c_int,
    // >,
    // pub registerClass: ::core::option::Option<
    //     unsafe extern "C" fn(
    //         name: *const ::core::ffi::c_char,
    //         reg: *const lua_reg,
    //         vals: *const lua_val,
    //         isstatic: ::core::ffi::c_int,
    //         outErr: *mut *const ::core::ffi::c_char,
    //     ) -> ::core::ffi::c_int,
    // >,
    // pub pushFunction: ::core::option::Option<unsafe extern "C" fn(f: lua_CFunction)>,
    // pub indexMetatable: ::core::option::Option<unsafe extern "C" fn() -> ::core::ffi::c_int>,
    // pub stop: ::core::option::Option<unsafe extern "C" fn()>,
    // pub start: ::core::option::Option<unsafe extern "C" fn()>,
    // pub getArgCount: ::core::option::Option<unsafe extern "C" fn() -> ::core::ffi::c_int>,
    // pub getArgType: ::core::option::Option<
    //     unsafe extern "C" fn(
    //         pos: ::core::ffi::c_int,
    //         outClass: *mut *const ::core::ffi::c_char,
    //     ) -> LuaType,
    // >,
    // pub argIsNil:
    //     ::core::option::Option<unsafe extern "C" fn(pos: ::core::ffi::c_int) -> ::core::ffi::c_int>,
    // pub getArgBool:
    //     ::core::option::Option<unsafe extern "C" fn(pos: ::core::ffi::c_int) -> ::core::ffi::c_int>,
    // pub getArgInt:
    //     ::core::option::Option<unsafe extern "C" fn(pos: ::core::ffi::c_int) -> ::core::ffi::c_int>,
    // pub getArgFloat: ::core::option::Option<unsafe extern "C" fn(pos: ::core::ffi::c_int) -> f32>,
    // pub getArgString: ::core::option::Option<
    //     unsafe extern "C" fn(pos: ::core::ffi::c_int) -> *const ::core::ffi::c_char,
    // >,
    // pub getArgBytes: ::core::option::Option<
    //     unsafe extern "C" fn(
    //         pos: ::core::ffi::c_int,
    //         outlen: *mut usize,
    //     ) -> *const ::core::ffi::c_char,
    // >,
    // pub getArgObject: ::core::option::Option<
    //     unsafe extern "C" fn(
    //         pos: ::core::ffi::c_int,
    //         type_: *mut ::core::ffi::c_char,
    //         outud: *mut *mut LuaUDObject,
    //     ) -> *mut ::core::ffi::c_void,
    // >,
    // pub getBitmap:
    //     ::core::option::Option<unsafe extern "C" fn(pos: ::core::ffi::c_int) -> *mut LCDBitmap>,
    // pub getSprite:
    //     ::core::option::Option<unsafe extern "C" fn(pos: ::core::ffi::c_int) -> *mut LCDSprite>,
    // pub pushNil: ::core::option::Option<unsafe extern "C" fn()>,
    // pub pushBool: ::core::option::Option<unsafe extern "C" fn(val: ::core::ffi::c_int)>,
    // pub pushInt: ::core::option::Option<unsafe extern "C" fn(val: ::core::ffi::c_int)>,
    // pub pushFloat: ::core::option::Option<unsafe extern "C" fn(val: f32)>,
    // pub pushString: ::core::option::Option<unsafe extern "C" fn(str_: *const ::core::ffi::c_char)>,
    // pub pushBytes:
    //     ::core::option::Option<unsafe extern "C" fn(str_: *const ::core::ffi::c_char, len: usize)>,
    // pub pushBitmap: ::core::option::Option<unsafe extern "C" fn(bitmap: *mut LCDBitmap)>,
    // pub pushSprite: ::core::option::Option<unsafe extern "C" fn(sprite: *mut LCDSprite)>,
    // pub pushObject: ::core::option::Option<
    //     unsafe extern "C" fn(
    //         obj: *mut ::core::ffi::c_void,
    //         type_: *mut ::core::ffi::c_char,
    //         nValues: ::core::ffi::c_int,
    //     ) -> *mut LuaUDObject,
    // >,
    // pub retainObject:
    //     ::core::option::Option<unsafe extern "C" fn(obj: *mut LuaUDObject) -> *mut LuaUDObject>,
    // pub releaseObject: ::core::option::Option<unsafe extern "C" fn(obj: *mut LuaUDObject)>,
    // pub setUserValue: ::core::option::Option<
    //     unsafe extern "C" fn(obj: *mut LuaUDObject, slot: ::core::ffi::c_uint),
    // >,
    // pub getUserValue: ::core::option::Option<
    //     unsafe extern "C" fn(
    //         obj: *mut LuaUDObject,
    //         slot: ::core::ffi::c_uint,
    //     ) -> ::core::ffi::c_int,
    // >,
    // pub callFunction_deprecated: ::core::option::Option<
    //     unsafe extern "C" fn(name: *const ::core::ffi::c_char, nargs: ::core::ffi::c_int),
    // >,
    // pub callFunction: ::core::option::Option<
    //     unsafe extern "C" fn(
    //         name: *const ::core::ffi::c_char,
    //         nargs: ::core::ffi::c_int,
    //         outerr: *mut *const ::core::ffi::c_char,
    //     ) -> ::core::ffi::c_int,
    // >,
}
