use std::collections::HashMap;

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct DashGastoPorCategoriaAno {
    pub grupo: String,
    pub valores: Vec<DashGastoPorCategoriaAnoValores>,
}

impl DashGastoPorCategoriaAno {
    pub fn new(grupo: &str) -> Self {
        Self {
            grupo: grupo.to_string(),
            valores: Vec::new(),
        }
    }

    pub fn add(&mut self, cat: String, base: &Vec<String>, meses: HashMap<String, f64>) {
        if let Some(item) = self.valores.iter_mut().find(|v| v.categoria == cat) {
            item.somar(meses);
        } else {
            let mut item = DashGastoPorCategoriaAnoValores::new(cat, base.clone());
            item.somar(meses);
            self.valores.push(item);
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct DashGastoPorCategoriaAnoValores {
    pub categoria: String,
    #[serde(flatten)]
    pub meses: HashMap<String, f64>,
}

impl DashGastoPorCategoriaAnoValores {
    pub fn new(categoria: String, base: Vec<String>) -> Self {
        let meses = base.into_iter().map(|m| (m, 0.0)).collect();
        Self { categoria, meses }
    }

    pub fn somar(&mut self, meses: HashMap<String, f64>) {
        meses.iter().for_each(|(k, v)| {
            if let Some(value) = self.meses.get_mut(k) {
                *value += v;
            }
        });
    }
}
