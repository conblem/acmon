#[cfg(test)]
use mockall::automock;

use std::time::{Duration, SystemTime as StdSystemTime, UNIX_EPOCH};

#[cfg_attr(test, automock)]
pub(super) trait Time: Send + Sync {
    fn now(&self) -> Duration;
}

#[derive(Default)]
pub(super) struct SystemTime;

impl Time for SystemTime {
    fn now(&self) -> Duration {
        StdSystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time not working")
    }
}