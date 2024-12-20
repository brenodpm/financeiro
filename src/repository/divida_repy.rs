use itertools::Itertools;

use crate::dto::{Divida, CSV};

use super::file_repy::arq_ler;

const FIN: &str = ".financeiro";
const CAT: &str = "dividas.csv";

impl Divida {
    pub fn listar() -> Vec<Divida> {
        arq_ler(FIN, CAT)
            .map(Divida::from_csv)
            .into_iter()
            .sorted_by(|a, b| a.prox_parcela().data_vencimento.partial_cmp(&b.prox_parcela().data_vencimento).unwrap())
            .collect::<Vec<Divida>>()
    }
}
