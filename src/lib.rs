mod chunk;
mod compression;
mod error;
mod reader;
mod writer;

pub use chunk::{PendingChunk, RawChunk};
pub use compression::CompressionType;
pub use error::McaError;
pub use reader::{RegionIter, RegionReader};
pub use writer::RegionWriter;

const SECTOR_SIZE: usize = 4096;

#[cfg(test)]
mod tests {
    use super::*;

    const REGION: &[u8] = include_bytes!("../benches/r.0.0.mca");

    #[test]
    fn new_region() {
        let region = RegionReader::new(REGION).unwrap();

        assert_eq!(region.inner().len(), REGION.len());
    }

    #[test]
    fn chunk_parse() {
        let region = RegionReader::new(REGION).unwrap();
        let chunk = region.get_chunk(0, 0).unwrap().unwrap();

        assert_eq!(chunk.get_compression_type(), CompressionType::Zlib);
        assert!(chunk.raw_data.len() >= 4096);
    }

    #[test]
    fn entire_region() {
        let region = RegionReader::new(REGION).unwrap();

        for chunk in region.iter() {
            let _ = chunk.unwrap();
        }
    }

    #[test]
    fn parse_nbt() {
        let region = RegionReader::new(REGION).unwrap();
        let chunk = region.get_chunk(18, 17).unwrap().unwrap();

        let data = chunk.decompress().unwrap();
        let _ = sculk::chunk::Chunk::from_bytes(&data).unwrap();
    }

    #[test]
    fn decompress() {
        let region = RegionReader::new(REGION).unwrap();
        let chunk = region.get_chunk(18, 17).unwrap().unwrap();

        let _ = chunk.decompress().unwrap();
    }

    #[test]
    fn get_location() {
        let region = RegionReader::new(REGION).unwrap();
        let location = region
            .get_location(RegionReader::chunk_offset(0, 0))
            .unwrap();

        assert_eq!(location, [0, 3, 22, 2]);
    }

    #[test]
    fn get_timestamp() {
        let region = RegionReader::new(REGION).unwrap();
        #[cfg(feature = "unsafe")]
        let timestamp = region.get_timestamp(RegionReader::chunk_offset(0, 0));

        #[cfg(not(feature = "unsafe"))]
        let timestamp = region
            .get_timestamp(RegionReader::chunk_offset(0, 0))
            .unwrap();

        assert_eq!(timestamp, [102, 128, 130, 115]);
    }

    #[test]
    fn no_chunk() {
        let mut bytes = vec![0, 0, 2, 1]; // offset 2 * SECTOR_SIZE
        bytes.extend_from_slice(&[0; 8188]);

        let region = RegionReader::new(&bytes).unwrap();

        let chunk = region.get_chunk(0, 0);

        if let Err(McaError::InvalidChunkPayload(_)) = chunk {
            assert!(true)
        } else {
            assert!(false)
        }
    }
}
