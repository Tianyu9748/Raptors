// LICENSE PLACEHOLDER
use crate::messages::*;

// wrap a dedicated executor module that only consider how to do computations
//
// TODO(long-term):
// as a interface, make refactor as Trait and expose to CRT level,
// make CRT vm to impl this trait
#[derive(Debug, PartialEq, Eq)]
pub struct Mailbox {
    // TODO replace with ringbuffer or deque
    // references:
    // *** circular-queue
    // *** ringbuffer
    // *** ringbuf
    // *** ring_queue
    mails: Vec<TypedMessage>,
}

// TODO traits tobe put together, this is a trait only for util, not for interface
pub trait Len {
    fn len(&self) -> usize;

    fn is_empty(&self) -> bool;
}

impl Len for Mailbox {
    fn is_empty(&self) -> bool {
        self.mails.is_empty()
    }

    fn len(&self) -> usize {
        self.mails.len()
    }
}

impl Mailbox {
    pub fn new() -> Self {
        return Self { mails: vec![] };
    }

    fn mails(&self) -> Vec<TypedMessage> {
        self.mails.clone()
    }

    /// ```
    /// use raptors::prelude::*;
    ///
    /// let mut mbx = Mailbox::new();
    /// let msg = SystemCommand::CreateActor(4, String::from("raptor"));
    /// mbx.enqueue(msg.into());
    /// assert_eq!(mbx.len(), 1);
    /// ```
    pub fn enqueue(&mut self, msg: TypedMessage) -> Result<(), String> {
        self.mails.push(msg);
        Ok(())
    }

    /// ```
    /// use raptors::prelude::*;
    ///
    /// let mut mbx = Mailbox::new();
    /// let msg1 = SystemCommand::CreateActor(4, String::from("raptor"));
    /// let msg2 = SystemCommand::CreateActor(2, String::from("raptor"));
    /// mbx.enqueue(msg1.into());
    /// mbx.enqueue(msg2.into());
    /// let msg = mbx.dequeue();
    /// assert_eq!(mbx.len(), 1);
    /// assert_eq!(msg.is_some(), true);
    /// assert_eq!(msg.unwrap(), SystemCommand::CreateActor(4, String::from("raptor")).into());
    /// ```
    ///
    /// test dequeue from empty message queue
    /// ```
    /// use raptors::prelude::*;
    ///
    /// let mut mbx = Mailbox::new();
    /// let msg = mbx.dequeue();
    /// assert_eq!(msg.is_none(), true);
    /// ```
    pub fn dequeue(&mut self) -> Option<TypedMessage> {
        if self.mails.is_empty() {
            None
        } else {
            Some(self.mails.remove(0))
        }
    }
}

// unit tests
#[cfg(test)]

mod tests {
    use super::*;
    use std::time;

    #[test]
    fn create_mailbox_test() {
        let mbx = Mailbox::new();
        assert_eq!(mbx.mails(), vec![]);
    }
}
