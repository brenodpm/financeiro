use std::fmt::{Display, Formatter, Result};

use super::{GrupoDespesa, SubVec};

#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub enum TipoFluxo {
    Receita(String),
    Despesa(GrupoDespesa),
    Investimento,
    Retorno,
    Transferencias,
    Vazio,
}

impl TipoFluxo {
    #[inline]
    pub fn to_line(&self) -> String {
        let mut resp: Vec<String> = Vec::new();

        match self {
            TipoFluxo::Receita(nome) => {
                resp.push("Receita".to_string());
                resp.push(nome.clone());
            }
            TipoFluxo::Despesa(grupo_despesa) => {
                resp.push("Despesa".to_string());
                resp.push(grupo_despesa.to_line());
            }
            TipoFluxo::Investimento => {
                resp.push("Investimento".to_string());
            }
            TipoFluxo::Retorno => {
                resp.push("Retorno".to_string());
            }
            TipoFluxo::Transferencias => {
                resp.push("Transferencias".to_string());
            }
            TipoFluxo::Vazio => {}
        }

        resp.join(";")
    }
}

impl From<Vec<String>> for TipoFluxo {
    #[inline]
    fn from(value: Vec<String>) -> Self {
        match value[0].as_str() {
            "Receita" => TipoFluxo::Receita(value[1].clone()),
            "Despesa" => TipoFluxo::Despesa(GrupoDespesa::from(value.sub_vec())),
            "Investimento" => TipoFluxo::Investimento,
            "Retorno" => TipoFluxo::Retorno,
            "Transferencias" => TipoFluxo::Transferencias,
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
            Self::Transferencias => f.write_str("Transferencias: "),
            Self::Retorno => f.write_str("Retorno: "),
            Self::Vazio => f.write_str("????: "),
        }
    }
}
