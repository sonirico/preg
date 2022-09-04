use clap::{Arg, Command};
use crossterm::style::Color;
use crossterm::style::Stylize;
use std::io::{self, Write};
struct Args {
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

#[derive(Debug, PartialEq, PartialOrd)]
struct Occurrence {
    start: usize,
    end: usize,
}

fn preg(raw: &str, matches: &str, oc: &mut Vec<Occurrence>) {
    find_occurrences(raw, matches, oc);
}

/// Buscar las ocurrencias de `literal` en `line` y guardarlas en `occurences`
fn find_occurrences(mut line: &str, literal: &str, occurences: &mut Vec<Occurrence>) {
    let len = literal.len();
    let mut offset = 0;

    // Busca la primera ocurrencia con `literal` en `line` y actualiza el slice
    // de `line` a apuntar por delante de la occurencia encontrada para buscar
    // una nueva en bucle hasta que no haya más.
    while let Some(start) = line.find(literal) {
        occurences.push(Occurrence {
            start: start + offset,
            end: start + offset + len - 1,
        });

        // Actualizar el offset y la linea, el offset se usa ya que el valor de
        // retorno de `line.find` será relativo al valor de line y este lo
        // estamos avanzando con cada ocurrencia
        offset += start + len;
        line = &line[start + len..];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Macro que genera mi propio code del test, para no tener que escribir
    /// todo el boilerplate del test para cada caso
    ///
    /// Aqui le estoy diciendo que recibe 2 parametros fijos, `$line` y `$lit`
    /// y uno que son 0 o más, que son las ocurrencias esperadas en forma de
    /// tuplas
    macro_rules! mytest {
        ($line:literal, $lit:literal, $($expected:expr),*) => {
            // Ejecutar `find_occurrences` con la entrada
            let mut occs = Vec::new();
            find_occurrences($line, $lit, &mut occs);

            // Generar un vector con las occurrencias esperadas
            let mut expected_occs = Vec::new();
            $(
                expected_occs.push(Occurrence {
                    start: $expected.0,
                    end: $expected.1
                });
            )*

            // Comparar ambos vectores
            assert_eq!(occs, expected_occs);
        }
    }

    #[test]
    fn find_occurences() {
        mytest!("[dependenciesdep]", "dep", (1, 3), (13, 15));
        mytest!("thank you cdecompilador", "ou", (7, 8));
        mytest!("thank you cdecompilador", "thank", (0, 4));
    }
}

fn output(line: &str, occurrences: &Vec<Occurrence>) {
    let mut w = io::stdout();
    let mut iter = occurrences.iter();
    let mut offset = 0;

    loop {
        if let Some(o) = iter.next() {
            write!(&mut w, "{}", line[offset..o.start].to_string());
            // write!(out_stream, "\x1B[31m{}\x1B[0m",
            //        &line[occ.start..occ.end]);
            write!(&mut w, "{}", line[o.start..o.end + 1].to_string().red());
            offset = o.end + 1;
        } else {
            write!(&mut w, "{}", line[offset..].to_string());
            break;
        }
    }

    writeln!(&mut w, "{}", "");
    io::stdout().flush().unwrap_or_default()
}

fn main() {
    let args = Args::parse();

    let mut num_line = 0;

    loop {
        num_line = num_line + 1;
        let mut buf = String::new();
        match io::stdin().read_line(&mut buf) {
            Ok(n) => {
                if n == 0 {
                    return;
                }
                let trimmed = buf.trim_end();
                let mut occurrences: Vec<Occurrence> = Vec::new();
                preg(&trimmed, &args.match_against, &mut occurrences);
                if occurrences.len() > 0 {
                    output(&trimmed, &occurrences);
                    occurrences.clear();
                }
            }
            Err(e) => {
                eprintln!("error ocurred: {e}");
                return;
            }
        }
    }
}
