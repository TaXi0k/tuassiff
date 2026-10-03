pub enum Error {
    Write(std::io::Error),
    Read(std::io::Error),
    Create(std::io::Error),
    InvalidMagic,
    InvalidHeader,
    UnexpectedEof,
    UnexpectedTrailingData,
}