pub use std::io::Error;
use std::{
    ffi::{CStr, CString},
    io::ErrorKind,
    os::raw::c_char,
    time::Duration,
};

#[repr(transparent)]
#[derive(Clone, Copy, Debug)]
struct KeySerial(i32);

const USER: *const c_char = unsafe { CStr::from_bytes_with_nul_unchecked(b"user\0").as_ptr() };
const KEY_SPEC_USER_SESSION_KEYRING: KeySerial = KeySerial(-5);
const KEY_SPEC_PROCESS_KEYRING: KeySerial = KeySerial(-2);

#[link(name = "keyutils")]
extern "C" {
    fn add_key(
        _type: *const c_char,
        description: *const c_char,
        payload: *const u8,
        plen: usize,
        ringid: KeySerial,
    ) -> KeySerial;
    fn keyctl_search(
        ringid: KeySerial,
        _type: *const c_char,
        description: *const c_char,
        destringid: KeySerial,
    ) -> i64;
    fn keyctl_read(id: KeySerial, buffer: *mut c_char, buflen: usize) -> i64;
    fn keyctl_set_timeout(key: KeySerial, timeout: u32) -> i64;
}

#[fehler::throws]
fn find_key(description: &str) -> KeySerial {
    let description = CString::new(description)?;
    let res = unsafe {
        keyctl_search(
            KEY_SPEC_USER_SESSION_KEYRING,
            USER,
            description.as_ptr(),
            KEY_SPEC_PROCESS_KEYRING,
        )
    };
    if res == -1 {
        fehler::throw!(Error::last_os_error());
    } else {
        KeySerial(res as i32)
    }
}

#[fehler::throws]
pub fn set_timeout(description: &str, timeout: Duration) {
    let key = find_key(description)?;
    let res = unsafe { keyctl_set_timeout(key, timeout.as_secs() as u32) };
    if res == -1 {
        fehler::throw!(Error::last_os_error());
    }
}

#[fehler::throws]
pub fn read_key(description: &str) -> Vec<u8> {
    let key = find_key(description)?;
    let res = unsafe { keyctl_read(key, &mut 0, 0) };
    if res == -1 {
        fehler::throw!(Error::last_os_error());
    }
    let size = res as usize;
    let mut buffer = vec![0; size];
    let res = unsafe { keyctl_read(key, buffer.as_mut_ptr() as *mut c_char, buffer.len()) };
    if res == -1 {
        fehler::throw!(Error::last_os_error());
    }
    if res as usize != buffer.len() {
        fehler::throw!(Error::new(ErrorKind::Other, "failed to read key"));
    }
    buffer
}

#[fehler::throws]
pub fn store_key(description: &str, data: &[u8]) {
    let description = CString::new(description)?;
    let res = unsafe {
        add_key(
            USER,
            description.as_ptr(),
            data.as_ptr(),
            data.len(),
            KEY_SPEC_USER_SESSION_KEYRING,
        )
    };
    if let KeySerial(-1) = res {
        fehler::throw!(Error::last_os_error());
    }
}
