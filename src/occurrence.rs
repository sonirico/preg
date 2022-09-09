#[derive(Debug, PartialEq, PartialOrd)]
pub struct Occurrence {
    pub start: usize,
    pub end: usize,
}

impl Clone for Occurrence {
    fn clone(&self) -> Self {
        Self {
            start: self.start,
            end: self.end,
        }
    }
}
/// Buscar las ocurrencias de `literal` en `line` y guardarlas en `occurences`
pub fn find_occurrences(mut line: &str, literal: &str, occurences: &mut Vec<Occurrence>) {
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
        mytest!("thank", "thank", (0, 4));
    }
}
