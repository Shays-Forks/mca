mod error;

pub use error::McaError;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Region<'a> {
    data: &'a [u8],
}

const SECTOR_SIZE: usize = 4096;

impl<'a> Region<'a> {
    pub fn new(data: &'a [u8]) -> Result<Region<'a>, McaError> {
        if data.len() < (SECTOR_SIZE * 2) {
            return Err(McaError::MissingHeader);
        }

        Ok(Region { data })
    }

    #[inline(always)]
    pub fn chunk_offset(x: i32, z: i32) -> usize {
        4 * ((x & 31) + (z & 31) * 32) as usize
    }

    pub fn get_chunk(&self, x: i32, z: i32) -> Result<RawChunk, McaError> {
        let offset = Region::chunk_offset(x, z);

        let chunk_location = match self.get_location(offset) {
            Some(loc) => loc,
            None => return Err(McaError::NotGenerated),
        };

        // fill the first byte when doing the 3 first chunk location offset
        let endians =
            u32::from_be_bytes([0, chunk_location[0], chunk_location[1], chunk_location[2]])
                as usize;

        let payload_offset: usize = endians * SECTOR_SIZE;

        let byte_length = u32::from_be_bytes(unsafe {
            [
                *self.data.get_unchecked(payload_offset),
                *self.data.get_unchecked(payload_offset + 1),
                *self.data.get_unchecked(payload_offset + 2),
                *self.data.get_unchecked(payload_offset + 3),
            ]
        }) as usize;

        let payload_offset = payload_offset + 4;

        let compression_type =
            CompressionType::from(unsafe { *self.data.get_unchecked(payload_offset) });

        let raw_data = &self.data[payload_offset + 1..=payload_offset + byte_length];

        Ok(RawChunk {
            raw_data,
            compression_type,
        })
    }

    #[inline]
    pub fn get_location(&self, offset: usize) -> Option<[u8; 4]> {
        unsafe {
            let first = *self.data.get_unchecked(offset);
            let last = *self.data.get_unchecked(offset + 3);

            // Empty chunk locations, hasnt been generated if None
            if first == 0 && last == 0 {
                return None;
            }

            let loc = [
                first,
                *self.data.get_unchecked(offset + 1),
                *self.data.get_unchecked(offset + 2),
                last,
            ];

            Some(loc)
        }
    }

    #[inline]
    pub fn get_timestamp(&self, offset: usize) -> [u8; 4] {
        unsafe {
            [
                *self.data.get_unchecked(SECTOR_SIZE + offset),
                *self.data.get_unchecked(SECTOR_SIZE + offset + 1),
                *self.data.get_unchecked(SECTOR_SIZE + offset + 2),
                *self.data.get_unchecked(SECTOR_SIZE + offset + 3),
            ]
        }
    }

    #[inline]
    pub fn get_u32_timestamp(&self, timestamp_bytes: [u8; 4]) -> u32 {
        u32::from_be_bytes(timestamp_bytes)
    }
}

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

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct RawChunk<'a> {
    pub raw_data: &'a [u8],
    compression_type: CompressionType,
}

impl<'a> RawChunk<'a> {
    pub fn decompress(&self) -> Result<Vec<u8>, McaError> {
        match self.compression_type {
            CompressionType::Zlib => Ok(miniz_oxide::inflate::decompress_to_vec_zlib(
                &self.raw_data,
            )?),
            CompressionType::Uncompressed => Ok(self.raw_data.to_vec()),
            CompressionType::LZ4 => Ok(lz4_flex::decompress_size_prepended(self.raw_data)?),
            CompressionType::GZip => unimplemented!("This is unused in practice and if you somehow need this, make an issue on github and i'll add it <3"),
            CompressionType::Custom => unimplemented!("Haven't implemented this and i don't personally need this but make an issue on github and i'll fix it <3")
        }
    }

    pub fn get_compression_type(&self) -> CompressionType {
        self.compression_type.clone()
    }
}
