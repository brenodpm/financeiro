use crate::dto::DashGastoPor;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct DashGastoPorCategoria {
    pub grupo: String,
    pub valores: Vec<DashGastoPor>,
}

impl DashGastoPorCategoria {
    pub fn new(grupo: &str) -> Self {
        Self {
            grupo: grupo.to_string(),
            valores: Vec::new(),
        }
    }
    pub fn add(&mut self, nome: &str, valor: f64) {
        self.valores.push(DashGastoPor {
            nome: nome.to_string(),
            valor,
        } );
    }
}