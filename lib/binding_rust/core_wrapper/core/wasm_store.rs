use std::os;

use crate::core::{util::*, *};
pub type __uint8_t = libc::c_uchar;
pub type __int16_t = libc::c_short;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type int16_t = __int16_t;
pub type int32_t = __int32_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type TSStateId = uint16_t;
pub type TSSymbol = uint16_t;
pub type TSFieldId = uint16_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub states: *const bool,
    pub symbol_map: *const TSSymbol,
    pub create: Option<unsafe extern "C" fn() -> *mut libc::c_void>,
    pub destroy: Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub scan: Option<unsafe extern "C" fn(*mut libc::c_void, *mut TSLexer, *const bool) -> bool>,
    pub serialize:
        Option<unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_char) -> libc::c_uint>,
    pub deserialize:
        Option<unsafe extern "C" fn(*mut libc::c_void, *const libc::c_char, libc::c_uint) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub count: uint8_t,
    pub reusable: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub type_: uint8_t,
    pub child_count: uint8_t,
    pub symbol: TSSymbol,
    pub dynamic_precedence: int16_t,
    pub production_id: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub type_: uint8_t,
    pub state: TSStateId,
    pub extra: bool,
    pub repetition: bool,
}
#[no_mangle]
pub unsafe extern "C" fn ts_wasm_store_delete(mut self_0: *mut TSWasmStore) {}
#[no_mangle]
pub unsafe extern "C" fn ts_wasm_store_start(
    mut self_0: *mut TSWasmStore,
    mut lexer: *mut TSLexer,
    mut language: *const TSLanguage,
) -> bool {
    return 0 as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn ts_wasm_store_reset(mut self_0: *mut TSWasmStore) {}
#[no_mangle]
pub unsafe extern "C" fn ts_wasm_store_call_lex_main(
    mut self_0: *mut TSWasmStore,
    mut state: TSStateId,
) -> bool {
    return 0 as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn ts_wasm_store_call_lex_keyword(
    mut self_0: *mut TSWasmStore,
    mut state: TSStateId,
) -> bool {
    return 0 as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn ts_wasm_store_call_scanner_create(
    mut self_0: *mut TSWasmStore,
) -> uint32_t {
    return 0 as libc::c_int as uint32_t;
}
#[no_mangle]
pub unsafe extern "C" fn ts_wasm_store_call_scanner_destroy(
    mut self_0: *mut TSWasmStore,
    mut scanner_address: uint32_t,
) {
}
#[no_mangle]
pub unsafe extern "C" fn ts_wasm_store_call_scanner_scan(
    mut self_0: *mut TSWasmStore,
    mut scanner_address: uint32_t,
    mut valid_tokens_ix: uint32_t,
) -> bool {
    return 0 as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn ts_wasm_store_call_scanner_serialize(
    mut self_0: *mut TSWasmStore,
    mut scanner_address: uint32_t,
    mut buffer: *mut libc::c_char,
) -> uint32_t {
    return 0 as libc::c_int as uint32_t;
}
#[no_mangle]
pub unsafe extern "C" fn ts_wasm_store_call_scanner_deserialize(
    mut self_0: *mut TSWasmStore,
    mut scanner_address: uint32_t,
    mut buffer: *const libc::c_char,
    mut length: libc::c_uint,
) {
}
#[no_mangle]
pub unsafe extern "C" fn ts_wasm_store_has_error(mut self_0: *const TSWasmStore) -> bool {
    return 0 as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn ts_language_is_wasm(mut self_0: *const TSLanguage) -> bool {
    return 0 as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn ts_wasm_language_retain(mut self_0: *const TSLanguage) {}
#[no_mangle]
pub unsafe extern "C" fn ts_wasm_language_release(mut self_0: *const TSLanguage) {}
