// src/backend/portable_pty_impl.rs

use super::PtyBackend;
use anyhow::Result;
use portable_pty::{native_pty_system, CommandBuilder, PtyPair, PtySize};

/// High‑level PTY backend using the `portable-pty` crate.
pub struct PortablePtyBackend {
    pair: PtyPair,
}

impl PortablePtyBackend {
    /// Create a new PTY and prepare to spawn the shell.
    pub fn new() -> Self {
        let pty_system = native_pty_system();
        let pair: PtyPair = pty_system
            .openpty(PtySize {
                rows: 24,
                cols: 80,
                pixel_width: 0,
                pixel_height: 0,
            })
            .expect("failed to open PTY"); // handle errors as you like
        PortablePtyBackend { pair }
    }
}

impl PtyBackend for PortablePtyBackend {
    fn spawn_shell(&mut self) -> Result<()> {
        // Spawn bash (or $SHELL) on the slave end
        let cmd = CommandBuilder::new("bash");
        let _child = self.pair.slave.spawn_command(cmd)?;
        Ok(())
    }

    fn read_master(&mut self, buf: &mut [u8]) -> Result<usize> {
        // Clone a reader and read bytes
        let mut reader = self.pair.master.try_clone_reader()?;
        let n = reader.read(buf)?;
        Ok(n)
    }

    fn write_master(&mut self, buf: &[u8]) -> Result<usize> {
        // Write keystrokes to the master
        let mut writer = self.pair.master.take_writer()?;
        let n = writer.write(buf)?;
        Ok(n)
    }

    fn resize(&mut self, rows: u16, cols: u16) -> Result<()> {
        // Build a PtySize and pass that one struct into resize()
        let new_size = PtySize {
            rows,
            cols,
            pixel_width: 0,
            pixel_height: 0,
        };
        self.pair.master.resize(new_size)?; // expects a single PtySize :contentReference[oaicite:0]{index=0}
        Ok(())
    }
}
