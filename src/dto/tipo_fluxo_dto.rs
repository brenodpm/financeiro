use std::fmt::{Display, Formatter, Result};

use super::{GrupoDespesa, SubVec};

#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub enum TipoFluxo {
    Receita(String),
    Despesa(GrupoDespesa),
    Investimento,
    Retorno,
    Vazio,
}

impl From<Vec<String>> for TipoFluxo {
    #[inline]
    fn from(value: Vec<String>) -> Self {
        match value[0].as_str() {
            "Receita" => TipoFluxo::Receita(value[1].clone()),
            "Despesa" => TipoFluxo::Despesa(GrupoDespesa::from(value.sub_vec())),
            "Investimento" => TipoFluxo::Investimento,
            "Retorno" => TipoFluxo::Retorno,
            _ => TipoFluxo::Vazio,
        }
    }
}

impl Display for TipoFluxo {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::Receita(nome) => f.write_fmt(format_args!("Receita: {nome} -")),
            Self::Despesa(grp) => f.write_fmt(format_args!("Despesa {grp} -")),
            Self::Investimento => f.write_str("Investimento: "),
            Self::Retorno => f.write_str("Retorno: "),
            Self::Vazio => f.write_str("????: "),
        }
    }
}
