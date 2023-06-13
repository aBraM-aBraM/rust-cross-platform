use libc::{c_char, c_int, c_void};
use super::file_types::*;

pub fn open(path: &str, mode: &str) -> Result<FileDescriptor, c_int> {
    let path_cstr = path.as_bytes();
    let mode_cstr = mode.as_bytes();

    unsafe {
        let file = libc::fopen(
            path_cstr.as_ptr() as *const c_char,
            mode_cstr.as_ptr() as *const c_char,
        );
        if file.is_null() {
            Err(*libc::__errno_location())
        } else {
            Ok(libc::fileno(file))
        }
    }
}

pub fn write(fd: FileDescriptor, buffer: &[u8]) -> Result<usize, c_int> {
    unsafe {
        let result = libc::write(fd, buffer.as_ptr() as *const c_void, buffer.len());
        if result == -1 {
            return Err(*libc::__errno_location());
        }
        Ok(result as usize)
    }
}

pub fn read(fd: FileDescriptor, buffer: &mut [u8]) -> Result<usize, c_int> {
    unsafe {
        let result = libc::read(fd, buffer.as_mut_ptr() as *mut c_void, buffer.len());
        if result == -1 {
            return Err(*libc::__errno_location());
        }
        Ok(result as usize)
    }
}

pub fn close(fd: FileDescriptor) -> Result<(), String> {
    let result = unsafe { libc::close(fd) };
    if result < 0 {
        Err("Failed to close file".to_string())
    } else {
        Ok(())
    }
}
