use chrono::NaiveDate;

use super::CSV;

#[derive(Clone)]
pub struct ParcelaDivida {
    pub num_parcela: i32,
    pub valor: f64,
    pub valor_pago: f64,
    pub pago: bool,
    pub data_vencimento: NaiveDate,
}

impl CSV for ParcelaDivida {
    fn from_csv(value: String) -> Self {
        let values: Vec<String> = value.split('|').map(String::from).collect();
        ParcelaDivida::from_csv_vec(values)
    }

    fn from_csv_vec(value: Vec<String>) -> Self {
        ParcelaDivida {
            num_parcela: value[0].parse().unwrap(),
            valor: value[1].parse().unwrap(),
            valor_pago: value[2].parse().unwrap(),
            pago: value[3].eq("true"),
            data_vencimento: NaiveDate::parse_from_str(&value[4], "%Y-%m-%d").unwrap(),
        }
    }

    fn to_csv(&self) -> String {
        let mut resp: Vec<String> = Vec::new();

        resp.push(self.num_parcela.to_string());
        resp.push(self.valor.to_string());
        resp.push(self.valor_pago.to_string());
        resp.push(if self.pago { "true".to_string() }else{ "false".to_string() });
        resp.push(self.data_vencimento.format("%Y-%m-%d").to_string());

        resp.join("|")
    }
}
