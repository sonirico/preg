use std::fs::File;
use std::io::{self, BufWriter, ErrorKind, Write};
use std::sync::mpsc::Receiver;

use crate::occurrence::Occurrence;
use crate::read::WritePayload;

pub fn write_loop(outfile: &str, recv: Receiver<WritePayload>) -> io::Result<()> {
    let mut writer: Box<dyn Write> = if outfile.is_empty() {
        Box::new(io::stdout())
    } else {
        Box::new(BufWriter::new(File::open(outfile)?))
    };

    let mut buf: Vec<u8> = Vec::new();
    loop {
        let data: WritePayload;
        match recv.recv() {
            Ok(d) if d.0.is_empty() => break,
            Ok(d) => {
                data = d;
            }
            Err(_) => break,
        };

        output(data.0.as_str(), &data.1, &mut buf);

        if let Err(e) = writer.write_all(buf.as_mut_slice()) {
            if e.kind() == ErrorKind::BrokenPipe {
                return Ok(());
            }
            return Err(e);
        }
    }

    Ok(())
}

fn output(line: &str, occurrences: &Vec<Occurrence>, buf: &mut Vec<u8>) {
    let mut w = BufWriter::new(buf);
    let mut iter = occurrences.iter();
    let mut offset = 0;

    loop {
        if let Some(o) = iter.next() {
            write!(&mut w, "{}", line[offset..o.start].to_string());
            write!(&mut w, "\x1B[31m{}\x1B[0m", &line[o.start..o.end + 1]);
            //write!(&mut w, "{}", line[o.start..o.end + 1].to_string().red());
            offset = o.end + 1;
        } else {
            write!(&mut w, "{}", line[offset..].to_string());
            break;
        }
    }

    writeln!(&mut w, "{}", "");
}
