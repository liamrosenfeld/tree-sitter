use std::os;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn abort() -> !;
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
unsafe extern "C" fn ts_malloc_default(mut size: size_t) -> *mut libc::c_void {
    let mut result: *mut libc::c_void = malloc(size);
    if size > 0 as libc::c_int as size_t && result.is_null() {
        fprintf(
            stderr,
            b"tree-sitter failed to allocate %zu bytes\0" as *const u8
                as *const libc::c_char,
            size,
        );
        abort();
    }
    return result;
}
unsafe extern "C" fn ts_calloc_default(
    mut count: size_t,
    mut size: size_t,
) -> *mut libc::c_void {
    let mut result: *mut libc::c_void = calloc(count, size);
    if count > 0 as libc::c_int as size_t && result.is_null() {
        fprintf(
            stderr,
            b"tree-sitter failed to allocate %zu bytes\0" as *const u8
                as *const libc::c_char,
            count * size,
        );
        abort();
    }
    return result;
}
unsafe extern "C" fn ts_realloc_default(
    mut buffer: *mut libc::c_void,
    mut size: size_t,
) -> *mut libc::c_void {
    let mut result: *mut libc::c_void = realloc(buffer, size);
    if size > 0 as libc::c_int as size_t && result.is_null() {
        fprintf(
            stderr,
            b"tree-sitter failed to reallocate %zu bytes\0" as *const u8
                as *const libc::c_char,
            size,
        );
        abort();
    }
    return result;
}
#[no_mangle]
pub static mut ts_current_malloc: Option::<
    unsafe extern "C" fn(size_t) -> *mut libc::c_void,
> = unsafe {
    Some(ts_malloc_default as unsafe extern "C" fn(size_t) -> *mut libc::c_void)
};
#[no_mangle]
pub static mut ts_current_calloc: Option::<
    unsafe extern "C" fn(size_t, size_t) -> *mut libc::c_void,
> = unsafe {
    Some(ts_calloc_default as unsafe extern "C" fn(size_t, size_t) -> *mut libc::c_void)
};
#[no_mangle]
pub static mut ts_current_realloc: Option::<
    unsafe extern "C" fn(*mut libc::c_void, size_t) -> *mut libc::c_void,
> = unsafe {
    Some(
        ts_realloc_default
            as unsafe extern "C" fn(*mut libc::c_void, size_t) -> *mut libc::c_void,
    )
};
#[no_mangle]
pub static mut ts_current_free: Option::<
    unsafe extern "C" fn(*mut libc::c_void) -> (),
> = unsafe { Some(free as unsafe extern "C" fn(*mut libc::c_void) -> ()) };
#[no_mangle]
pub unsafe extern "C" fn ts_set_allocator(
    mut new_malloc: Option::<unsafe extern "C" fn(size_t) -> *mut libc::c_void>,
    mut new_calloc: Option::<unsafe extern "C" fn(size_t, size_t) -> *mut libc::c_void>,
    mut new_realloc: Option::<
        unsafe extern "C" fn(*mut libc::c_void, size_t) -> *mut libc::c_void,
    >,
    mut new_free: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
) {
    ts_current_malloc = if new_malloc.is_some() {
        new_malloc
    } else {
        Some(ts_malloc_default as unsafe extern "C" fn(size_t) -> *mut libc::c_void)
    };
    ts_current_calloc = if new_calloc.is_some() {
        new_calloc
    } else {
        Some(
            ts_calloc_default
                as unsafe extern "C" fn(size_t, size_t) -> *mut libc::c_void,
        )
    };
    ts_current_realloc = if new_realloc.is_some() {
        new_realloc
    } else {
        Some(
            ts_realloc_default
                as unsafe extern "C" fn(*mut libc::c_void, size_t) -> *mut libc::c_void,
        )
    };
    ts_current_free = if new_free.is_some() {
        new_free
    } else {
        Some(free as unsafe extern "C" fn(*mut libc::c_void) -> ())
    };
}
