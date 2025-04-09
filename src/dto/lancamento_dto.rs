use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use super::{gerar_sha1, Categoria, OptionalLazy, Regra, Unico};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Lancamento {
    pub id: String,
    pub descricao: String,
    pub valor: f64,
    pub data: NaiveDate,
    pub categoria: OptionalLazy<Categoria>,
    pub conta: Option<String>,
    pub regra: OptionalLazy<Regra>,
}

impl Unico for Lancamento {
    fn gerar_id(&mut self) {
        let mut itens: Vec<String> = Vec::new();

        itens.push(self.descricao.clone());
        itens.push(self.valor.to_string());
        itens.push(self.data.format("%Y%m%d").to_string());

        self.id = gerar_sha1(itens.join("-"));
    }
}
