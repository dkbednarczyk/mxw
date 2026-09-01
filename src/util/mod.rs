pub mod color;
pub mod key;
pub mod status;

use std::time::Duration;

/// Pause around feature-report I/O, giving the device time to process
/// commands before the next read/write.
pub(crate) const IO_DELAY: Duration = Duration::from_millis(50);
