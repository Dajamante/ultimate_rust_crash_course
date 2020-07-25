use std::env;
use std::io::{self, Read, Write};

const CHUNK_SIZE: usize = 16 * 1024;
//cargo fmt

fn main() {
    let silent = !env::var("PV_SILENT").unwrap_or_default().is_empty();
    // command D copies last line
    let mut total_bytes = 0;
    loop {
        let mut buffer = [0; CHUNK_SIZE];
        let num_read = match io::stdin().read(&mut buffer) {
            Ok(0) => break,
            Ok(x) => x,
            Err(_) => break,
        };
        total_bytes += num_read;
        io::stdout().write_all(&buffer[..num_read]).unwrap();
    }
    if !silent {
        eprintln!("num_read: {}", total_bytes);
    }
}
//ctrl D is the sign for end of file.
