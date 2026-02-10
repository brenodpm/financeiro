use std::vec;

use itertools::Itertools;

use crate::dto::{Categoria, Unico};

use super::file_repy::{arq_escrever, arq_ler};

const FIN: &str = ".financeiro";
const CAT: &str = "categorias.json";

impl Categoria {
    pub fn listar() -> Vec<Categoria> {
        let mut json: String = arq_ler(FIN, CAT).collect();
        if json.is_empty() {
            json = "[]".to_string();
        }
        let mut resp: Vec<Categoria> = match serde_json::from_str(&json) {
            Ok(vec) => vec,
            Err(error) => {
                log::error!("Erro ao desserializar categorias: {}", error);
                vec![]
            }
        };

        resp = resp
            .into_iter()
            .sorted_by(|a, b| a.to_string().partial_cmp(&b.to_string()).unwrap())
            .collect();

        if resp.len() == 0usize {
            resp = Categoria::lista_padrao();
            Categoria::salvar_lista(&mut resp);
        }

        resp
    }

    pub fn salvar(&self) {
        let mut lista = Categoria::listar();

        if let Some(i) = lista.iter().position(|a| a.id == self.id) {
            lista[i] = self.clone();
        } else {
            lista.push(self.clone());
        }

        match serde_json::to_string_pretty(&lista) {
            Ok(json) => arq_escrever(FIN, CAT, json),
            Err(erro) => log::error!("Erro ao serializar categorias: {}", erro),
        };
    }

    pub fn salvar_lista(categorias: &mut Vec<Categoria>) {
        categorias.into_iter().for_each(|c| {
            if c.id.is_empty() {
                c.gerar_id();
            }
        });

        match serde_json::to_string_pretty(&categorias) {
            Ok(json) => arq_escrever(FIN, CAT, json),
            Err(erro) => log::error!("Erro ao serializar categorias: {}", erro),
        };
    }

    pub fn deletar(&self) {
        let mut lista = Categoria::listar();

        if let Some(pos) = lista.iter().position(|a| a.id == self.id) {
            lista.remove(pos);
        }

        match serde_json::to_string_pretty(&lista) {
            Ok(json) => arq_escrever(FIN, CAT, json),
            Err(erro) => log::error!("Erro ao serializar categorias: {}", erro),
        };
    }
}