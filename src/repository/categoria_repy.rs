use itertools::Itertools;

use crate::dto::{Categoria, Unico, CSV};

use super::file_repy::{arq_escrever_linhas, arq_ler};

const FIN: &str = ".financeiro";
const CAT: &str = "categorias.csv";

impl Categoria {
    pub fn listar() -> Vec<Categoria> {
        let mut resp: Vec<Categoria> = arq_ler(FIN, CAT)
            .map(Categoria::from_csv)
            .into_iter()
            .sorted_by(|a, b| a.to_string().partial_cmp(&b.to_string()).unwrap())
            .collect();

        if resp.len() == 0usize {
            resp = Categoria::lista_padrao();
            Categoria::salvar(&mut resp);
        }

        resp
    }

    pub fn salvar(categorias: &mut Vec<Categoria>) {
        categorias.into_iter().for_each(|c| {
            if c.id.is_empty() {
                c.gerar_id();
            }
        });

        arq_escrever_linhas(
            FIN,
            CAT,
            &categorias.into_iter().map(|i| i.to_csv()).collect(),
        )
    }
}
