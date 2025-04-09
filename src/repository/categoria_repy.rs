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
        let mut resp: Vec<Categoria> = serde_json::from_str(&json).unwrap();

        resp = resp
            .into_iter()
            .sorted_by(|a, b| a.to_string().partial_cmp(&b.to_string()).unwrap())
            .collect();

        if resp.len() == 0usize {
            resp = Categoria::lista_padrao();
            Categoria::salvar(&mut resp);
        }

        resp
    }

    pub fn salvar(categorias: &mut Vec<Categoria>) {
        categorias.into_iter().for_each(|c| {
            if c.id.is_empty() {
                c.gerar_id();
            }
        });

        arq_escrever(FIN, CAT, serde_json::to_string(&categorias).unwrap());
    }
}
