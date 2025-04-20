// src/backend/mod.rs

use anyhow::Result;

/// Common interface for our PTY backends.
pub trait PtyBackend: Send {
    fn spawn_shell(&mut self) -> Result<()>;
    fn read_master(&mut self, buf: &mut [u8]) -> Result<usize>;
    fn write_master(&mut self, buf: &[u8]) -> Result<usize>;
    fn resize(&mut self, rows: u16, cols: u16) -> Result<()>;
}

#[cfg(feature = "portable-pty")]
mod portable_pty_impl;
#[cfg(feature = "portable-pty")]
pub use portable_pty_impl::PortablePtyBackend;

#[cfg(feature = "pty")]
mod pty_impl;
#[cfg(feature = "pty")]
pub use pty_impl::LowLevelPtyBackend;

pub fn create_backend() -> Box<dyn PtyBackend> {
    // If the "portable-pty" feature is enabled, return that backend:
    #[cfg(feature = "portable-pty")]
    {
        return Box::new(PortablePtyBackend::new());
    }

    // If the "pty" feature is enabled, return the low‑level backend:
    #[cfg(feature = "pty")]
    {
        return Box::new(LowLevelPtyBackend::new());
    }

    // **Only** if neither feature is active, emit a compile‑time error:
    #[cfg(not(any(feature = "portable-pty", feature = "pty")))]
    compile_error!(
        "You must compile with --features portable-pty or --features pty"
    );
}

