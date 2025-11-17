use std::collections::HashMap;

use crate::dto::Lancamento;
use chrono::Datelike;

pub trait Lancamentos {
    fn serparar_por_mes(&self) -> HashMap<i32, HashMap<u32, Vec<Lancamento>>>;
    fn ultimos_dias(&self, dias: i64) -> Vec<Lancamento>;
}

impl Lancamentos for Vec<Lancamento> {
    fn serparar_por_mes(&self) -> HashMap<i32, HashMap<u32, Vec<Lancamento>>> {
        let mut mapa: HashMap<i32, HashMap<u32, Vec<Lancamento>>> = HashMap::new();

        self.into_iter().for_each(|lancamento| {
            let ano = lancamento.data.year();
            let mes = lancamento.data.month();

            mapa.entry(ano)
                .or_insert_with(HashMap::new)
                .entry(mes)
                .or_insert_with(Vec::new)
                .push(lancamento.clone());
        });

        mapa
    }

    fn ultimos_dias(&self, dias: i64) -> Vec<Lancamento> {
        let hoje = chrono::Local::now().date_naive();
        let limite = hoje
            .checked_sub_signed(chrono::Duration::days(dias))
            .unwrap();

        self.clone()
            .into_iter()
            .filter(|l| l.data >= limite)
            .collect()
    }
}
