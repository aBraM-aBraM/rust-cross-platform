use crate::pal;

pub struct File {
    fd: pal::file_types::FileDescriptor,
}

impl File {
    pub fn create(filename: &str, mode: &str) -> Result<Self, i32> {
        match pal::file::open(filename, mode) {
            Ok(fd) => Ok(Self { fd }),
            Err(native_err) => Err(native_err),
        }
    }

    pub fn write(&self, buffer: &[u8]) -> Result<usize, i32> {
        match pal::file::write(self.fd, buffer) {
            Ok(write_count) => Ok(write_count),
            Err(native_err) => Err(native_err),
        }
    }

    pub fn read(&self, buffer: &mut [u8]) -> Result<usize, i32> {
        match pal::file::read(self.fd, buffer) {
            Ok(read_count) => Ok(read_count),
            Err(native_err) => Err(native_err),
        }
    }
}

impl Drop for File {
    fn drop(&mut self) {
        let _ = pal::file::close(self.fd);
    }
}