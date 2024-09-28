use crate::dto::{Categoria, Regra};

use super::{file::arq_ler, SubVec};

const FIN: &str = "financeiro";
const REGRAS: &str = "regras.csv";


pub trait Buscar {
    fn buscar(&self, descricao: &String) -> Option<Categoria>;
}

impl Buscar for Vec<Regra> {
    fn buscar(&self, descricao: &String) -> Option<Categoria> {
        self.into_iter()
            .find(|item| descricao.contains(&item.regex))
            .map(|item| item.categoria.clone())
    }
}

impl Regra {
    pub fn listar() -> Vec<Regra> {
        arq_ler(FIN, REGRAS).map(Regra::from).collect()
    }
}

impl From<String> for Regra {
    #[inline]
    fn from(s: String) -> Regra {
        let attrs: Vec<String> = s.split(';').map(String::from).collect();
        Regra {
            regex: attrs[0].clone(),
            categoria: Categoria::from(attrs.sub_vec()),
        }
    }
}