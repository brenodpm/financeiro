mod categoria;
mod categorizador;
mod file;
mod lancamento;
mod ofx;
mod regra;

use std::fs::create_dir_all;

use homedir::my_home;
use lancamento::Lancamento;

fn preparar_diretorios() {
    let home = my_home().unwrap().unwrap();
    for path in ["Downloads/importar", "Downloads/importado", "financeiro"] {
        let mut importar = home.clone();
        importar.push(path);
        create_dir_all(importar.clone()).expect("Falha ao criar pasta");
    }
}

fn main() {
    preparar_diretorios();
    Lancamento::categorizar(Lancamento::from_ofx());
}
