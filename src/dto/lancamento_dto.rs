use chrono::NaiveDate;

use super::{gerar_sha1, Categoria, OptionalLazy, OptionalLazyFn, Regra, Unico, CSV};

#[derive(Debug, Clone, Default)]
pub struct Lancamento {
    pub id: String,
    pub descricao: String,
    pub valor: f64,
    pub data: NaiveDate,
    pub categoria: OptionalLazy<Categoria>,
    pub conta: Option<String>,
    pub regra: OptionalLazy<Regra>,
}

impl CSV for Lancamento {
    fn from_csv(value: String) -> Self {
        let values: Vec<String> = value.split(';').map(String::from).collect();
        Lancamento::from_csv_vec(values)
    }

    fn from_csv_vec(value: Vec<String>) -> Self {
        Lancamento {
            id: value[0].clone(),
            descricao: value[1].clone(),
            valor: value[2].parse().unwrap(),
            data: NaiveDate::parse_from_str(&value[3], "%Y-%m-%d").unwrap(),
            categoria: OptionalLazy::Id(value[4].clone()),
            conta: Some(value[5].clone()),
            regra: OptionalLazy::Id(value[6].clone()),
        }
    }

    fn to_csv(&self) -> String {
        let mut resp: Vec<String> = Vec::new();

        resp.push(self.id.clone());
        resp.push(self.descricao.clone());
        resp.push(self.valor.to_string());
        resp.push(self.data.format("%Y-%m-%d").to_string());
        resp.push(self.categoria.id());
        resp.push(match self.conta.clone() {
            Some(c) => c,
            None => String::new(),
        });
        resp.push(self.regra.id());

        resp.join(";")
    }
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
