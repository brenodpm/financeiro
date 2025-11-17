use std::collections::HashMap;

use crate::{
    calc::calc_lancamentos_filtros::Lancamentos,
    dto::{DashGastoPorConta, Lancamento, OptionalLazyFn, TipoFluxo},
};

pub fn calcular_gasto_por_conta_d30(lancamentos: Vec<Lancamento>) -> Vec<DashGastoPorConta> {
    let mut resp: HashMap<String, f64> = HashMap::new();

    filtrar_e_somar_despesa_por_conta(&mut resp, lancamentos);

    let mut list: Vec<DashGastoPorConta> = resp
        .iter()
        .map(|h| DashGastoPorConta {
            conta: h.0.clone(),
            valor: h.1.clone(),
        })
        .collect();
    list.sort_by(|a, b| {
        b.valor
            .partial_cmp(&a.valor)
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    list
}

fn filtrar_e_somar_despesa_por_conta(
    resp: &mut HashMap<String, f64>,
    lancamentos: Vec<Lancamento>,
) {
    for l in lancamentos.ultimos_dias(30) {
        somar_gasto_por_conta(resp, l);
    }
}

fn somar_gasto_por_conta(resp: &mut HashMap<String, f64>, l: Lancamento) {
    if is_despesa(&l) {
        somar_valor_por_conta(resp, l);
    }
}

fn somar_valor_por_conta(resp: &mut HashMap<String, f64>, l: Lancamento) {
    if let Some(conta) = l.conta.as_ref() {
        if let Some(total) = resp.get_mut(conta) {
            *total += l.valor.abs();
        } else {
            resp.insert(conta.clone(), l.valor.abs());
        }
    }
}

fn is_despesa(l: &Lancamento) -> bool {
    l.valor < 0.0
        && if let Some(c) = l.categoria.some() {
            if let TipoFluxo::Despesa(_) = c.tipo {
                true
            } else {
                false
            }
        } else {
            false
        }
}
