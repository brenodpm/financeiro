use std::vec::IntoIter;

use itertools::Itertools;

use crate::dto::Categoria;

use super::file_repy::arq_ler;

const FIN: &str = "financeiro";
const CAT: &str = "categorias.csv";

impl Categoria {
    pub fn listar() -> IntoIter<Categoria> {
        arq_ler(FIN, CAT)
            .map(Categoria::from)
            .into_iter()
            .sorted_by(|a, b| a.partial_cmp(b).unwrap())
    }
}
