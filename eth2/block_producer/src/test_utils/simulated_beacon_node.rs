use crate::traits::{BeaconNode, BeaconNodeError, PublishOutcome};
use std::sync::RwLock;
use types::{BeaconBlock, Signature, Slot};

type ProduceResult = Result<Option<BeaconBlock>, BeaconNodeError>;
type PublishResult = Result<PublishOutcome, BeaconNodeError>;

/// A test-only struct used to simulate a Beacon Node.
#[derive(Default)]
pub struct SimulatedBeaconNode {
    pub produce_input: RwLock<Option<(Slot, Signature)>>,
    pub produce_result: RwLock<Option<ProduceResult>>,

    pub publish_input: RwLock<Option<BeaconBlock>>,
    pub publish_result: RwLock<Option<PublishResult>>,
}

impl SimulatedBeaconNode {
    /// Set the result to be returned when `produce_beacon_block` is called.
    pub fn set_next_produce_result(&self, result: ProduceResult) {
        *self.produce_result.write().unwrap() = Some(result);
    }

    /// Set the result to be returned when `publish_beacon_block` is called.
    pub fn set_next_publish_result(&self, result: PublishResult) {
        *self.publish_result.write().unwrap() = Some(result);
    }
}

impl BeaconNode for SimulatedBeaconNode {
    /// Returns the value specified by the `set_next_produce_result`.
    fn produce_beacon_block(&self, slot: Slot, randao_reveal: &Signature) -> ProduceResult {
        *self.produce_input.write().unwrap() = Some((slot, randao_reveal.clone()));
        match *self.produce_result.read().unwrap() {
            Some(ref r) => r.clone(),
            None => panic!("SimulatedBeaconNode: produce_result == None"),
        }
    }

    /// Returns the value specified by the `set_next_publish_result`.
    fn publish_beacon_block(&self, block: BeaconBlock) -> PublishResult {
        *self.publish_input.write().unwrap() = Some(block);
        match *self.publish_result.read().unwrap() {
            Some(ref r) => r.clone(),
            None => panic!("SimulatedBeaconNode: publish_result == None"),
        }
    }
}
