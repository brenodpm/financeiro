use std::collections::HashMap;

use crate::{
    calc::calc_lancamentos_filtros::Lancamentos,
    dto::{Categoria, DashGastoPorCategoria, Lancamento, OptionalLazy, OptionalLazyFn, TipoFluxo},
};

pub fn calcular_gasto_por_categoria(
    categorias: Vec<Categoria>,
    mut lancamentos: Vec<Lancamento>,
) -> Vec<DashGastoPorCategoria> {
    lancamentos = lancamentos.ultimos_dias(30);
    let mapa = maper_tipos(categorias);
    let mut resultados: Vec<DashGastoPorCategoria> = Vec::new();

    let mut tipos = DashGastoPorCategoria::new("Por tipos");

    for tp in mapa.keys() {
        let valor = por_tipo(
            tp.as_str(),
            mapa.get(tp).unwrap().clone(),
            &lancamentos,
            &mut resultados,
        );
        if valor > 0.0 {
            tipos.add(tp, valor);
        }
    }

    resultados.push(tipos);

    resultados
}

fn por_tipo(
    nome: &str,
    tp: HashMap<String, Vec<(String, String)>>,
    lancamentos: &Vec<Lancamento>,
    resultados: &mut Vec<DashGastoPorCategoria>,
) -> f64 {
    let mut total = 0.0;
    let mut tipos = DashGastoPorCategoria::new(nome);

    for grupo in tp.keys() {
        let valor = por_categoria(
            grupo.as_str(),
            tp.get(grupo).unwrap().clone(),
            lancamentos,
            resultados,
        );
        if valor > 0.0 {
            total += valor;
            tipos.add(grupo, valor);
        }
    }

    if tipos.valores.len() > 1 {
        resultados.push(tipos);
    }

    total
}

fn por_categoria(
    nome: &str,
    categorias: Vec<(String, String)>,
    lancamentos: &Vec<Lancamento>,
    resultados: &mut Vec<DashGastoPorCategoria>,
) -> f64 {
    let mut total = 0.0;

    let mut categoria = DashGastoPorCategoria::new(nome);
    for (id, nome) in categorias {
        let valor = lancamentos
            .iter()
            .filter(|l| l.categoria.id() == id)
            .map(|l| l.valor)
            .sum::<f64>()
            * -1.0;
        if valor > 0.0 {
            total += valor;
            categoria.add(&nome, valor);
        }
    }

    if categoria.valores.len() > 1 {
        resultados.push(categoria);
    }

    total
}

fn maper_tipos(
    categorias: Vec<Categoria>,
) -> HashMap<String, HashMap<String, Vec<(String, String)>>> {
    let mut tipos: HashMap<String, HashMap<String, Vec<(String, String)>>> = HashMap::new();

    for cat in categorias {
        match cat.tipo {
            TipoFluxo::Despesa(grupo) => {
                let nome = match grupo.tipo {
                    crate::dto::TipoDespesa::Fixa => "Despesas fixas",
                    crate::dto::TipoDespesa::Variavel => "Despesas variaveis",
                    crate::dto::TipoDespesa::Perda => "Perdas",
                    crate::dto::TipoDespesa::Vazio => "Sem categoria",
                };

                if !tipos.contains_key(nome) {
                    tipos.insert(nome.to_string(), HashMap::new());
                }

                let grupo_map = tipos.get_mut(nome).unwrap();

                if !grupo_map.contains_key(&grupo.grupo) {
                    grupo_map.insert(grupo.grupo.clone(), Vec::new());
                }

                let cat_vec = grupo_map.get_mut(&grupo.grupo).unwrap();
                cat_vec.push((cat.id.clone(), cat.nome.clone()));
            }
            TipoFluxo::SemCategoria => {
                let nome = "Sem categoria";

                if !tipos.contains_key(nome) {
                    tipos.insert(nome.to_string(), HashMap::new());
                }

                let grupo_map = tipos.get_mut(nome).unwrap();

                if !grupo_map.contains_key("Sem grupo") {
                    grupo_map.insert("Sem grupo".to_string(), Vec::new());
                }

                let cat_vec = grupo_map.get_mut("Sem grupo").unwrap();
                cat_vec.push((cat.id.clone(), cat.nome.clone()));
            }
            _ => {}
        }
    }

    tipos
}
