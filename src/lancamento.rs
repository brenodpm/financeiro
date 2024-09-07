use chrono::NaiveDate;

use crate::{
    categoria::Categoria,
    file::{arq_escrever, arq_ler},
};

const FIN: &str = "financeiro";
const NAO_CAT: &str = "nao-cat.csv";

#[derive(Debug, Clone)]
pub struct Lancamento {
    pub id: String,
    pub descricao: String,
    pub valor: f64,
    pub data: NaiveDate,
    pub categoria: Option<Categoria>,
}

impl Default for Lancamento {
    fn default() -> Self {
        Lancamento {
            id: String::new(),
            descricao: String::new(),
            valor: 0.0,
            data: NaiveDate::default(),
            categoria: None,
        }
    }
}

impl From<String> for Lancamento {
    #[inline]
    fn from(s: String) -> Lancamento {
        let attrs: Vec<String> = s.split(';').map(String::from).collect();
        Lancamento {
            id: attrs[0].clone(),
            descricao: attrs[1].clone(),
            valor: attrs[2].parse().unwrap(),
            data: NaiveDate::parse_from_str(&attrs[3], "%Y-%m-%d").unwrap(),
            categoria: None,
        }
    }
}

impl Lancamento {
    pub fn nao_categorizados_listar() -> Vec<Lancamento> {
        arq_ler(FIN, NAO_CAT).map(Lancamento::from).collect()
    }
    pub fn nao_categorizados_salvar(itens: &Vec<Lancamento>) {
        arq_escrever(
            FIN,
            NAO_CAT,
            &itens.into_iter().map(|i| i.to_string()).collect(),
        )
    }

    fn to_string(&self) -> String {
        let mut resp: Vec<String> = Vec::new();

        resp.push(self.id.clone());
        resp.push(self.descricao.clone());
        resp.push(self.valor.to_string());
        resp.push(self.data.format("%Y-%m-%d").to_string());

        resp.join(";")
    }
}
