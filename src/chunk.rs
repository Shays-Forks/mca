use std::io::Read;

use crate::{compression::CompressionType, McaError};

/// A raw compressed chunk, holds the compression type used.  
/// And the specific chunk byte slice from the region data
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct RawChunk<'a> {
    pub raw_data: &'a [u8],
    compression_type: CompressionType,
}

impl<'a> RawChunk<'a> {
    /// Decompresses the raw chunk data depending on it's compression type
    ///
    /// ## Example
    /// ```ignore
    /// // ...
    ///
    /// let chunk = region.get_chunk(0, 0)?.unwrap();
    ///
    /// let data = chunk.decompress()?;
    /// ```
    pub fn decompress(&self) -> Result<Vec<u8>, McaError> {
        match self.compression_type {
            CompressionType::Zlib => Ok(miniz_oxide::inflate::decompress_to_vec_zlib(
                &self.raw_data,
            )?),
            CompressionType::Uncompressed => Ok(self.raw_data.to_vec()),
            CompressionType::LZ4 => Ok({
                let mut buf: Vec<u8> = Vec::new();
                lz4_java_wrc::Lz4BlockInput::new(&self.raw_data[..]).read_to_end(&mut buf).unwrap();
                buf
            }),
            CompressionType::GZip => unimplemented!("This is unused in practice and if you somehow need this, make an issue on github and i'll add it <3"),
            CompressionType::Custom => unimplemented!("Haven't implemented this and i don't personally need this but make an issue on github and i'll fix it <3")
        }
    }

    /// Get the chunks [`CompressionType`]
    pub fn get_compression_type(&self) -> CompressionType {
        self.compression_type.clone()
    }

    /// Creates a new raw chunk from it's bytes and compression type
    pub fn new(data: &'a [u8], compression: CompressionType) -> RawChunk {
        RawChunk {
            raw_data: data,
            compression_type: compression,
        }
    }
}
