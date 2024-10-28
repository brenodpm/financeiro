use crate::dto::{FluxoRegra, Regra};

use super::file_repy::{arq_escrever, arq_ler};

const FIN: &str = "financeiro";
const REGRAS: &str = "regras.csv";

pub trait Buscar {
    fn buscar(&self, descricao: &String, fluxo: FluxoRegra) -> Option<String>;
}

impl Buscar for Vec<Regra> {
    fn buscar(&self, descricao: &String, fluxo: FluxoRegra) -> Option<String> {
        self.into_iter()
            .find(|item| item.fluxo == fluxo && descricao.contains(&item.regex))
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

        atuais.sort_by(|a, b| b.regex.len().cmp(&a.regex.len()));

        arq_escrever(
            FIN,
            REGRAS,
            &atuais
            .into_iter()
            .map(|r| r.to_line())
            .collect(),
        );
    }
}

