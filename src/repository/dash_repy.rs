use include_dir::{include_dir, Dir};

use crate::dto::{DashDivida, DashGastoPorConta, DashResumo};

use super::file_repy::{arq_deletar_dir, arq_escrever};

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

impl DashGastoPorConta {
    pub fn salvar(gastos: Vec<DashGastoPorConta>) {
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

pub fn atualizar_base() {
    arq_deletar_dir(FIN);
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
            match file.path().file_name() {
                Some(file) => arq_escrever(
                    &(FIN.to_string() + destino),
                    file.to_str().unwrap(),
                    content.to_string(),
                ),
                None => log::error!("Erro ao transferir o arquivo {}", file.path().display()),
            }
        }
    }

    for dir in dir.dirs() {
        match dir.path().to_str() {
            Some(destino) => transferir_diretorio(dir, destino),
            None => log::error!("Erro ao transferir o diretório {}", dir.path().display()),
        };
    }
}
