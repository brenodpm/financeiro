use std::{
    fs::{create_dir_all, write, File},
    io::{BufRead, BufReader, Lines, Read}, iter::Flatten, path::{Path, PathBuf},
};

use encoding_rs::WINDOWS_1252;
use encoding_rs_io::DecodeReaderBytesBuilder;
use homedir::my_home;

pub fn arq_ler_windows_1252(arquivo: &str) -> Vec<String> {
    let file = File::open(arquivo).expect("Falha ao abrir o arquivo");
    let mut rdr = DecodeReaderBytesBuilder::new()
        .encoding(Some(WINDOWS_1252))
        .build(file);

    let mut text = String::new();
    rdr.read_to_string(&mut text)
        .expect("Falha ao ler o arquivo");

    text.lines().map(|s| s.trim().replace(";", ",")).collect()
}

pub fn arq_ler(dir: &str, file: &str) -> Flatten<Lines<BufReader<File>>> {
    let mut path = my_home().unwrap().unwrap();
    path.push(&dir);
    path.push(&file);

    checar_dir(dir);
    checar_arq(&path);

    BufReader::new(File::open(&path).unwrap()).lines().flatten()
}

pub fn arq_escrever(dir: &str, file: &str, linhas: &Vec<String>) {
    let mut path = my_home().unwrap().unwrap();
    path.push(&dir);
    path.push(&file);

    checar_dir(dir);
    checar_arq(&path);

    write(path, linhas.join("\n"))
        .expect("Falha ao escrever no arquivo");
}

fn checar_dir(path: &str) {
    if !Path::new(&path).exists() {
        create_dir_all(&path).expect("Falha ao criar diretorio");
    }
}

fn checar_arq(path: &PathBuf) {
    if !Path::new(&path).exists() {
        write(&path, "").expect("Falha ao criar arquivo");
    }
}