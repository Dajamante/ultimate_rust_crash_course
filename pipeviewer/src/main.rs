use pipeviewer::{args::Args, read, stats, write};
//accessing functions through their modules
use std::io::Result;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;

//cargo fmt

fn main() -> Result<()> {
    let args = Args::parse();
    let Args {
        infile,
        outfile,
        silent,
    } = args;

    //multiple producer, single consummer!
    let (stats_tx, stats_rx) = mpsc::channel();
    let (write_tx, write_rx) = mpsc::channel();
    let (_read_tx, _read_rx): (Sender<Vec<u8>>, Receiver<Vec<u8>>) = mpsc::channel();

    // toggle hint settings OPTION H
    let read_handle = thread::spawn(move || read::read_loop(&infile, stats_tx));
    let stats_handle = thread::spawn(move || stats::stats_loop(silent, stats_rx, write_tx));
    // write thread has it's own receiver
    let write_handle = thread::spawn(move || write::write_loop(&outfile, write_rx));

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
