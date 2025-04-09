use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct ParcelaDivida {
    pub num_parcela: i32,
    pub valor: f64,
    pub pago: bool,
    pub data_vencimento: NaiveDate,
}