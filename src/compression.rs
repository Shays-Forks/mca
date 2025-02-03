use std::io::{Read, Write};

use crate::McaError;

/// Compression types used in chunks
///
/// **`GZip` & `Custom` is unsupported currently**
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum CompressionType {
    GZip = 1,
    Zlib = 2,
    Uncompressed = 3,
    LZ4 = 4,
    Custom = 127,
}

impl From<u8> for CompressionType {
    fn from(value: u8) -> Self {
        CompressionType::from_u8(value)
    }
}

impl From<CompressionType> for u8 {
    fn from(value: CompressionType) -> Self {
        value.to_u8()
    }
}

impl CompressionType {
    pub fn from_u8(value: u8) -> CompressionType {
        match value {
            1 => CompressionType::GZip,
            2 => CompressionType::Zlib,
            3 => CompressionType::Uncompressed,
            4 => CompressionType::LZ4,
            127 => CompressionType::Custom,
            _ => panic!("Invalid compression type: {}", value),
        }
    }

    pub fn to_u8(&self) -> u8 {
        match self {
            CompressionType::GZip => 1,
            CompressionType::Zlib => 2,
            CompressionType::Uncompressed => 3,
            CompressionType::LZ4 => 4,
            CompressionType::Custom => 127,
        }
    }

    /// Takes in a byte slice and uses the current compression type to **compress** the data
    pub fn compress(&self, data: &[u8]) -> Result<Vec<u8>, McaError> {
        match self {
            CompressionType::Zlib => Ok(miniz_oxide::deflate::compress_to_vec_zlib(data, 4)),
            CompressionType::Uncompressed => Ok(data.to_vec()),
            CompressionType::LZ4 => Ok({
                let mut buf: Vec<u8> = Vec::new();
                lz4_java_wrc::Lz4BlockOutput::new(&mut buf).write_all(data)?;
                buf
            }),
            CompressionType::GZip => unimplemented!("This is unused in practice and if you somehow need this, make an issue on github and i'll add it <3"),
            CompressionType::Custom => unimplemented!("Haven't implemented this and i don't personally need this but make an issue on github and i'll fix it <3"),
        }
    }

    /// Takes in a byte slice and uses the current compression type to **decompress** the data
    pub fn decompress(&self, data: &[u8]) -> Result<Vec<u8>, McaError> {
        match self {
            CompressionType::Zlib => Ok(miniz_oxide::inflate::decompress_to_vec_zlib(
                data,
            )?),
            CompressionType::Uncompressed => Ok(data.to_vec()),
            CompressionType::LZ4 => Ok({
                let mut buf: Vec<u8> = Vec::new();
                lz4_java_wrc::Lz4BlockInput::new(data).read_to_end(&mut buf)?;
                buf
            }),
            CompressionType::GZip => unimplemented!("This is unused in practice and if you somehow need this, make an issue on github and i'll add it <3"),
            CompressionType::Custom => unimplemented!("Haven't implemented this and i don't personally need this but make an issue on github and i'll fix it <3")
        }
    }
}
