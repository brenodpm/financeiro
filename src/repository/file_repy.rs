use std::{
    fs::{create_dir_all, remove_dir_all, write, File},
    io::{BufRead, BufReader, Lines, Read},
    iter::Flatten,
    path::{Path, PathBuf},
};

use chardet::detect;
use encoding_rs::Encoding;

use crate::get_home_dir;

pub fn arq_externo_ler(arquivo: &str) -> Vec<String> {
    let mut file = File::open(arquivo).expect("Failed to open file");
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).expect("Failed to read file");

    let (encoding_name, _, _) = detect(&buffer);
    let encoding = Encoding::for_label(encoding_name.as_bytes()).expect("Failed to get encoding");

    let (cow, _, _) = encoding.decode(&buffer);
    cow.to_string()
        .lines()
        .map(|s| s.trim().replace(";", ","))
        .collect()
}

pub fn arq_ler(dir: &str, file: &str) -> Flatten<Lines<BufReader<File>>> {
    let mut path = get_home_dir();
    path.push(&dir);
    checar_dir(&path);

    path.push(&file);
    checar_arq(&path);

    BufReader::new(File::open(&path).unwrap()).lines().flatten()
}

pub fn arq_escrever(dir: &str, file: &str, texto: String) {
    let mut path = get_home_dir();
    path.push(&dir);
    checar_dir(&path);

    path.push(&file);
    checar_arq(&path);

    write(path, texto).expect("Falha ao escrever no arquivo");
}

pub fn arq_deletar_dir(dir: &str) {
    let mut path = get_home_dir();
    path.push(&dir);

    if Path::new(&path).exists() {
        match remove_dir_all(&path) {
            Ok(_) => {}
            Err(erro) => log::error!("Erro ao deletar diretório: {}", erro),
        }
    }
}

fn checar_dir(path: &PathBuf) {
    if !Path::new(&path).exists() {
        create_dir_all(&path).expect("Falha ao criar diretorio");
    }
}

fn checar_arq(path: &PathBuf) {
    if !Path::new(&path).exists() {
        write(&path, "").expect("Falha ao criar arquivo");
    }
}
