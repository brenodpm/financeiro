use std::fmt::{Display, Formatter, Result};

use serde::{Deserialize, Serialize};

use super::GrupoDespesa;

#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum TipoFluxo {
    Receita(String),
    Despesa(GrupoDespesa),
    Investimento,
    Retorno,
    Transferencias,
    SemCategoria,
}

impl TipoFluxo {
    pub fn new(tipo: &str, grupo: &str, sub_grupo: &str)->Self{
        match tipo {
            "Receita" => TipoFluxo::Receita(sub_grupo.to_string()),
            "Despesa" => TipoFluxo::Despesa(GrupoDespesa::new(grupo.clone(), sub_grupo.clone())),
            "Investimento" => TipoFluxo::Investimento,
            "Retorno" => TipoFluxo::Retorno,
            "Transferencias" => TipoFluxo::Transferencias,
            _ => TipoFluxo::SemCategoria,
        }
    }
}

impl Display for TipoFluxo {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::Receita(nome) => f.write_fmt(format_args!("Receita: {nome} - ")),
            Self::Despesa(grp) => f.write_fmt(format_args!("Despesa {grp} - ")),
            Self::Investimento => f.write_str("Investimento: "),
            Self::Transferencias => f.write_str("Transferencias: "),
            Self::Retorno => f.write_str("Retorno: "),
            Self::SemCategoria => f.write_str("Sem categoria"),
        }
    }
}
