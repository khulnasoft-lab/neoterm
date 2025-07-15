use portable_pty::{CommandBuilder, NativePtySystem, PtySize, PtySystem};
use std::io::{Read, Write};
use std::sync::mpsc::{Sender, Receiver, channel};
use std::thread;

// A handle to the PTY master, allowing us to write to the shell
pub type PtyWriter = Box<dyn Write + Send>;
// A receiver for the PTY output
pub type PtyReader = Receiver<Vec<u8>>;

// Spawns a shell and returns a writer and a reader for it
pub fn spawn_shell() -> (PtyWriter, PtyReader) {
    let pty_system = NativePtySystem::default();
    let pair = pty_system
        .openpty(PtySize {
            rows: 24,
            cols: 80,
            pixel_width: 0,
            pixel_height: 0,
        })
        .expect("Failed to create PTY");

    // For this scaffold, we'll hardcode bash.
    // In a real app, you'd get the user's default shell from env vars.
    let cmd = CommandBuilder::new("bash");
    let mut child = pair.slave.spawn_command(cmd).expect("Failed to spawn shell");

    // The writer is the master PTY. We can write to it to send commands.
    let writer = pair.master.try_clone_writer().expect("Failed to get PTY writer");
    
    // The reader needs to run in a separate thread to avoid blocking the UI
    let (tx, rx) = channel();
    let mut reader = pair.master.try_clone_reader().expect("Failed to get PTY reader");

    thread::spawn(move || {
        let mut buf = [0u8; 4096];
        loop {
            match reader.read(&mut buf) {
                Ok(n) if n > 0 => {
                    if tx.send(buf[..n].to_vec()).is_err() {
                        // The receiver has been dropped, so we can exit
                        break;
                    }
                }
                _ => break, // Error or EOF
            }
        }
    });

    (Box::new(writer), rx)
} 