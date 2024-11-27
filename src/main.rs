mod app;
mod config_log;
mod dto;
mod repository;
mod widget;

use app::App;
use color_eyre::eyre::Result;
use dto::{Banco, Lancamento};
use std::{fs::create_dir_all, path::PathBuf, sync::Once};

static INIT: Once = Once::new();
static mut HOME_DIR: Option<PathBuf> = None;
pub fn init_home_dir() {
    unsafe {
        INIT.call_once(|| {
            HOME_DIR = Some(get_home_dir_path());
        });
    }
}

#[cfg(debug_assertions)]
fn get_home_dir_path() -> PathBuf {
    use std::env;

    let mut home = env::current_dir().unwrap();
    home.push("../baseTest");
    home
}

#[cfg(not(debug_assertions))]
fn get_home_dir_path() -> PathBuf {
    use homedir::my_home;

    PathBuf::from(my_home().unwrap().unwrap())
}

pub fn get_home_dir() -> PathBuf {
    unsafe { HOME_DIR.clone().expect("HOME_DIR not initialized") }
}

fn preparar_diretorios() {
    let home = get_home_dir();
    for path in ["Downloads/importar", "Downloads/importado", ".financeiro"] {
        let mut importar = home.clone();
        importar.push(path);
        create_dir_all(importar.clone())
            .unwrap_or_else(|e| log::error!("Falha ao criar diretório {path} - erro: {e:?}"))
    }
}

fn main() {
    init_home_dir();
    config_log::config();
    log::info!("Início");

    preparar_diretorios();
    importar();

    start_tui().unwrap_or_else(|e| log::error!("Falha ao executar o terminal: {e:?}"));
    log::info!("Finalizado");
}

fn importar() {
    let (lancamentos, bancos) = Lancamento::from_ofx();

    Banco::salvar(bancos);
    Lancamento::categorizar(&lancamentos);
}

fn start_tui() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let app_result = App::default().run(terminal);
    ratatui::restore();
    app_result
}
