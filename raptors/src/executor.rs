// LICENSE PLACEHOLDER
//
use crate::actors;
use crate::messages;

// wrap a dedicated executor module that only consider how to do computations
//
// TODO:
// as a interface, make refactor as Trait and expose to CRT level,
// make CRT vm to impl this trait
pub struct Executor {}

impl Executor {
    pub fn new() -> Self {
        return Self {};
    }
    pub fn compute(&self, workload: messages::DummyWorkload) -> () {
        workload.mock_run();
    }
}

/// Documentation test
/// ```
/// use raptors::executor::Executor;
/// use raptors::messages::DummyWorkload;
/// use std::time;
///
/// let exec = Executor::new();
/// let load = DummyWorkload::new(16);
/// let now = std::time::Instant::now();
/// exec.compute(load);
/// assert!(now.elapsed() >= time::Duration::from_millis(16));
/// ```
fn doc_test() -> () {}

// unit tests
#[cfg(test)]

mod tests {
    use super::*;
    use std::{thread, time};

    #[test]
    fn compute_workload() {
        let exec = Executor::new();
        let load = messages::DummyWorkload::new(16);
        let now = time::Instant::now();
        exec.compute(load);
        assert!(now.elapsed() >= time::Duration::from_millis(16));
    }
}
