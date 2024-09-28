use chrono::NaiveDate;

use super::Categoria;

#[derive(Debug, Clone, Default)]
pub struct Lancamento {
    pub id: String,
    pub descricao: String,
    pub valor: f64,
    pub data: NaiveDate,
    pub categoria: Option<Categoria>,
}