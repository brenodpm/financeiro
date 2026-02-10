use std::fmt::{Display, Formatter, Result};

use serde::{Deserialize, Serialize};

use super::TipoDespesa;

#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct GrupoDespesa {
    pub grupo: String,
    pub tipo: TipoDespesa,
}

impl GrupoDespesa {
    pub fn new(grupo: &str, sub_grupo: &str)->Self{
        Self{
            grupo: grupo.to_string(),
            tipo: match sub_grupo {
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
        f.write_fmt(format_args!("{:?}; {}", self.tipo, self.grupo))
    }
}
