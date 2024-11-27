use super::{fluxo_regra_dto::FluxoRegra, Categoria, Lazy, LazyFn, CSV};

#[derive(Debug)]
pub struct Regra {
    pub fluxo: FluxoRegra,
    pub regex: String,
    pub categoria: Lazy<Categoria>,
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
            categoria: Lazy::Id(value[2].clone()),
        }
    }

    fn to_csv(&self) -> String {
        let mut resp: Vec<String> = Vec::new();

        resp.push(self.regex.to_lowercase());
        resp.push(self.fluxo.to_string());
        resp.push(self.categoria.id());

        resp.join(";")
    }
}
