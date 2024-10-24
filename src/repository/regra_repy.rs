use crate::dto::Regra;

use super::file_repy::{arq_escrever, arq_ler};

const FIN: &str = "financeiro";
const REGRAS: &str = "regras.csv";

pub trait Buscar {
    fn buscar(&self, descricao: &String) -> Option<String>;
}

impl Buscar for Vec<Regra> {
    fn buscar(&self, descricao: &String) -> Option<String> {
        self.into_iter()
            .find(|item| descricao.contains(&item.regex))
            .map(|item| item.categoria.clone())
    }
}

impl Regra {
    pub fn listar() -> Vec<Regra> {
        arq_ler(FIN, REGRAS).map(Regra::from).collect()
    }

    pub fn adicionar(regras: &mut Vec<Regra>) {
        let mut atuais = Regra::listar();
        atuais.append(regras);
        arq_escrever(
            FIN,
            REGRAS,
            &atuais.into_iter().map(|r| r.to_line()).collect(),
        );
    }
}

