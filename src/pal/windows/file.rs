use std::os::fd::RawFd;

pub fn open(path: &str) {
    println!("open file");
}

pub fn write(fd: RawFd, buffer: &[u8]) {
    println!("write file");
}

pub fn read(fd: RawFd, buffer: &mut [u8]) {
    println!("read file");
}

pub fn close(fd: RawFd) {
    println!("close file");
}
