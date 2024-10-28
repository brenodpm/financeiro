use crate::dto::{Lancamento, CSV};

use super::file_repy::{arq_escrever, arq_ler};

const FIN: &str = "financeiro";
const NAO_CAT: &str = "nao-cat.csv";
const LANCAMENTOS: &str = "lancamentos.csv";

impl Lancamento {
    pub fn nao_categorizados_listar() -> Vec<Lancamento> {
        arq_ler(FIN, NAO_CAT).map(Lancamento::from_csv).collect()
    }

    pub fn nao_categorizados_salvar(itens: &Vec<Lancamento>) {
        arq_escrever(
            FIN,
            NAO_CAT,
            &itens.into_iter().map(|i| i.to_csv()).collect(),
        )
    }

    pub fn lancamentos_listar() -> Vec<Lancamento>{
        arq_ler(FIN, LANCAMENTOS).map(Lancamento::from_csv).collect()
    }
    
    pub fn lancamentos_salvar(itens: &Vec<Lancamento>) {
        arq_escrever(
            FIN,
            LANCAMENTOS,
            &itens.into_iter().map(|i| i.to_csv()).collect(),
        )
    }
}
