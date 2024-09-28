use std::fs::read_dir;

use chrono::NaiveDate;
use homedir::my_home;

use crate::{dto::Lancamento, repository::arq_ler_windows_1252};

impl Lancamento {
    pub fn from_ofx() -> Vec<Lancamento> {
        let mut dir = my_home().unwrap().unwrap();
        dir.push("Downloads/importar");

        println!("\n\nImportando XSD");
        let mut resp: Vec<Lancamento> = Vec::new();
        read_dir(dir)
            .unwrap()
            .map(|r| r.unwrap().path().display().to_string())
            .filter(|s| s.ends_with("ofx"))
            .for_each(|arquivo| {
                importar_lancts(&mut resp, &arquivo);
            });

        resp
    }
}

fn importar_lancts(lista: &mut Vec<Lancamento>, arquivo: &str) {
    let mut item = Lancamento::default();
    let mut count: u32 = 0;

    for mut linha in arq_ler_windows_1252(arquivo) {
        if let Some(pos) = linha.find('>') {
            linha = linha[1..].to_string();
            if pos + 1 < linha.len() {
                let chave = &linha[..pos - 1];
                let valor = &linha[pos..linha.find('<').unwrap_or(linha.len())];
                match chave {
                    "FITID" => item.id = valor.to_string(),
                    "MEMO" => item.descricao = valor.to_string(),
                    "TRNAMT" => item.valor = valor.parse().unwrap(),
                    "DTPOSTED" => {
                        item.data = NaiveDate::parse_from_str(&valor[..8], "%Y%m%d").unwrap()
                    }
                    _ => {}
                }
            } else if linha.eq("/STMTTRN>") {
                lista.push(item);
                item = Lancamento::default();
                count += 1;
            }
        }
    }
    println!("arquivo: {}: {count} itens", arquivo.split('/').last().unwrap());
}
