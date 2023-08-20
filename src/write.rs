use tokio::fs::File;
use tokio::io::{self, AsyncWrite, AsyncWriteExt, BufWriter, ErrorKind};
use tokio::sync::mpsc::Receiver;

use crate::occurrence::Occurrence;
use crate::read::WritePayload;

pub async fn write_loop(outfile: &str, mut recv: Receiver<WritePayload>) -> io::Result<()> {
    //let mut writer: Box<dyn AsyncWrite> = if outfile.is_empty() {
    //    Box::new(io::stdout())
    //} else {
    //    Box::new(BufWriter::new(File::open(outfile).await?))
    //};

    let mut writer = BufWriter::new(io::stdout());

    let mut buf: Vec<u8> = Vec::new();

    while let Some(d) = recv.recv().await {
        output(d.0.as_str(), &d.1, &mut buf).await?;

        if let Err(e) = writer.write_all(buf.as_mut_slice()).await {
            if e.kind() == ErrorKind::BrokenPipe {
                return Ok(());
            }
            return Err(e);
        }
    }

    Ok(())
}

//macro_rules! async_write {
//    ($dst: expr) => {
//        {
//            tokio::io::AsyncWriteExt::write_all(&mut $dst).await
//        }
//    };
//    ($dst: expr, $fmt: expr) => {
//        {
//            use std::io::Write;
//            let mut buf = Vec::<u8>::new();
//            writeln!(buf, $fmt)?;
//            tokio::io::AsyncWriteExt::write_all(&mut $dst, &buf).await
//        }
//    };
//    ($dst: expr, $fmt: expr, $($arg: tt)*) => {
//        {
//            use std::io::Write;
//            let mut buf = Vec::<u8>::new();
//            writeln!(buf, $fmt, $( $arg )*)?;
//            tokio::io::AsyncWriteExt::write_all(&mut $dst, &buf).await
//        }
//    };
//}
//macro_rules! async_writeln {
//    ($dst: expr) => {
//        {
//            tokio::io::AsyncWriteExt::write_all(&mut $dst, b"\n").await
//        }
//    };
//    ($dst: expr, $fmt: expr) => {
//        {
//            use std::io::Write;
//            let mut buf = Vec::<u8>::new();
//            writeln!(buf, $fmt)?;
//            tokio::io::AsyncWriteExt::write_all(&mut $dst, &buf).await
//        }
//    };
//    ($dst: expr, $fmt: expr, $($arg: tt)*) => {
//        {
//            use std::io::Write;
//            let mut buf = Vec::<u8>::new();
//            writeln!(buf, $fmt, $( $arg )*)?;
//            tokio::io::AsyncWriteExt::write_all(&mut $dst, &buf).await
//        }
//    };
//}

async fn output(line: &str, occurrences: &Vec<Occurrence>, buf: &mut Vec<u8>) -> io::Result<()> {
    let mut w = BufWriter::new(buf);
    let mut iter = occurrences.iter();
    let mut offset = 0;

    loop {
        if let Some(o) = iter.next() {
            w.write_all(line[offset..o.start].as_bytes()).await?;
            w.write_all(format!("\x1B[31m{}\x1B[0m", &line[o.start..o.end + 1]).as_bytes())
                .await?;
            //w.write_all(&line[o.start..o.end + 1].as_bytes()).await?;
            w.flush().await?; // Flush the buffer
            offset = o.end + 1;
        } else {
            w.write_all(&line[offset..].as_bytes()).await?;
            w.flush().await?; // Flush the buffer
            break;
        }
    }

    w.write_all(b"\n").await?;
    w.flush().await?; // Flush the buffer

    Ok(())
}
