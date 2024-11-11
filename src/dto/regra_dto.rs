use super::{fluxo_regra_dto::FluxoRegra, CSV};

#[derive(Debug)]
pub struct Regra {
    pub fluxo: FluxoRegra,
    pub regex: String,
    pub categoria: String,
}

impl CSV for Regra {
    fn from_csv(value: String) -> Self {
        let values: Vec<String> = value.split(';').map(String::from).collect();
        Regra::from_csv_vec(values)
    }

    fn from_csv_vec(value: Vec<String>) -> Self {
        Regra {
            regex: value[0].clone(),
            fluxo: FluxoRegra::from_string(value[1].clone()),
            categoria: value[2].clone(),
        }
    }

    fn to_csv(&self) -> String {
        let mut resp: Vec<String> = Vec::new();

        resp.push(self.regex.to_lowercase());
        resp.push(self.fluxo.to_string());
        resp.push(self.categoria.clone());

        resp.join(";")
    }
}
