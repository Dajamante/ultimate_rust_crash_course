use crossbeam::channel::{bounded, unbounded};
use pipeviewer::{args::Args, read, stats, write};
//accessing functions through their modules
use std::io::Result;
use std::thread;

// cargo fmt
// the channels are unbounded?? bounded??
// some back pressure to slow down the reads???


//yes | cargo run -- | head -n 100000000 > /dev/null
//add that
// check video 2.1 and copy command line terminal
//dd if=/dev/urandom bs=1024 count=128 of=myfile
//cat myfile | target/debug/pipeviewer > myfile2
fn main() -> Result<()> {
    let args = Args::parse();
    let Args {
        infile,
        outfile,
        silent,
    } = args;

    //multiple producer, single consummer!
    let (stats_tx, stats_rx) = unbounded();
    let (write_tx, write_rx) = bounded(1024); //is it number of attempts, or number of bytes?

    // toggle hint settings OPTION H
    let read_handle = thread::spawn(move || read::read_loop(&infile, stats_tx, write_tx));
    let stats_handle = thread::spawn(move || stats::stats_loop(silent, stats_rx));
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
