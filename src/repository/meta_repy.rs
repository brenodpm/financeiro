use crate::dto::Meta;

use super::file_repy::{arq_escrever, arq_ler};

const FIN: &str = ".financeiro";
const METAS: &str = "metas.json";

impl Meta{
    pub fn listar() -> Vec<Meta> {
        let mut json: String = arq_ler(FIN, METAS).collect();
        if json.is_empty() {
            json = "[]".to_string();
        }
        serde_json::from_str(&json).unwrap()
    }

    pub fn salvar(&self) {
        let mut lista = Meta::listar();

        if let Some(i) = lista.iter().position(|a| a.id == self.id) {
            lista[i] = self.clone();
        } else {
            lista.push(self.clone());
        }

        arq_escrever(FIN, METAS, serde_json::to_string(&lista).unwrap());
    }

    pub fn deletar(&self) {
        let mut lista = Meta::listar();

        if let Some(pos) = lista.iter().position(|a| a.id == self.id) {
            lista.remove(pos);
        }

        arq_escrever(FIN, METAS, serde_json::to_string(&lista).unwrap());
    }
}