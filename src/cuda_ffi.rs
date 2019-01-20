use libc::c_char;
use std::ffi::{ CStr, CString };


#[link(name="cuda_crypto", kind="static")]
extern {
    pub fn MD5(a: *const c_char) -> *const c_char;
    pub fn SHA256(a: *const c_char) -> *const c_char;
    pub fn SHA384(a: *const c_char) -> *const c_char;
    pub fn SHA512(a: *const c_char) -> *const c_char;
}


pub fn cuda_md5<T: AsRef<str>>(t: T) -> String {
    let c_str = CString::new(t.as_ref()).unwrap();
    let r_ptr = unsafe { MD5(c_str.as_ptr()) }; // c_char
    let r_str: &CStr = unsafe { CStr::from_ptr(r_ptr) }; // &CStr
    let str_slice = r_str.to_str().unwrap(); // &str
    str_slice.to_string() // String
}


pub fn cuda_sha256<T: AsRef<str>>(t: T) -> String {
    let c_str = CString::new(t.as_ref()).unwrap();
    let r_ptr = unsafe { SHA256(c_str.as_ptr()) }; // c_char
    let r_str: &CStr = unsafe { CStr::from_ptr(r_ptr) }; // &CStr
    let str_slice = r_str.to_str().unwrap(); // &str
    str_slice.to_string() // String
}


pub fn cuda_sha384<T: AsRef<str>>(t: T) -> String {
    let c_str = CString::new(t.as_ref()).unwrap();
    let r_ptr = unsafe { SHA384(c_str.as_ptr()) }; // c_char
    let r_str: &CStr = unsafe { CStr::from_ptr(r_ptr) }; // &CStr
    let str_slice = r_str.to_str().unwrap(); // &str
    str_slice.to_string() // String
}


pub fn cuda_sha512<T: AsRef<str>>(t: T) -> String {
    let c_str = CString::new(t.as_ref()).unwrap();
    let r_ptr = unsafe { SHA512(c_str.as_ptr()) }; // c_char
    let r_str: &CStr = unsafe { CStr::from_ptr(r_ptr) }; // &CStr
    let str_slice = r_str.to_str().unwrap(); // &str
    str_slice.to_string() // String
}