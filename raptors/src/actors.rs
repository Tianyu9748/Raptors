// LICENSE PLACEHOLDER
use uuid::{Urn, Uuid};

use std::collections::HashMap;

use crate::mailbox::*;
use crate::messages;
use crate::workloads::{OpCode, Workload};
use std::cmp::Ordering;
use std::str::Bytes;

// placehold for actors
#[derive(Debug, Eq)]
pub struct Actor {
    name: String,
    // TODO(long-term) use v5 uuid, and give a hardcoded namespace, for removing randomness, also to allow
    // testing
    id: Uuid,
    pub addr: Address,
    // TODO(albert), how to access mailboxes from actor if mailboxes are owned by
    // system/context/environment
    pub mbx: Mailbox,
}

impl Actor {
    pub fn new(name: &str) -> Actor {
        let new_uuid = Uuid::new_v4();
        let _addr = new_uuid.clone().hyphenated().to_string();
        return Self {
            name: String::from(name),
            id: new_uuid,
            addr: Address::new(_addr),
            mbx: Mailbox::new(),
        };
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn addr(&self) -> Address {
        self.addr.clone()
    }

    pub fn mailbox(&self) -> &Mailbox {
        &self.mbx
    }

    /// ```
    /// use raptors::prelude::*;
    ///
    /// let mut actor = Actor::new("A");
    /// let mbx = Mailbox::new();
    /// //actor.set_mbx(&mbx);
    /// ```
    // pub fn set_mbx(&mut self, registry: &'a HashMap<Address, Mailbox>) -> () {
    // pub fn set_mbx(&mut self, mbx: &'a Mailbox) -> () {
    //self.mbx = Some(registry.get(&self.addr()).unwrap());
    //    self.mbx = Some(mbx);
    //    ()
    //}

    // pub fn mbx(&self) -> &Mailbox {
    //     &self.mailbox
    // }

    // TODO: make it message passing, test with inter-threads
    // TODO: gradually support higher granularity parallelism
    pub fn receive_workload(&self, msg: Workload) -> () {
        self.on_compute(msg);
    }

    /// TODO Error Handling
    /// ```
    /// use raptors::prelude::*;
    ///
    /// let mut actor = Actor::new("raptor");
    /// let msg = TypedMessage::WorkloadMsg(Workload::new(OpCode::AddOp));
    /// actor.receive_msg(msg.into());
    /// // assert_eq!(actor.mailbox.len(), 1);
    /// ```
    pub fn receive_msg(&mut self, msg: messages::TypedMessage) -> Result<(), String> {
        match msg {
            messages::TypedMessage::WorkloadMsg(ref workload) => {
                // self.mailbox.enqueue(msg.into());
                Ok(())
            }
            _ => Err("Unknown message received by actor".to_string()),
        }
    }

    fn on_compute(&self, workload: Workload) -> () {
        workload.mock_run();
    }
}

impl PartialOrd for Actor {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Actor {
    fn cmp(&self, other: &Self) -> Ordering {
        self.name.cmp(&other.name)
    }
}

// TODO fix duplicate with uuid add to name
impl PartialEq for Actor {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

// unit tests
#[cfg(test)]

mod tests {
    use super::*;
    use std::time;

    // test visibility
    #[test]
    fn create_dummy_workload_test() {
        let load = Workload::new(OpCode::AddOp);
        assert_eq!(load.payload(), 11 as usize);
    }

    #[test]
    fn workload_mock_run_test() {
        let load = Workload::new(OpCode::AddOp);
        let now = time::Instant::now();
        load.mock_run();
        assert!(now.elapsed() >= time::Duration::from_millis(11));
    }

    #[test]
    fn query_actor_name() {
        let actor = Actor::new("A");
        assert_eq!(actor.name(), "A");
    }

    #[test]
    fn receive_workload() {
        let actor = Actor::new("A");
        let load = Workload::new(OpCode::AddOp);
        let now = time::Instant::now();
        actor.receive_workload(load);
        assert!(now.elapsed() >= time::Duration::from_millis(11));
    }
}
