use serde_json;

use crate::{
    dto::CsvLayout,
    repository::file_repy::{arq_escrever, arq_ler},
};

const DIR: &str = ".financeiro";
const FILE: &str = "csv_layouts.json";

pub fn listar() -> Vec<CsvLayout> {
    let json: String = arq_ler(DIR, FILE).collect();
    if json.is_empty() {
        return vec![];
    }
    serde_json::from_str(&json).unwrap_or_else(|e| {
        log::error!("Erro ao ler csv_layouts: {e}");
        vec![]
    })
}

pub fn salvar_lista(layouts: &Vec<CsvLayout>) {
    match serde_json::to_string_pretty(layouts) {
        Ok(json) => arq_escrever(DIR, FILE, json),
        Err(e) => log::error!("Erro ao salvar csv_layouts: {e}"),
    }
}

pub fn salvar(layout: CsvLayout) {
    let mut lista = listar();
    if let Some(pos) = lista.iter().position(|l| l.id == layout.id) {
        lista[pos] = layout;
    } else {
        lista.push(layout);
    }
    salvar_lista(&lista);
}
