use crate::{compression::CompressionType, McaError};

/// A raw compressed chunk, holds the compression type used.  
/// And the specific chunk byte slice from the region data
///
/// This is used when getting chunk data **from** a region file.  
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct RawChunk<'a> {
    pub raw_data: &'a [u8],
    compression_type: CompressionType,
}

impl RawChunk<'_> {
    /// Decompresses the raw chunk data depending on its compression type
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
        self.compression_type.decompress(self.raw_data)
    }

    /// Get the chunks [`CompressionType`]
    pub fn get_compression_type(&self) -> CompressionType {
        self.compression_type.clone()
    }

    /// Creates a new raw chunk from its bytes and compression type
    pub fn new(data: &[u8], compression: CompressionType) -> RawChunk {
        RawChunk {
            raw_data: data,
            compression_type: compression,
        }
    }
}

/// A `pending` chunk, holds all metadata used in region chunk payloads.  
///
/// This is used when **writing** region files.  
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct PendingChunk {
    pub compressed_data: Vec<u8>,
    pub compression: CompressionType,
    pub timestamp: u32,
    pub coordinate: (u8, u8),
}

impl PendingChunk {
    /// Create a new pending chunk
    ///
    /// ## Example
    /// ```ignore
    /// use mca::{PendingChunk, CompressionType};
    ///
    /// let data: &[u8] = // ...
    ///
    /// let chunk = PendingChunk::new(&data, CompressionType::LZ4, 1724372177, (4, 6));
    /// ```
    pub fn new(
        raw_data: &[u8],
        compression: CompressionType,
        timestamp: u32,
        coordinate: (u8, u8),
    ) -> Result<PendingChunk, McaError> {
        assert!(coordinate.0 < 32);
        assert!(coordinate.1 < 32);

        let compressed_data = compression.compress(raw_data)?;

        Ok(PendingChunk {
            compressed_data,
            compression,
            timestamp,
            coordinate,
        })
    }
}
