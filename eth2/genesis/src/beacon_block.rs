use types::{BeaconBlock, BeaconBlockBody, ChainSpec, Eth1Data, Hash256};

/// Generate a genesis BeaconBlock.
pub fn genesis_beacon_block(state_root: Hash256, spec: &ChainSpec) -> BeaconBlock {
    BeaconBlock {
        slot: spec.genesis_slot,
        parent_root: spec.zero_hash,
        state_root,
        randao_reveal: spec.empty_signature.clone(),
        eth1_data: Eth1Data {
            deposit_root: spec.zero_hash,
            block_hash: spec.zero_hash,
        },
        signature: spec.empty_signature.clone(),
        body: BeaconBlockBody {
            proposer_slashings: vec![],
            casper_slashings: vec![],
            attestations: vec![],
            custody_reseeds: vec![],
            custody_challenges: vec![],
            custody_responses: vec![],
            deposits: vec![],
            exits: vec![],
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bls::Signature;

    #[test]
    fn test_state_root() {
        let spec = ChainSpec::foundation();
        let state_root = Hash256::from("cats".as_bytes());

        let block = genesis_beacon_block(state_root, &spec);

        assert_eq!(block.state_root, state_root);
    }

    #[test]
    fn test_zero_items() {
        let spec = ChainSpec::foundation();

        let state_root = Hash256::zero();

        let genesis_block = genesis_beacon_block(state_root, &spec);

        assert!(genesis_block.slot == 0);
        assert!(genesis_block.parent_root.is_zero());
        assert_eq!(genesis_block.randao_reveal, Signature::empty_signature());
        assert!(genesis_block.eth1_data.deposit_root.is_zero());
        assert!(genesis_block.eth1_data.block_hash.is_zero());
    }

    #[test]
    fn test_beacon_body() {
        let spec = ChainSpec::foundation();

        let state_root = Hash256::zero();

        let genesis_block = genesis_beacon_block(state_root, &spec);

        // Custody items are not being implemented until phase 1 so tests to be added later

        assert!(genesis_block.body.proposer_slashings.is_empty());
        assert!(genesis_block.body.casper_slashings.is_empty());
        assert!(genesis_block.body.attestations.is_empty());
        assert!(genesis_block.body.deposits.is_empty());
        assert!(genesis_block.body.exits.is_empty());
    }

    #[test]
    fn test_signature() {
        let spec = ChainSpec::foundation();

        let state_root = Hash256::zero();

        let genesis_block = genesis_beacon_block(state_root, &spec);

        // Signature should consist of [bytes48(0), bytes48(0)]
        // Note this is implemented using Apache Milagro BLS which requires one extra byte -> 97bytes
        let raw_sig = genesis_block.signature.as_raw();
        let raw_sig_bytes = raw_sig.as_bytes();

        for item in raw_sig_bytes.iter() {
            assert!(*item == 0);
        }
        assert_eq!(genesis_block.signature, Signature::empty_signature());
    }
}
