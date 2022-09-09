pub mod args;
pub mod occurrence;
pub mod read;
pub mod write;

pub const READ_BUFFER_SIZE: usize = 10 * (1 << 10); // 10MB
pub const NEW_LINE: u8 = b'\n';
