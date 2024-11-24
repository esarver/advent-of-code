use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("send channel error: {0}")]
    SendChannelError(String),

    #[error("part error ({year},{day},{part}): {desc}")]
    PartError {
        year: u16,
        day: u8,
        part: u8,
        desc: String,
    },
}
