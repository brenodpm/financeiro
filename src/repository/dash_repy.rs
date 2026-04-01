use include_dir::{include_dir, Dir};
use std::fs;

use crate::dto::{
    DashDivida, DashGastoPor, DashGastoPorCategoria, DashGastoPorCategoriaAno, DashResumo, Orientacao,
};

use super::file_repy::arq_escrever;
use crate::get_home_dir;

static PROJECT_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/dashfiles");

const FIN: &str = "Financeiro/";

impl DashResumo {
    pub fn salvar(resumo: DashResumo) {
        match serde_json::to_string_pretty(&resumo) {
            Ok(json) => escrever("resumo", json),
            Err(erro) => log::error!("Erro ao salvar o resumo: {}", erro),
        };
    }
}

impl DashGastoPorCategoria {
    pub fn salvar(gastos: Vec<DashGastoPorCategoria>) {
        match serde_json::to_string_pretty(&gastos) {
            Ok(json) => escrever("gasto_por_categoria", json),
            Err(erro) => log::error!("Erro ao salvar os gastos por categoria: {}", erro),
        };
    }
}

impl DashGastoPorCategoriaAno {
    pub fn salvar(gastos: Vec<DashGastoPorCategoriaAno>) {
        match serde_json::to_string_pretty(&gastos) {
            Ok(json) => escrever("gasto_por_categoria_ano", json),
            Err(erro) => log::error!("Erro ao salvar os gastos por categoria anuais: {}", erro),
        };
    }
}

impl DashGastoPor {
    pub fn salvar(gastos: Vec<DashGastoPor>) {
        match serde_json::to_string_pretty(&gastos) {
            Ok(json) => escrever("gasto_por_conta", json),
            Err(erro) => log::error!("Erro ao salvar os gastos por conta: {}", erro),
        };
    }
}

impl DashDivida {
    pub fn salvar(dividas: Vec<DashDivida>) {
        match serde_json::to_string_pretty(&dividas) {
            Ok(json) => escrever("dividas", json),
            Err(erro) => log::error!("Erro ao salvar as dívidas: {}", erro),
        };
    }
}

impl Orientacao {
    pub fn salvar(orientacoes: &Vec<Orientacao>) {
        match serde_json::to_string_pretty(orientacoes) {
            Ok(json) => escrever("orientacoes", json),
            Err(erro) => log::error!("Erro ao salvar as orientações: {}", erro),
        };
    }
}

pub fn atualizar_base() {
    transferir_diretorio(&PROJECT_DIR, "");
}

fn escrever(nome: &str, conteudo: String) {
    let valor = format!("var {} = {};", nome, conteudo);
    arq_escrever(
        format!("{}/data", FIN).as_str(),
        format!("{}.js", nome).as_str(),
        valor,
    );
}

fn transferir_diretorio(dir: &Dir, destino: &str) {
    for file in dir.files() {
        if let Some(content) = file.contents_utf8() {
            if let Some(nome) = file.path().file_name() {
                let dir_destino = FIN.to_string() + destino;
                let mut path = get_home_dir();
                path.push(&dir_destino);
                path.push(nome);
                if fs::read_to_string(&path).unwrap_or_default() != content {
                    arq_escrever(&dir_destino, nome.to_str().unwrap(), content.to_string());
                }
            }
        }
    }

    for subdir in dir.dirs() {
        if let Some(d) = subdir.path().to_str() {
            transferir_diretorio(subdir, d);
        }
    }
}
