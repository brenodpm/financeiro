use chrono::NaiveDate;

use super::{gerar_sha1, Categoria, DtoIdentificado};

#[derive(Debug, Clone, Default)]
pub struct Lancamento {
    pub id: String,
    pub descricao: String,
    pub valor: f64,
    pub data: NaiveDate,
    pub categoria: Option<Categoria>,
}

impl Lancamento {
    fn new(descricao: String, valor: f64, data: NaiveDate) -> Lancamento {
        let mut resp = Self {
            id: String::new(),
            descricao: descricao,
            valor: valor,
            data: data,
            categoria: None,
        };
        resp.gerar_id();
        resp
    }

    

    pub fn to_string(&self) -> String {
        let mut resp: Vec<String> = Vec::new();

        resp.push(self.id.clone());
        resp.push(self.descricao.clone());
        resp.push(self.valor.to_string());
        resp.push(self.data.format("%Y-%m-%d").to_string());

        resp.join(";")
    }
}

impl DtoIdentificado for Lancamento {
    fn gerar_id(&mut self){
        let mut itens: Vec<String> = Vec::new();

        itens.push(self.descricao.clone());
        itens.push(self.valor.to_string());
        itens.push(self.data.format("%Y%m%d").to_string());

        self.id = gerar_sha1(itens.join("-"));
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