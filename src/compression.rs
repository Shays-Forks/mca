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
        match value {
            1 => CompressionType::GZip,
            2 => CompressionType::Zlib,
            3 => CompressionType::Uncompressed,
            4 => CompressionType::LZ4,
            127 => CompressionType::Custom,
            _ => panic!("Invalid compression type"),
        }
    }
}

impl From<CompressionType> for u8 {
    fn from(value: CompressionType) -> Self {
        match value {
            CompressionType::GZip => 1,
            CompressionType::Zlib => 2,
            CompressionType::Uncompressed => 3,
            CompressionType::LZ4 => 4,
            CompressionType::Custom => 127,
        }
    }
}
