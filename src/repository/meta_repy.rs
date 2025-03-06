use crate::dto::{Meta, CSV};

use super::file_repy::{arq_escrever_linhas, arq_ler};

const FIN: &str = ".financeiro";
const METAS: &str = "metas.csv";

impl Meta{
    pub fn listar() -> Vec<Meta> {
        arq_ler(FIN, METAS).map(Meta::from_csv).collect()
    }

    pub fn salvar(&self) {
        let mut lista = Meta::listar();

        if let Some(i) = lista.iter().position(|a| a.id == self.id) {
            lista[i] = self.clone();
        } else {
            lista.push(self.clone());
        }

        arq_escrever_linhas(FIN, METAS, &lista.into_iter().map(|i| i.to_csv()).collect())
    }

    pub fn deletar(&self) {
        let mut lista = Meta::listar();

        if let Some(pos) = lista.iter().position(|a| a.id == self.id) {
            lista.remove(pos);
        }

        arq_escrever_linhas(FIN, METAS, &lista.into_iter().map(|i| i.to_csv()).collect::<Vec<_>>());
    }
}