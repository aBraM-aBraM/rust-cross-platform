mod pal;
mod fs;

use pal::file;

const FILE_PATH: &str = "myfile.txt\0";
const DATA_TO_WRITE: &[u8; 13] = b"Hello, World!";

fn test_pal_file()
{
    let fd = match file::open(FILE_PATH, "w+") {
        Ok(fd) => fd,
        Err(err) => {
            panic!("Failed to open file: {}", err);
        }
    };

    match file::write(fd, DATA_TO_WRITE) {
        Ok(_) => (),
        Err(err) => {
            panic!("Failed to write data: {}", err);
        }
    }

    match file::close(fd) {
        Ok(_) => (),
        Err(err) => {
            panic!("Failed to close file: {}", err);
        }
    }

    let fd = match file::open(FILE_PATH, "r") {
        Ok(fd) => fd,
        Err(err) => {
            panic!("Failed to open file: {}", err);
        }
    };

    let mut buffer = vec![0u8; DATA_TO_WRITE.len()];
    match file::read(fd, &mut buffer) {
        Ok(_) => {
            assert_eq!(DATA_TO_WRITE, buffer.as_slice());
        }
        Err(err) => {
            panic!("Failed to read data: {}", err);
        }
    }
}

fn test_file_obj()
{
    {
        let file = fs::File::create(FILE_PATH, "w+").unwrap();
        file.write(DATA_TO_WRITE).unwrap();
    }
    let mut buffer = vec![0u8; DATA_TO_WRITE.len()];
    {
        let file = fs::File::create(FILE_PATH, "r").unwrap();
        match file.read(&mut buffer) {
            Ok(_) => {
                assert_eq!(DATA_TO_WRITE, buffer.as_slice());
            }
            Err(err) => {
                panic!("Failed to read data: {}", err);
            }
        }
    }
}

fn main() {
    test_pal_file();
    test_file_obj();
}
