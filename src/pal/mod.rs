#[cfg(feature = "unix")]
pub mod unix {
    pub mod file;
    pub mod file_types;
}

#[cfg(feature = "windows")]
pub mod windows {
    pub mod file;
}

#[cfg(feature = "unix")]
pub use self::unix::*;
#[cfg(feature = "windows")]
pub use self::windows::*;
