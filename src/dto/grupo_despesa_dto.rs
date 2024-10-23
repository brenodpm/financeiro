use std::fmt::{Display, Formatter, Result};

use super::TipoDespesa;

#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct GrupoDespesa {
    pub grupo: String,
    pub tipo: TipoDespesa,
}
impl GrupoDespesa{
    #[inline]
    pub fn to_line(&self) -> String {
        let mut resp: Vec<String> = Vec::new();

        resp.push(self.grupo.clone());
        match self.tipo {
            TipoDespesa::Fixa => resp.push("Fixa".to_string()),
            TipoDespesa::Variavel => resp.push("Variavel".to_string()),
            TipoDespesa::Perda => resp.push("Perda".to_string()),
            TipoDespesa::Vazio => resp.push("".to_string()),
        }

        resp.join(";")
    }
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
