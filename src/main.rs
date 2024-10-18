mod app;
mod dto;
mod repository;
mod widget;
mod config_log;

use app::App;
use color_eyre::eyre::Result;
use dto::Lancamento;
use homedir::my_home;
use std::fs::create_dir_all;

fn preparar_diretorios() {
    let home = my_home().unwrap().unwrap();
    for path in ["Downloads/importar", "Downloads/importado", "financeiro"] {
        let mut importar = home.clone();
        importar.push(path);
        create_dir_all(importar.clone()).expect("Falha ao criar pasta");
    }
}

fn main() {
    config_log::config();
    log::info!("Início");

    preparar_diretorios();
    Lancamento::categorizar(Lancamento::from_ofx());
    start_tui().expect("msg");
    
    log::info!("Finalizado");
}

fn start_tui() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let app_result = App::default().run(terminal);
    ratatui::restore();
    app_result
}
