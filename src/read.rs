use std::fs::File;
use std::io::{self, BufRead, BufReader, Result};
use std::sync::mpsc::Sender;

use crate::occurrence::{find_occurrences, Occurrence};

pub struct WritePayload(pub String, pub Vec<Occurrence>);

pub fn read_loop(infile: &str, target: &str, send: Sender<WritePayload>) -> Result<()> {
    let mut reader: Box<dyn BufRead> = if infile.is_empty() {
        Box::new(BufReader::new(io::stdin()))
    } else {
        Box::new(BufReader::new(File::open(infile)?))
    };

    let mut occ: Vec<Occurrence> = Vec::new();
    let mut line: String = String::new();

    loop {
        line.clear();
        let read = reader.read_line(&mut line)?;
        if read == 0 {
            break;
        }

        let trimed = line.trim();
        if trimed.len() < 1 {
            continue;
        }

        find_occurrences(&line, &target, &mut occ);

        if occ.len() > 0 {
            let _ = send.send(WritePayload(line.to_string(), occ.to_vec()));
            occ.clear();
        }
    }

    Ok(())
}
