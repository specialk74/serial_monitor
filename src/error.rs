#[derive(Debug)]
pub enum ProgramError {
    NoPortFound,
    UnableToOpen(String, tokio_serial::Error),
    IoError(std::io::Error),
    SerialPortError(tokio_serial::Error),
}
