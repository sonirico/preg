use preg::args::Args;
use preg::read::{read_loop, WritePayload};
use preg::write::write_loop;
use std::io::Result;
use std::sync::mpsc::channel;
use std::thread;

fn main() -> Result<()> {
    let args = Args::parse();

    let (sender, receiver) = channel::<WritePayload>();

    let read_handle = thread::spawn(move || read_loop("", &args.match_against, sender));

    let read_res = read_handle.join().unwrap();

    let _ = write_loop("", receiver);

    read_res?;

    Ok(())
}
