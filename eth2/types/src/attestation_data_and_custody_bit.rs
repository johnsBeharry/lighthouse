use super::AttestationData;
use crate::test_utils::TestRandom;
use rand::RngCore;
use serde_derive::Serialize;
use ssz::TreeHash;
use ssz_derive::{Decode, Encode};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Encode, Decode)]
pub struct AttestationDataAndCustodyBit {
    pub data: AttestationData,
    pub custody_bit: bool,
}

impl TreeHash for AttestationDataAndCustodyBit {
    fn hash_tree_root_internal(&self) -> Vec<u8> {
        let mut result: Vec<u8> = vec![];
        result.append(&mut self.data.hash_tree_root());
        result.append(&mut self.custody_bit.hash_tree_root());
        ssz::hash(&result)
    }
}

impl<T: RngCore> TestRandom<T> for AttestationDataAndCustodyBit {
    fn random_for_test(rng: &mut T) -> Self {
        Self {
            data: <_>::random_for_test(rng),
            custody_bit: false,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::test_utils::{SeedableRng, TestRandom, XorShiftRng};
    use ssz::{ssz_encode, Decodable};

    #[test]
    pub fn test_ssz_round_trip() {
        let mut rng = XorShiftRng::from_seed([42; 16]);

        let original = AttestationDataAndCustodyBit::random_for_test(&mut rng);

        let bytes = ssz_encode(&original);

        let (decoded, _) = <_>::ssz_decode(&bytes, 0).unwrap();

        assert_eq!(original, decoded);
    }

    #[test]
    pub fn test_hash_tree_root_internal() {
        let mut rng = XorShiftRng::from_seed([42; 16]);
        let original = AttestationDataAndCustodyBit::random_for_test(&mut rng);

        let result = original.hash_tree_root_internal();

        assert_eq!(result.len(), 32);
        // TODO: Add further tests
        // https://github.com/sigp/lighthouse/issues/170
    }
}
