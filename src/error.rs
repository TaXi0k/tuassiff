#[derive(Debug)]
pub enum Error {
    Write(std::io::Error),
    Read(std::io::Error),
    Create(std::io::Error),
    InvalidMagic,
    InvalidHeader,
    UnexpectedEof,
    UnexpectedTrailingData,
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Write(e) => write!(f, "Failed to write to file: {e}"),
            Error::Read(e) => write!(f, "Failed to read file: {e}"),
            Error::Create(e) => write!(f, "Failed to create file: {e}"),
            Error::InvalidMagic => write!(f, "Invalid magic bytes (file is not a valid tuassiff format)"),
            Error::InvalidHeader => write!(f, "Invalid header format (could not parse width or height)"),
            Error::UnexpectedEof => write!(f, "Unexpected end of file (file is truncated or missing data)"),
            Error::UnexpectedTrailingData => write!(f, "Unexpected trailing data found at the end of the file"),
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