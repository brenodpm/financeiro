use serde::{Deserialize, Serialize};

use super::{gerar_sha1, Unico};

#[derive(PartialEq, Default, Clone, Serialize, Deserialize)]
pub struct Meta {
    pub id: String,
    pub nome: String,
    pub desc: String,
    pub tipo_meta: String,
    pub filtro: String,
    pub metrica: String,
    pub fluxo: String,
    pub periodo: String,
    pub valor: f64,
}

impl Unico for Meta {
    fn gerar_id(&mut self) {
        let mut itens: Vec<String> = Vec::new();

        itens.push(self.nome.clone());

        self.id = gerar_sha1(itens.join("-"));
    }
}
