use clap::{App, Arg};
use preg::{
    read::{read_loop, WritePayload},
    write::write_loop,
};
use tokio::sync::mpsc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("preg")
        .arg(
            Arg::new("matches")
                .short('m')
                .long("matches")
                .value_name("example")
                .help("string occurrence to match against"),
        )
        .arg(
            Arg::new("infile")
                .short('f')
                .long("infile")
                .value_name("infile")
                .help("file to read from"),
        )
        .get_matches();

    let match_against = matches.value_of("matches").unwrap_or_default().to_string();
    let infile = matches.value_of("infile").unwrap_or_default().to_string();

    let (tx, rx) = mpsc::channel::<WritePayload>(32);

    let read_task = tokio::spawn(async move { read_loop(&infile, &match_against, tx).await });
    let write_task = tokio::spawn(async move {
        let _ = write_loop("", rx).await;
    });

    let _ = read_task.await.unwrap();
    write_task.await.unwrap();

    Ok(())
}
