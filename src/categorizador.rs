use crate::{
    lancamento::Lancamento,
    regra::{Buscar, Regra},
};

impl Lancamento {
    pub fn categorizar(itens: Vec<Lancamento>) {
        println!("\n\nCategorizar");
        let mut pendente = Lancamento::nao_categorizados_listar();

        itens.into_iter().for_each(|novo| {
            if !pendente.iter().any(|a| a.id == novo.id) {
                pendente.push(novo);
            }
        });

        println!("Total a ser categorizados: {}", pendente.len());
        encontrar_categoria(pendente);
    }
}

fn encontrar_categoria(pendente: Vec<Lancamento>) {
    let regras = Regra::listar();
    let mut encontrados: Vec<Lancamento> = Vec::new();
    let mut nao_encontrado: Vec<Lancamento> = Vec::new();

    for mut item in pendente {
        match &regras.buscar(&item.descricao) {
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
}
