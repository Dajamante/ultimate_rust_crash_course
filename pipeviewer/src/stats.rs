use std::io::Result;
use std::sync::mpsc::{Receiver, Sender};

//cargo fmt made the vertical format style
pub fn stats_loop(
    silent: bool,
    stats_rx: Receiver<Vec<u8>>,
    write_tx: Sender<Vec<u8>>,
) -> Result<()> {
    let mut total_bytes = 0;
    loop {
        // blocking call
        let buffer = stats_rx.recv().unwrap();
        let num_bytes = buffer.len();
        total_bytes += buffer.len();
        if !silent {
            eprint!("\r{}", total_bytes);
        }
        //buffer consummes!
        if write_tx.send(buffer).is_err() {
            break;
        }
        if num_bytes == 0 {
            break;
        }
    }
    if !silent {
        eprintln!();
    }
    Ok(())
}
