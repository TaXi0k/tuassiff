/// Represents all possible errors that can occur when opening,
/// saving, or processing `.tuassiff` files.
#[derive(Debug)]
pub enum Error {
    /// Failed to write data to the file.
    Write(std::io::Error),
    /// Failed to read the file from disk.
    Read(std::io::Error),
    /// Failed to create a new file on disk.
    Create(std::io::Error),
    /// The file doesn't start with the required magic bytes [`crate::MAGIC_BYTES`].
    InvalidMagic,
    /// Failed to parse the width or height header as `u32` (likely file corruption).
    InvalidHeader,
    /// The file ended abruptly before whole header could be read.
    UnexpectedEof,
    /// Count of pixels provided doesn't match expected from width and height.
    InvalidPixels,
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Write(e) => write!(f, "Failed to write to file: {e}"),
            Error::Read(e) => write!(f, "Failed to read file: {e}"),
            Error::Create(e) => write!(f, "Failed to create file: {e}"),
            Error::InvalidMagic => write!(f, "Invalid magic bytes (file is not a valid tuassiff format)."),
            Error::InvalidHeader => write!(f, "Invalid header format (could not parse width or height)."),
            Error::UnexpectedEof => write!(f, "Unexpected end of file (file is truncated or missing data)."),
            Error::InvalidPixels => write!(f, "Count of pixels provided doesn't match expected."),
        }
    }
}
impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Write(e) | Error::Read(e) | Error::Create(e) => Some(e),
            _ => None,
        }
    }
}