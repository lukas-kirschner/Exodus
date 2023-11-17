use crate::world::io_error::GameWorldParseError;
use crate::world::GameWorld;
use bytebuffer::ByteBuffer;
use sha2::{Digest, Sha256};

#[derive(Debug)]
pub enum RecomputeHashResult {
    /// The hash remained the same after re-computation
    SAME,
    /// The hash changed
    CHANGED { old_hash: [u8; 32] },
    /// There was an error computing the hash
    ERROR { error: GameWorldParseError },
}

impl GameWorld {
    pub fn recompute_hash(&mut self) -> RecomputeHashResult {
        let old_hash = self.hash;
        let mut buf = ByteBuffer::new();
        match self.serialize_world_content(&mut buf) {
            Ok(()) => {},
            Err(error) => return RecomputeHashResult::ERROR { error },
        };
        let mut hasher = Sha256::new();
        hasher.update(buf.as_bytes());
        let new_hash: [u8; 32] = hasher.finalize().into();
        self.hash = new_hash;
        let differing = self
            .hash
            .iter()
            .zip(old_hash.iter())
            .filter(|&(a, b)| a == b)
            .count();
        if differing == 32 {
            RecomputeHashResult::SAME
        } else {
            RecomputeHashResult::CHANGED { old_hash }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::tiles::Tile;
    use crate::tilesets::Tileset::Classic;
    use crate::world::hash::RecomputeHashResult;
    use crate::world::GameWorld;

    fn assert_hashes_are_equal(hash1: &[u8; 32], hash2: &[u8; 32]) {
        assert_eq!(
            32,
            hash1
                .iter()
                .zip(hash2.iter())
                .filter(|&(a, b)| a == b)
                .count()
        )
    }

    fn assert_hashes_are_not_equal(hash1: &[u8; 32], hash2: &[u8; 32]) {
        assert_ne!(
            32,
            hash1
                .iter()
                .zip(hash2.iter())
                .filter(|&(a, b)| a == b)
                .count()
        )
    }

    #[test]
    fn test_map_hash_simple() {
        let mut map = GameWorld::exampleworld();
        assert_hashes_are_equal(&[0u8; 32], &map.hash);
        match map.recompute_hash() {
            RecomputeHashResult::CHANGED { .. } => {},
            x => {
                panic!("Result was {:?} - Hash was {:?}", x, map.hash)
            },
        };
        let new_hash = map.hash.clone();
        match map.recompute_hash() {
            RecomputeHashResult::SAME => {},
            x => {
                panic!("Result was {:?} - Hash was {:?}", x, map.hash)
            },
        };
        assert_hashes_are_equal(&map.hash, &new_hash);
    }
    #[test]
    fn test_map_hash_stays_same_for_different_tileset() {
        let mut map = GameWorld::exampleworld();
        let mut same_map = GameWorld::exampleworld();
        map.recompute_hash();
        same_map.recompute_hash();
        assert_hashes_are_equal(&map.hash, &same_map.hash);
        same_map.forced_tileset = Some(Classic);
        assert_ne!(map.forced_tileset, same_map.forced_tileset);
        assert!(matches!(
            same_map.recompute_hash(),
            RecomputeHashResult::SAME
        ));
        assert_hashes_are_equal(&map.hash, &same_map.hash);
    }

    #[test]
    fn test_map_hash_changing() {
        let mut map = GameWorld::exampleworld();
        assert_ne!(&Tile::DOOR, map.get(0, 0).unwrap());
        let old_tile = map.get(0, 0).unwrap().clone();
        match map.recompute_hash() {
            RecomputeHashResult::CHANGED { .. } => {},
            x => {
                panic!("Result was {:?} - Hash was {:?}", x, map.hash)
            },
        };
        let old_hash = map.hash.clone();
        map.set(0, 0, Tile::DOOR);
        match map.recompute_hash() {
            RecomputeHashResult::CHANGED { .. } => {},
            x => {
                panic!("Result was {:?} - Hash was {:?}", x, map.hash)
            },
        };
        assert_hashes_are_not_equal(&map.hash, &old_hash);
        let new_hash = map.hash.clone();
        map.set(0, 0, old_tile);
        match map.recompute_hash() {
            RecomputeHashResult::CHANGED { .. } => {},
            x => {
                panic!("Result was {:?} - Hash was {:?}", x, map.hash)
            },
        };
        assert_hashes_are_equal(&old_hash, &map.hash);
        assert_hashes_are_not_equal(&old_hash, &new_hash);
    }
}
