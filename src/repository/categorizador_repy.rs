use crate::dto::{FluxoRegra, Lancamento, Regra};

use super::regra_repy::Buscar;

impl Lancamento {
    pub fn recategorizar() {
        Lancamento::categorizar(Vec::new());
    }

    pub fn categorizar(itens: Vec<Lancamento>) {
        log::info!("Categorizar");
        let mut pendente = Lancamento::nao_categorizados_listar();

        itens.into_iter().for_each(|novo| {
            if !pendente.iter().any(|a| a.id == novo.id) {
                pendente.push(novo);
            }
        });

        log::info!("Total a ser categorizados: {}", pendente.len());
        encontrar_categoria(pendente);
    }
}

fn encontrar_categoria(pendente: Vec<Lancamento>) {
    let regras = Regra::listar();
    let mut encontrados: Vec<Lancamento> = Vec::new();
    let mut nao_encontrado: Vec<Lancamento> = Vec::new();

    for mut item in pendente {
        match &regras.buscar(
            &item.descricao.to_lowercase(),
            if item.valor > 0.0 {
                FluxoRegra::Entrada
            } else {
                FluxoRegra::Saida
            },
        ) {
            Some(c) => {
                item.categoria = Some(c.clone());
                encontrados.push(item);
            }
            None => {
                nao_encontrado.push(item);
            }
        }
    }

    Lancamento::nao_categorizados_salvar(&nao_encontrado);
    adicionar_categorizados(encontrados);
}

fn adicionar_categorizados(novos: Vec<Lancamento>) {
    let mut lancamentos = Lancamento::lancamentos_listar();
    novos.into_iter().for_each(|novo| {
        if !lancamentos.iter().any(|a| a.id == novo.id) {
            lancamentos.push(novo);
        }
    });
    Lancamento::lancamentos_salvar(&lancamentos);
}
