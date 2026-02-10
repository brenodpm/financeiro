use std::collections::HashMap;

const MONTHS: [&str; 12] = [
    "Jan", "Fev", "Mar", "Abr", "Mai", "Jun", "Jul", "Ago", "Set", "Out", "Nov", "Dez",
];

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct DashGastoPorCategoriaAno {
    pub grupo: String,
    pub valores: HashMap<String, DashGastoPorCategoriaAnoValores>,
}

impl DashGastoPorCategoriaAno {
    pub fn new(grupo: &str) -> Self {
        Self {
            grupo: grupo.to_string(),
            valores: HashMap::new(),
        }
    }

    pub fn add(&mut self, cat: String, base: Vec<String>, meses: HashMap<String, f64>) {
        if !self.valores.contains_key(&cat) {
            self.valores
                .insert(cat.clone(), DashGastoPorCategoriaAnoValores::new(base));
        };
        if let Some(valores) = self.valores.get_mut(&cat) {
            valores.somar(meses);
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct DashGastoPorCategoriaAnoValores {
    #[serde(flatten)]
    pub meses: HashMap<String, f64>,
}

impl DashGastoPorCategoriaAnoValores {
    pub fn new(base: Vec<String>) -> Self {
        let mut meses: HashMap<String, f64> = HashMap::new();

        for i in 0..base.len() {
            meses.insert(base[i].clone(), 0.0);
        }

        Self { meses: meses }
    }

    pub fn somar(&mut self, meses: HashMap<String, f64>) {
        meses.iter().for_each(|m| {
            if let Some(value) = self.meses.get_mut(m.0) {
                *value += m.1;
            }
        });
    }
}
