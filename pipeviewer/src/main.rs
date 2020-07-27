use pipeviewer::{args::Args, read, stats, write};
//accessing functions through their modules
use std::io::Result;
use std::sync::{Arc, Mutex};
use std::thread;

//cargo fmt

fn main() -> Result<()> {
    let args = Args::parse();
    let Args {
        infile,
        outfile,
        silent,
    } = args;

    //mutex guarding boolean
    let quit = Arc::new(Mutex::new(false));

    // three clones of quit. To coordinate clean up and closing.
    let (quit1, quit2, quit3) = (quit.clone(), quit.clone(), quit.clone());
    // command D copies last line

    let read_handle = thread::spawn(move || read::read_loop(&infile, quit1));
    let stats_handle = thread::spawn(move || stats::stats_loop(silent, quit2));
    let write_handle = thread::spawn(move || write::write_loop(&outfile, quit3));

    //join threads and crash if any thread has crashed
    //`.join()`returns a thread::Result::<io::Result<()>>

    let read_io_result = read_handle.join().unwrap();
    let stats_io_result = stats_handle.join().unwrap();
    let write_io_result = write_handle.join().unwrap();

    //return error if any of them are errors

    read_io_result?;
    stats_io_result?;
    write_io_result?;

    Ok(())
}

//ctrl D is the sign for end of file.
