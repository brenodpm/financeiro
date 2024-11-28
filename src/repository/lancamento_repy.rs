use crate::dto::{Lancamento, CSV};

use super::file_repy::{arq_escrever, arq_ler};

const FIN: &str = ".financeiro";
const NAO_CAT: &str = "nao-cat.csv";
const LANCAMENTOS: &str = "lancamentos.csv";

impl Lancamento {
    pub fn categorizar(itens: &Vec<Lancamento>){
        match itens.len() {
            0 => log::info!("nenhum novo lançamento importado"),
            1 => log::info!("um novo lançamento importado"),
            _ => log::info!("{} novos lançamentos importados", itens.len())
        } 
        
        let mut pendente = Lancamento::nao_categorizados_listar();

        itens.into_iter().for_each(|novo| {
            if !pendente.iter().any(|a| a.id == novo.id) {
                pendente.push(novo.clone());
            }
        });

        Lancamento::nao_categorizados_salvar(&pendente);
    }

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

    pub fn lancamentos_adicionar(itens: &Vec<Lancamento>) {
        let mut lista = Lancamento::lancamentos_listar();

        itens.into_iter().for_each(|novo| {
            if !lista.iter().any(|a| a.id == novo.id) {
                lista.push(novo.clone());
            }
        });

        Lancamento::lancamentos_salvar(&lista);
    }
    
    pub fn lancamentos_salvar(itens: &Vec<Lancamento>) {
        arq_escrever(
            FIN,
            LANCAMENTOS,
            &itens.into_iter().map(|i| i.to_csv()).collect(),
        )
    }
}
