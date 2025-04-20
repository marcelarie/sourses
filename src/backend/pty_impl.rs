use super::PtyBackend;
use pty::fork::*;

pub struct LowLevelPtyBackend {
    fork: Fork,
}

impl LowLevelPtyBackend {
    pub fn new() -> Self {
        let fork = Fork::from_ptmx().unwrap();
        LowLevelPtyBackend { fork }
    }
}

impl PtyBackend for LowLevelPtyBackend {
    fn spawn_shell(&mut self) -> anyhow::Result<()> {
        if let Some(mut master) = self.fork.is_parent().ok() {
            // in parent: read/write master
        } else {
            // in child: exec bash
            std::process::Command::new("bash").status()?;
        }
        Ok(())
    }
}
