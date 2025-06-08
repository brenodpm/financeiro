use crate::dto::{Lancamento, OptionalLazy};

use super::file_repy::{arq_escrever, arq_ler};

const FIN: &str = ".financeiro";
const NAO_CAT: &str = "nao-cat.json";
const LANCAMENTOS: &str = "lancamentos.json";

impl Lancamento {
    pub fn categorizar(itens: &Vec<Lancamento>) {
        match itens.len() {
            0 => log::info!("nenhum novo lançamento importado"),
            1 => log::info!("um novo lançamento importado"),
            _ => log::info!("{} novos lançamentos importados", itens.len()),
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
        let mut json: String = arq_ler(FIN, NAO_CAT).collect();
        if json.is_empty() {
            json = "[]".to_string();
        }
        serde_json::from_str(&json).unwrap()
    }

    pub fn nao_categorizados_salvar(itens: &Vec<Lancamento>) {
        arq_escrever(FIN, NAO_CAT, serde_json::to_string(&itens).unwrap());
    }

    pub fn lancamentos_listar() -> Vec<Lancamento> {
        let mut json: String = arq_ler(FIN, LANCAMENTOS).collect();
        if json.is_empty() {
            json = "[]".to_string();
        }
        serde_json::from_str(&json).unwrap()
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
        let mut salvar = itens.clone();

        for lanc in salvar.iter_mut() {
            if let OptionalLazy::Some(t) = lanc.categoria.clone() {
                lanc.categoria = OptionalLazy::Id(t.id);
            }
            if let OptionalLazy::Some(r) = lanc.regra.clone() {
                lanc.regra = OptionalLazy::Id(r.id);
            }
        }

        arq_escrever(FIN, LANCAMENTOS, serde_json::to_string(&salvar).unwrap());
    }

    pub fn lancamentos_recategorizar(&self) {
        let mut nao_cat = Lancamento::nao_categorizados_listar();
        nao_cat.push(self.clone());
        Lancamento::nao_categorizados_salvar(&nao_cat);

        let cat: Vec<Lancamento> = Lancamento::lancamentos_listar()
            .into_iter()
            .filter(|l| l.id != self.id)
            .collect();
        Lancamento::lancamentos_salvar(&cat);
    }
}
