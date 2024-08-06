mod chunk;
mod compression;
mod error;
mod region;

pub use chunk::RawChunk;
pub use compression::CompressionType;
pub use error::McaError;
pub use region::{Region, RegionIter};

#[cfg(test)]
mod tests {
    use super::*;

    const REGION: &[u8] = include_bytes!("../benches/r.0.0.mca");

    #[test]
    fn new_region() {
        let region = Region::new(REGION).unwrap();

        assert_eq!(region.inner().len(), REGION.len());
    }

    #[test]
    fn chunk_parse() {
        let region = Region::new(REGION).unwrap();
        let chunk = region.get_chunk(0, 0).unwrap().unwrap();

        assert_eq!(chunk.get_compression_type(), CompressionType::Zlib);
        assert!(chunk.raw_data.len() >= 4096);
    }

    #[test]
    fn entire_region() {
        let region = Region::new(REGION).unwrap();

        for chunk in region.iter() {
            let _ = chunk.unwrap();
        }
    }

    #[test]
    fn get_location() {
        let region = Region::new(REGION).unwrap();
        let location = region.get_location(Region::chunk_offset(0, 0)).unwrap();

        assert_eq!(location, [0, 3, 22, 2]);
    }

    #[test]
    fn get_timestamp() {
        let region = Region::new(REGION).unwrap();
        let timestamp = region.get_timestamp(Region::chunk_offset(0, 0));

        assert_eq!(timestamp, [102, 128, 130, 115]);
    }

    #[test]
    fn no_chunk() {
        let mut bytes = vec![0, 0, 2, 1]; // offset 2 * SECTOR_SIZE
        bytes.extend_from_slice(&[0; 8188]);

        let region = Region::new(&bytes).unwrap();

        let chunk = region.get_chunk(0, 0);

        if let Err(McaError::InvalidChunkPayload(_)) = chunk {
            assert!(true)
        } else {
            assert!(false)
        }
    }
}
