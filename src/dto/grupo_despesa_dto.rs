use std::fmt::{Display, Formatter, Result};

use super::TipoDespesa;

#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct GrupoDespesa {
    pub grupo: String,
    pub tipo: TipoDespesa,
}

impl From<Vec<String>> for GrupoDespesa {
    #[inline]
    fn from(value: Vec<String>) -> Self {
        GrupoDespesa {
            grupo: value[0].clone(),
            tipo: match value[1].as_str() {
                "Fixa" => TipoDespesa::Fixa,
                "Variavel" => TipoDespesa::Variavel,
                "Perda" => TipoDespesa::Perda,
                _ => TipoDespesa::Vazio,
            },
        }
    }
}

impl Display for GrupoDespesa {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.write_fmt(format_args!("{:?}: {}", self.tipo, self.grupo))
    }
}
