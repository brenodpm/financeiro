use crate::dto::{Categoria, Regra};

use super::file_repy::{arq_escrever, arq_ler};

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

    pub fn adicionar(regex: String, categoria: Categoria) {
        let mut atuais = Regra::listar();
        atuais.push(Regra {
            regex: regex,
            categoria: categoria,
        });
        arq_escrever(
            FIN,
            REGRAS,
            &atuais.into_iter().map(|r| r.to_string()).collect(),
        );
    }
}

