pub enum Error {
    Write(std::io::Error),
    Read(std::io::Error),
    InvalidMagic,
    InvalidHeader,
    UnexpectedEof,
    
}