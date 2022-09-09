use clap::{Arg, Command};

pub struct Args {
    pub match_against: String,
}

impl Args {
    pub fn parse() -> Args {
        let matches = Command::new("preg")
            .arg(
                Arg::new("matches")
                    .short('m')
                    .long("matches")
                    .value_name("example")
                    .help("string occurrence to match against"),
            )
            .get_matches();

        Self {
            match_against: matches.value_of("matches").unwrap_or_default().to_string(),
        }
    }
}
