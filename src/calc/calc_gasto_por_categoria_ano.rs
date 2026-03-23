use std::collections::HashMap;

use chrono::{Local, Months};

use crate::{
    calc::calc_lancamentos_filtros::Lancamentos,
    dto::{Categoria, DashGastoPorCategoriaAno, Lancamento, OptionalLazyFn},
};

pub fn calcular_gasto_por_categoria_ano(
    ordem: &Vec<String>,
    mut lancamentos: Vec<Lancamento>,
) -> Vec<DashGastoPorCategoriaAno> {
    lancamentos = lancamentos.ultimos_dias(365);
    let mut mapa: HashMap<String, HashMap<String, HashMap<String, f64>>> = HashMap::new();

    agrupar_lancamentos(&mut mapa, lancamentos);

    let meses_base = gerar_base_meses();

    gerar_graficos(&meses_base, ordem, mapa)
}

fn gerar_graficos(
    base: &Vec<String>,
    ordem: &Vec<String>,
    mapa: HashMap<String, HashMap<String, HashMap<String, f64>>>,
) -> Vec<DashGastoPorCategoriaAno> {
    let mut resultados: Vec<DashGastoPorCategoriaAno> = Vec::new();

    for nome in ordem {
        if let Some(valores) = mapa.get(nome) {
            se_mais_de_1_item_criar_grafico(base, &mut resultados, nome, valores);
        }
    }

    resultados
}

fn se_mais_de_1_item_criar_grafico(
    base: &Vec<String>,
    resultados: &mut Vec<DashGastoPorCategoriaAno>,
    nome: &String,
    valores: &HashMap<String, HashMap<String, f64>>,
) {
    if valores.len() > 1 {
        criar_novo_grafico(base, resultados, nome, valores);
    }
}

fn criar_novo_grafico(
    base: &Vec<String>,
    resultados: &mut Vec<DashGastoPorCategoriaAno>,
    nome: &String,
    valores: &HashMap<String, HashMap<String, f64>>,
) {
    let mut item = DashGastoPorCategoriaAno::new(nome.as_str());
    grafico_preencher_valores_por_categira(base, valores, &mut item);
    item.valores
        .iter_mut()
        .for_each(|v| v.meses.retain(|_, val| *val != 0.0));
    item.valores.retain(|v| !v.meses.is_empty());
    if !item.valores.is_empty() {
        resultados.push(item);
    }
}

fn grafico_preencher_valores_por_categira(
    base: &Vec<String>,
    valores: &HashMap<String, HashMap<String, f64>>,
    item: &mut DashGastoPorCategoriaAno,
) {
    for (cat, valor) in valores {
        item.add(cat.clone(), base, valor.clone());
    }
}

fn gerar_base_meses() -> Vec<String> {
    let data = Local::now().date_naive();
    let mut meses_base: Vec<String> = Vec::new();
    for mes in 0..12 {
        meses_base.push(
            data.checked_sub_months(Months::new(mes))
                .unwrap()
                .format("%m/%Y")
                .to_string(),
        );
    }
    meses_base
}

fn agrupar_lancamentos(
    mapa: &mut HashMap<String, HashMap<String, HashMap<String, f64>>>,
    lancamentos: Vec<Lancamento>,
) {
    lancamentos
        .iter()
        .filter(|l| l.valor < 0.0)
        .for_each(|lancamento| {
            agrupar_lancamento(mapa, lancamento);
        });
}

fn agrupar_lancamento(
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
    let mut grupo_pai = String::new();
    let mut grupo = "Saídas".to_string();

    for i in 0..agrupamento.len() {
        let nome_grupo = format!("{}{}", grupo_pai, grupo);
        grupo_somar_lancamento(
            mapa,
            nome_grupo.clone(),
            agrupamento[i].to_string(),
            mes.clone(),
            valor,
        );

        grupo_pai = format!("{} >> ", grupo);
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
    mapa: &mut HashMap<String, HashMap<String, HashMap<String, f64>>>,
    nome_grupo: String,
    categoria: String,
    mes: String,
    valor: f64,
) {
    mapa.entry(nome_grupo)
        .or_insert_with(HashMap::new)
        .entry(categoria)
        .or_insert_with(HashMap::new)
        .entry(mes)
        .and_modify(|v| *v -= valor)
        .or_insert(valor * -1.0);
}
