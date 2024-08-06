use miniz_oxide::inflate;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum McaError {
    #[error("Chunk hasn't been generated yet")]
    NotGenerated,

    #[error("No region header")]
    MissingHeader,

    #[error("Invalid chunk: {0}")]
    InvalidChunkPayload(String),

    #[cfg(not(feature = "unsafe"))]
    #[error("Out of bounds byte access")]
    OutOfBoundsByte,

    #[error("Io failed: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Zlib Decompression failed: {0}")]
    ZLib(#[from] inflate::DecompressError),

    #[error("LZ4 Decompression failed: {0}")]
    Lz4Error(#[from] lz4_flex::block::DecompressError),
}
