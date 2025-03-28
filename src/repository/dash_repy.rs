use include_dir::{include_dir, Dir};

use super::file_repy::{arq_deletar_dir, arq_escrever};

static PROJECT_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/dashfiles");

const FIN: &str = "Financeiro/";

pub fn atualizar_base(){
    arq_deletar_dir(FIN);
    transferir_diretorio(&PROJECT_DIR, "");
}

fn transferir_diretorio(dir: &Dir, destino: &str){
    for file in dir.files() {
        if let Some(content) = file.contents_utf8() {
            arq_escrever(&(FIN.to_string() + destino), file.path().file_name().unwrap().to_str().unwrap(), content.to_string());
        }
    }

    for dir in dir.dirs(){
        transferir_diretorio(dir, dir.path().to_str().unwrap());
    }
}