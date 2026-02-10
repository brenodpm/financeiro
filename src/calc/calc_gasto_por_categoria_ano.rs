use std::collections::HashMap;

use chrono::{Local, Months};

use crate::{
    calc::calc_lancamentos_filtros::Lancamentos,
    dto::{Categoria, DashGastoPorCategoriaAno, Lancamento, OptionalLazyFn},
};

pub fn calcular_gasto_por_categoria_ano(
    mut lancamentos: Vec<Lancamento>,
) -> Vec<DashGastoPorCategoriaAno> {
    lancamentos = lancamentos.ultimos_dias(30);
    let mut mapa: HashMap<String, HashMap<String, HashMap<String, f64>>> = HashMap::new();

    agrupar_lancamentos_em_grupos(&mut mapa, lancamentos);

    let meses_base = gerar_base_meses();

    let mut resultados: Vec<DashGastoPorCategoriaAno> = Vec::new();
    for (nome, valores) in mapa {
        if valores.len() > 1 {
            let mut item = DashGastoPorCategoriaAno::new(nome.as_str());
            for (cat, meses) in valores {
                item.add(cat.clone(), meses_base.clone(), meses);
            }
            resultados.push(item);
        }
    }
    resultados
}

fn gerar_base_meses() -> Vec<String> {
    let mut data = Local::now().date_naive();
    let mut meses_base: Vec<String> = Vec::new();
    for mes in 12..0 {
        meses_base.push(
            data.checked_sub_months(Months::new(mes))
                .unwrap()
                .format("%m/%Y")
                .to_string(),
        );
    }
    meses_base
}

fn agrupar_lancamentos_em_grupos(
    mapa: &mut HashMap<String, HashMap<String, HashMap<String, f64>>>,
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
    mapa: &mut HashMap<String, HashMap<String, HashMap<String, f64>>>,
    lancamento: &Lancamento,
) {
    if let Some(cat) = lancamento.categoria.some() {
        preencher_grupos(
            mapa,
            cat,
            lancamento.data.format("%m/%Y").to_string(),
            lancamento.valor,
        );
    }
}

fn preencher_grupos(
    mapa: &mut HashMap<String, HashMap<String, HashMap<String, f64>>>,
    cat: Categoria,
    mes: String,
    valor: f64,
) {
    let agrupamento = gerar_agrupamento(cat);
    let mut grupo = "Tipo".to_string();

    for i in 0..agrupamento.len() {
        grupo_somar_lancamento(
            mapa,
            grupo.clone(),
            agrupamento[i].to_string(),
            mes.clone(),
            valor,
        );
        grupo = agrupamento[i].to_string();
    }
}

fn gerar_agrupamento(cat: Categoria) -> Vec<String> {
    let cat_str = cat.to_string();
    let agrupamento = cat_str
        .split(";")
        .map(|s| s.to_string())
        .collect::<Vec<_>>();
    agrupamento
}

fn grupo_somar_lancamento(
    mapa: &mut HashMap<String, HashMap<String, HashMap<String, f64>>>,
    grupo: String,
    categoria: String,
    mes: String,
    valor: f64,
) {
    mapa.entry(grupo)
        .or_insert_with(HashMap::new)
        .entry(categoria)
        .or_insert_with(HashMap::new)
        .entry(mes)
        .and_modify(|v| *v -= valor)
        .or_insert(valor * -1.0);
}
