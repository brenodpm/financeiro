use crate::dto::{Categoria, FluxoRegra, Lazy, LazyFn, Regra, CSV};

use super::file_repy::{arq_escrever, arq_ler};

const FIN: &str = ".financeiro";
const REGRAS: &str = "regras.csv";

pub trait Buscar {
    fn buscar(&self, descricao: &String, fluxo: FluxoRegra) -> Option<Regra>;
}

impl Buscar for Vec<Regra> {
    fn buscar(&self, descricao: &String, fluxo: FluxoRegra) -> Option<Regra> {
        self.into_iter()
            .find(|item| item.fluxo == fluxo && descricao.contains(&item.regex))
            .map(|item| item.clone())
    }
}

impl Regra {
    pub fn listar() -> Vec<Regra> {
        let cats = Categoria::listar();
        let mut resp: Vec<Regra> = arq_ler(FIN, REGRAS).map(Regra::from_csv).collect();

        resp.iter_mut().for_each(|r| {
            r.categoria =  Lazy::Some(cats.iter().find(|c| c.id == r.categoria.id()).unwrap().clone())
        });

        resp
    }

    pub fn adicionar(regras: &mut Vec<Regra>) {
        let mut atuais = Regra::listar();
        atuais.append(regras);

        salvar(atuais);
    }

    pub fn nova(regra: Regra) {
        let mut atuais = Regra::listar();
        atuais.push(regra);

        salvar(atuais);
    }
}

fn salvar(mut regras: Vec<Regra>) {
    regras.sort_by(|a, b| b.regex.len().cmp(&a.regex.len()));

    arq_escrever(
        FIN,
        REGRAS,
        &regras.into_iter().map(|r| r.to_csv()).collect(),
    );
}
