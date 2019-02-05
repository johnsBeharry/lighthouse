use attester::{
    BeaconNode as AttesterBeaconNode, BeaconNodeError as NodeError,
    PublishOutcome as AttestationPublishOutcome,
};
use beacon_chain::BeaconChain;
use block_producer::{
    BeaconNode as BeaconBlockNode, BeaconNodeError as BeaconBlockNodeError,
    PublishOutcome as BlockPublishOutcome,
};
use db::ClientDB;
use parking_lot::RwLock;
use slot_clock::SlotClock;
use std::sync::Arc;
use types::{AttestationData, BeaconBlock, FreeAttestation, PublicKey, Signature};

// mod attester;
// mod producer;

/// Connect directly to a borrowed `BeaconChain` instance so an attester/producer can request/submit
/// blocks/attestations.
///
/// `BeaconBlock`s and `FreeAttestation`s are not actually published to the `BeaconChain`, instead
/// they are stored inside this struct. This is to allow one to benchmark the submission of the
/// block/attestation directly, or modify it before submission.
pub struct DirectBeaconNode<T: ClientDB, U: SlotClock> {
    beacon_chain: Arc<BeaconChain<T, U>>,
    published_blocks: RwLock<Vec<BeaconBlock>>,
    published_attestations: RwLock<Vec<FreeAttestation>>,
}

impl<T: ClientDB, U: SlotClock> DirectBeaconNode<T, U> {
    pub fn new(beacon_chain: Arc<BeaconChain<T, U>>) -> Self {
        Self {
            beacon_chain,
            published_blocks: RwLock::new(vec![]),
            published_attestations: RwLock::new(vec![]),
        }
    }

    /// Get the last published block (if any).
    pub fn last_published_block(&self) -> Option<BeaconBlock> {
        Some(self.published_blocks.read().last()?.clone())
    }

    /// Get the last published attestation (if any).
    pub fn last_published_free_attestation(&self) -> Option<FreeAttestation> {
        Some(self.published_attestations.read().last()?.clone())
    }
}

impl<T: ClientDB, U: SlotClock> AttesterBeaconNode for DirectBeaconNode<T, U> {
    fn produce_attestation_data(
        &self,
        _slot: u64,
        shard: u64,
    ) -> Result<Option<AttestationData>, NodeError> {
        match self.beacon_chain.produce_attestation_data(shard) {
            Ok(attestation_data) => Ok(Some(attestation_data)),
            Err(e) => Err(NodeError::RemoteFailure(format!("{:?}", e))),
        }
    }

    fn publish_attestation_data(
        &self,
        free_attestation: FreeAttestation,
    ) -> Result<AttestationPublishOutcome, NodeError> {
        self.published_attestations.write().push(free_attestation);
        Ok(AttestationPublishOutcome::ValidAttestation)
    }
}

impl<T: ClientDB, U: SlotClock> BeaconBlockNode for DirectBeaconNode<T, U> {
    /// Requests the `proposer_nonce` from the `BeaconChain`.
    fn proposer_nonce(&self, pubkey: &PublicKey) -> Result<u64, BeaconBlockNodeError> {
        let validator_index = self
            .beacon_chain
            .validator_index(pubkey)
            .ok_or_else(|| BeaconBlockNodeError::RemoteFailure("pubkey unknown.".to_string()))?;

        self.beacon_chain
            .proposer_slots(validator_index)
            .ok_or_else(|| {
                BeaconBlockNodeError::RemoteFailure("validator_index unknown.".to_string())
            })
    }

    /// Requests a new `BeaconBlock from the `BeaconChain`.
    fn produce_beacon_block(
        &self,
        slot: u64,
        randao_reveal: &Signature,
    ) -> Result<Option<BeaconBlock>, BeaconBlockNodeError> {
        let (block, _state) = self
            .beacon_chain
            .produce_block(randao_reveal.clone())
            .ok_or_else(|| {
                BeaconBlockNodeError::RemoteFailure(format!("Did not produce block."))
            })?;

        if block.slot == slot {
            Ok(Some(block))
        } else {
            Err(BeaconBlockNodeError::RemoteFailure(
                "Unable to produce at non-current slot.".to_string(),
            ))
        }
    }

    /// A block is not _actually_ published to the `BeaconChain`, instead it is stored in the
    /// `published_block_vec` and a successful `ValidBlock` is returned to the caller.
    ///
    /// The block may be retrieved and then applied to the `BeaconChain` manually, potentially in a
    /// benchmarking scenario.
    fn publish_beacon_block(
        &self,
        block: BeaconBlock,
    ) -> Result<BlockPublishOutcome, BeaconBlockNodeError> {
        self.published_blocks.write().push(block);
        Ok(BlockPublishOutcome::ValidBlock)
    }
}
