use std::collections::HashMap;

use crate::{
    calc::calc_lancamentos_filtros::Lancamentos,
    dto::{DashResumo, Lancamento, OptionalLazyFn, TipoDespesa, TipoFluxo},
};
use chrono::Datelike;

pub fn calcular_resumo(lancamentos: Vec<Lancamento>) -> DashResumo {
    let grupos = lancamentos.serparar_por_mes();

    let mut ano = chrono::Local::now().date_naive().year();
    let mut mes = chrono::Local::now().date_naive().month();

    let atual_entrada = total_por(&grupos, ano, mes, TipoTotal::Entrada);
    let atual_saida = total_por(&grupos, ano, mes, TipoTotal::Saida);

    let mut entradas = 0.0;
    let mut saidas = 0.0;
    let mut custo = 0.0;
    let mut count = 0;

    for _ in 1..3 {
        if mes > 1 {
            mes -= 1;
        } else {
            mes = 12;
            ano -= 1;
        }
        entradas += total_por(&grupos, ano, mes, TipoTotal::Entrada);
        saidas += total_por(&grupos, ano, mes, TipoTotal::Saida);
        custo += total_por(&grupos, ano, mes, TipoTotal::Custo);
        count += 1;
    }

    DashResumo {
        media_entradas: entradas / count as f64,
        media_saidas: (saidas / count as f64) * -1.0,
        media_custo_vida: (custo / count as f64) * -1.0,
        atual_entradas: atual_entrada,
        atual_saidas: (atual_saida) * -1.0,
    }
}

enum TipoTotal {
    Entrada,
    Saida,
    Custo,
}

fn total_por(
    grupos: &HashMap<i32, HashMap<u32, Vec<Lancamento>>>,
    ano: i32,
    mes: u32,
    tipo: TipoTotal,
) -> f64 {
    let atual_entrada = grupos
        .get(&ano)
        .and_then(|m| m.get(&mes))
        .map(|lancamentos| {
            lancamentos
                .iter()
                .filter(|l| match tipo {
                    TipoTotal::Entrada => l.valor > 0.0,
                    TipoTotal::Saida => l.valor < 0.0,
                    TipoTotal::Custo => e_custo(l.clone().clone()),
                })
                .map(|l| l.valor)
                .sum()
        })
        .unwrap_or(0.0);
    atual_entrada
}

fn e_custo(l: Lancamento) -> bool {
    let mut resp = false;
    if let Some(c) = l.categoria.some() {
        if let TipoFluxo::Despesa(g) = c.tipo {
            if g.tipo == TipoDespesa::Fixa {
                resp = true;
            }
        }
    }

    resp
}
