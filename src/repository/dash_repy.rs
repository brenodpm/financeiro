use include_dir::{include_dir, Dir};

use crate::dto::{DashDivida, DashGastoPorConta, DashResumo};

use super::file_repy::{arq_deletar_dir, arq_escrever};

static PROJECT_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/dashfiles");

const FIN: &str = "Financeiro/";

impl DashResumo {
    pub fn salvar(resumo: DashResumo) {
        escrever("resumo", serde_json::to_string_pretty(&resumo).unwrap());
    }
}

impl DashGastoPorConta {
    pub fn salvar(gastos: Vec<DashGastoPorConta>) {
        escrever(
            "gasto_por_conta",
            serde_json::to_string_pretty(&gastos).unwrap(),
        );
    }
}

impl DashDivida {
    pub fn salvar(dividas: Vec<DashDivida>) {
        escrever("dividas", serde_json::to_string_pretty(&dividas).unwrap());
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
            arq_escrever(
                &(FIN.to_string() + destino),
                file.path().file_name().unwrap().to_str().unwrap(),
                content.to_string(),
            );
        }
    }

    for dir in dir.dirs() {
        transferir_diretorio(dir, dir.path().to_str().unwrap());
    }
}
