use tokio::fs::File;
use tokio::io::{self, AsyncBufReadExt, BufReader, Lines, Result};
use tokio::sync::mpsc::Sender;

use crate::occurrence::{find_occurrences, Occurrence};

pub struct WritePayload(pub String, pub Vec<Occurrence>);

async fn read_loop_reader<R>(
    mut lines: Lines<R>,
    target: &str,
    send: Sender<WritePayload>,
) -> Result<()>
where
    R: AsyncBufReadExt + Unpin,
{
    let mut occ: Vec<Occurrence> = Vec::new();

    while let Some(line) = lines.next_line().await? {
        let trimed = line.trim();
        if trimed.len() < 1 {
            continue;
        }

        find_occurrences(&line, &target, &mut occ);

        if occ.len() == 0 {
            continue;
        }

        let _ = send
            .send(WritePayload(line.to_string(), occ.to_vec()))
            .await
            .unwrap();

        occ.clear();
    }

    Ok(())
}

async fn read_loop_stdin(target: &str, send: Sender<WritePayload>) -> Result<()> {
    let reader: BufReader<io::Stdin> = BufReader::new(io::stdin());
    let lines = reader.lines();
    return read_loop_reader(lines, target, send).await;
}

async fn read_loop_file(infile: &str, target: &str, send: Sender<WritePayload>) -> Result<()> {
    let reader = BufReader::new(File::open(infile).await?);
    let lines = reader.lines();
    return read_loop_reader(lines, target, send).await;
}

pub async fn read_loop(infile: &str, target: &str, send: Sender<WritePayload>) -> Result<()> {
    if infile.is_empty() {
        read_loop_stdin(target, send).await
    } else {
        read_loop_file(infile, target, send).await
    }
}
