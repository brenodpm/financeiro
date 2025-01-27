use include_dir::{include_dir, Dir};

use super::file_repy::arq_escrever;

static PROJECT_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/dashfiles");

const FIN: &str = "Financeiro";

pub fn atualizar_base(){
    for file in PROJECT_DIR.files() {
        if let Some(content) = file.contents_utf8() {
            arq_escrever(FIN, file.path().file_name().unwrap().to_str().unwrap(), content.to_string());
        }
    }
}