use std::fs::read_dir;

use chrono::Local;
use log::LevelFilter;
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Config, Root};
use log4rs::encode::pattern::PatternEncoder;

use crate::get_home_dir;

pub fn config() {
    let mut path = diretorio_base();
    path.push(Local::now().format("%Y-%m-%d.log").to_string());

    let logfile = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            format!("{{d(%H:%M:%S%.3f)}} - {{l}} - {} - {{m}}\n", env!("CARGO_PKG_VERSION")).as_str(),
        )))
        .build(path)
        .unwrap();

    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .build(Root::builder().appender("logfile").build(LevelFilter::Info))
        .unwrap();

    log4rs::init_config(config).unwrap();

    remover_apos_30_dias(diretorio_base());
}

fn diretorio_base() -> std::path::PathBuf {
    let mut path = get_home_dir();
    path.push(".financeiro/log/");
    path
}

fn remover_apos_30_dias(path: std::path::PathBuf) {
    read_dir(path)
        .unwrap()
        .filter_map(|entry| entry.ok())
        .for_each(|entry| {
            let metadata = entry.metadata().unwrap();
            let modified = metadata.modified().unwrap();
            let modified_date: chrono::DateTime<Local> = modified.into();
            if Local::now().signed_duration_since(modified_date).num_days() > 30 {
                std::fs::remove_file(entry.path()).unwrap();
            }
        });
}
