use itertools::Itertools;

use crate::dto::{Categoria, Categorias, TipoFluxo};

use super::file_repy::arq_ler;

const FIN: &str = "financeiro";
const CAT: &str = "categorias.csv";

impl Categoria {
    pub fn listar() -> Categorias {
        let mut resp = Categorias {
            receitas: Vec::new(),
            despesas: Vec::new(),
        };

        arq_ler(FIN, CAT)
            .map(Categoria::from)
            .into_iter()
            .sorted_by(|a, b| a.partial_cmp(b).unwrap())
            .for_each(|c| match c.tipo.clone() {
                TipoFluxo::Receita(_) => resp.receitas.push(c),
                TipoFluxo::Retorno => resp.receitas.push(c),

                TipoFluxo::Despesa(_) => resp.despesas.push(c),
                TipoFluxo::Investimento => resp.despesas.push(c),

                TipoFluxo::Vazio => {}
            });


        resp
    }
}
