use super::PtyBackend;
use anyhow::{Context, Result};
use nix::{
    libc::STDIN_FILENO,                                               // STDIN FD
    poll::{poll, PollFd, PollFlags}, 
    sys::termios::{cfmakeraw, tcgetattr, tcsetattr, SetArg, Termios}, // raw mode
    unistd::{read as nix_read, write as nix_write}, // low‑level I/O
};
use portable_pty::{native_pty_system, CommandBuilder, PtyPair, PtySize};
use std::{
    io::{Read, Write},
    sync::mpsc::Sender,
    thread,
};

/// PTY backend using `portable-pty`, raw‑mode + `poll` multiplexing.
pub struct PortablePtyBackend {
    pair: PtyPair,
}

impl PortablePtyBackend {
    pub fn new() -> Self {
        let system = native_pty_system();
        let pair = system
            .openpty(PtySize {
                rows: 24,
                cols: 80,
                pixel_width: 0,
                pixel_height: 0,
            })
            .expect("Failed to open PTY");
        PortablePtyBackend { pair }
    }

    /// Enable raw mode on stdin, returning the original settings.
    fn enable_raw_mode() -> Result<Termios> {
        let fd = STDIN_FILENO;
        let original = tcgetattr(fd).context("tcgetattr failed")?;
        let mut raw = original.clone();
        cfmakeraw(&mut raw); // disable ICANON & ECHO
        tcsetattr(fd, SetArg::TCSANOW, &raw).context("tcsetattr failed")?;
        Ok(original)
    }

    /// Restore the terminal settings saved earlier.
    fn disable_raw_mode(original: &Termios) -> Result<()> {
        let fd = STDIN_FILENO;
        tcsetattr(fd, SetArg::TCSANOW, original).context("tcsetattr restore failed")?;
        Ok(())
    }
}

impl PtyBackend for PortablePtyBackend {
    fn spawn_shell(&mut self, tx: Sender<Vec<u8>>) -> Result<()> {
        // 1) Enter raw mode for immediate keystrokes
        let orig_termios = Self::enable_raw_mode()?;

        // 2) Spawn the real shell under the PTY slave
        let shell = std::env::var("SHELL").unwrap_or_else(|_| "bash".into());
        let cmd = CommandBuilder::new(&shell);
        let _child = self
            .pair
            .slave
            .spawn_command(cmd)
            .context("spawn_command failed")?;

        // 3) Get the PTY master fd and unwrap to i32
        let master_fd = self
            .pair
            .master
            .as_raw_fd()
            .expect("Failed to get raw FD from PTY master"); // unwrap Option<i32> :contentReference[oaicite:1]{index=1}

        // 4) Prepare poll FDs
        let mut fds = [
            PollFd::new(master_fd, PollFlags::POLLIN),
            PollFd::new(STDIN_FILENO, PollFlags::POLLIN),
        ];

        // 5) Spawn I/O multiplexing thread
        thread::spawn(move || {
            let mut stdout = std::io::stdout();
            let mut buf = [0u8; 4096];

            loop {
                // block until PTY or stdin ready :contentReference[oaicite:2]{index=2}
                let _ = poll(&mut fds, -1).expect("poll failed");

                // a) PTY → index + stdout
                if fds[0].revents().unwrap().contains(PollFlags::POLLIN) {
                    let n = nix_read(master_fd, &mut buf).unwrap_or(0);
                    if n == 0 {
                        break;
                    }
                    let chunk = buf[..n].to_vec();
                    let _ = tx.send(chunk);
                    let _ = stdout.write_all(&buf[..n]);
                    let _ = stdout.flush();
                }

                // b) stdin → PTY
                if fds[1].revents().unwrap().contains(PollFlags::POLLIN) {
                    let n = nix_read(STDIN_FILENO, &mut buf[..1]).unwrap_or(0);
                    if n == 0 {
                        break;
                    }
                    let _ = nix_write(master_fd, &buf[..1]);
                }
            }

            // 6) Restore terminal
            let _ = PortablePtyBackend::disable_raw_mode(&orig_termios);
        });

        Ok(())
    }

    fn read_master(&mut self, buf: &mut [u8]) -> Result<usize> {
        let mut reader = self
            .pair
            .master
            .try_clone_reader()
            .context("clone_reader failed")?;
        Ok(reader.read(buf)?)
    }

    fn write_master(&mut self, buf: &[u8]) -> Result<usize> {
        let mut writer = self
            .pair
            .master
            .take_writer()
            .context("take_writer failed")?;
        Ok(writer.write(buf)?)
    }

    fn resize(&mut self, rows: u16, cols: u16) -> Result<()> {
        let new_size = PtySize {
            rows,
            cols,
            pixel_width: 0,
            pixel_height: 0,
        };
        self.pair.master.resize(new_size).context("resize failed")?;
        Ok(())
    }
}
