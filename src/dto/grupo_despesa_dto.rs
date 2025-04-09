use std::fmt::{Display, Formatter, Result};

use serde::{Deserialize, Serialize};

use super::{TipoDespesa, CSV};

#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct GrupoDespesa {
    pub grupo: String,
    pub tipo: TipoDespesa,
}

impl CSV for GrupoDespesa {
    fn from_csv(value: String) -> Self {
        let values: Vec<String> = value.split(';').map(String::from).collect();
        GrupoDespesa::from_csv_vec(values)
    }

    fn from_csv_vec(value: Vec<String>) -> Self {
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

    fn to_csv(&self) -> String {
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

impl Display for GrupoDespesa {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.write_fmt(format_args!("{:?}: {}", self.tipo, self.grupo))
    }
}
