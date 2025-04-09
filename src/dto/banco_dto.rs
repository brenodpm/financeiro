use serde::{Deserialize, Serialize};

use super::Conta;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Banco {
    pub id: String,
    pub nome: String,
    pub contas: Vec<Conta>,
}

impl Banco {
    pub fn novo(id: String) -> Self {
        Self {
            id: id.clone(),
            nome: id,
            contas: Vec::new(),
        }
    }
}
