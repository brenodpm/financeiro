use crate::dto::{Categoria, GrupoDespesa, TipoDespesa, TipoFluxo};

use super::{file::arq_ler, SubVec};

const FIN: &str = "financeiro";
const CAT: &str = "categorias.csv";

impl Categoria {
    pub fn listar() -> Vec<Categoria> {
        arq_ler(FIN, CAT).map(Categoria::from).collect()
    }
}

impl From<String> for Categoria {
    #[inline]
    fn from(value: String) -> Categoria {
        let values: Vec<String> = value.split(';').map(String::from).collect();
        Categoria::from(values)
    }
}

impl From<Vec<String>> for Categoria {
    #[inline]
    fn from(value: Vec<String>) -> Categoria {
        Categoria {
            _nome: value[0].clone(),
            tipo: TipoFluxo::from(value.sub_vec()),
        }
    }
}

impl From<Vec<String>> for TipoFluxo {
    #[inline]
    fn from(value: Vec<String>) -> Self {
        match value[0].as_str() {
            "Receitas" => TipoFluxo::Receitas(value[1].clone()),
            "Despesas" => TipoFluxo::Despesas(GrupoDespesa::from(value.sub_vec())),
            "Investimento" => TipoFluxo::Investimento,
            "Retorno" => TipoFluxo::Retorno,
            _ => TipoFluxo::Vazio,
        }
    }
}

impl From<Vec<String>> for GrupoDespesa {
    #[inline]
    fn from(value: Vec<String>) -> Self {
        GrupoDespesa {
            nome: value[0].clone(),
            tipo: match value[1].as_str() {
                "Fixa" => TipoDespesa::Fixa,
                "Variavel" => TipoDespesa::Variavel,
                "Perdas" => TipoDespesa::Perdas,
                _ => TipoDespesa::Vazio,
            },
        }
    }
}
