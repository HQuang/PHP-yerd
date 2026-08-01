//! User-terminal launching abstraction.

use std::path::Path;

use crate::PlatformError;

/// OS terminal-launch abstraction.
pub trait TerminalLauncher {
    /// Open a terminal with `path` as its working directory.
    fn open_terminal(&self, path: &Path) -> Result<(), PlatformError>;
}
