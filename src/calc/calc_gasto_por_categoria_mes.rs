use std::{collections::HashMap, fmt::format};

use crate::{
    calc::calc_lancamentos_filtros::Lancamentos,
    dto::{Categoria, DashGastoPorCategoria, Lancamento, OptionalLazyFn},
};

pub fn calcular_gasto_por_categoria_d30(
    ordem: Vec<String>,
    mut lancamentos: Vec<Lancamento>,
) -> Vec<DashGastoPorCategoria> {
    lancamentos = lancamentos.ultimos_dias(30);
    let mut mapa: HashMap<String, HashMap<String, f64>> = HashMap::new();

    agrupar_lancamentos_em_grupos(&mut mapa, lancamentos);

    gerar_graficos(ordem, mapa)
}

fn gerar_graficos(
    ordem: Vec<String>,
    mapa: HashMap<String, HashMap<String, f64>>,
) -> Vec<DashGastoPorCategoria> {
    let mut resultados: Vec<DashGastoPorCategoria> = Vec::new();

    for nome in ordem {
        if let Some(valores) = mapa.get(&nome) {
            se_mais_de_1_item_criar_grafico(&mut resultados, nome, valores);
        }
    }

    resultados
}

fn se_mais_de_1_item_criar_grafico(
    resultados: &mut Vec<DashGastoPorCategoria>,
    nome: String,
    valores: &HashMap<String, f64>,
) {
    if valores.len() > 1 {
        criar_novo_grafico(resultados, nome, valores);
    }
}

fn criar_novo_grafico(
    resultados: &mut Vec<DashGastoPorCategoria>,
    nome: String,
    valores: &HashMap<String, f64>,
) {
    let mut item = DashGastoPorCategoria::new(nome.as_str());
    grafico_preencher_valores_por_categira(valores, &mut item);
    resultados.push(item);
}

fn grafico_preencher_valores_por_categira(
    valores: &HashMap<String, f64>,
    item: &mut DashGastoPorCategoria,
) {
    for (cat, valor) in valores {
        item.add(cat.as_str(), valor.clone());
    }
}

fn agrupar_lancamentos_em_grupos(
    mapa: &mut HashMap<String, HashMap<String, f64>>,
    lancamentos: Vec<Lancamento>,
) {
    lancamentos
        .iter()
        .filter(|l| l.valor < 0.0)
        .for_each(|lancamento| {
            agrupar_lancamento_em_grupos(mapa, lancamento);
        });
}

fn agrupar_lancamento_em_grupos(
    mapa: &mut HashMap<String, HashMap<String, f64>>,
    lancamento: &Lancamento,
) {
    if let Some(cat) = lancamento.categoria.some() {
        preencher_grupos(mapa, cat.clone(), lancamento.valor);
    }
}

fn preencher_grupos(
    mapa: &mut HashMap<String, HashMap<String, f64>>,
    categoria: Categoria,
    valor: f64,
) {
    let agrupamento = gerar_agrupamento(categoria);
    let mut grupo_pai = String::new();
    let mut grupo = "SaídasS >> {}", nomes".to_string();

    for i in 0..agrupamento.len() {
        grupo_somar_lancamento(mapa, format!("{}{}",grupo_pai.clone(), grupo.clone()) , agrupamento[i].clone(), valor);
        grupo_pai = format!("{} >> ", grupo.clone());
        grupo = agrupamento[i].clone();
    }
}

fn gerar_agrupamento(cat: Categoria) -> Vec<String> {
    let cat_str = cat.to_string();
    let agrupamento = cat_str
        .split(";")
        .map(|s| s.trim().to_string())
        .collect::<Vec<_>>();
    agrupamento
}

fn grupo_somar_lancamento(
    mapa: &mut HashMap<String, HashMap<String, f64>>,
    grupo: String,
    categoria: String,
    valor: f64,
) {
    mapa.entry(grupo)
        .or_insert_with(HashMap::new)
        .entry(categoria)
        .and_modify(|v| *v -= valor)
        .or_insert(valor * -1.0);
}
