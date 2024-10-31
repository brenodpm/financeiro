use std::fs::{read_dir, rename};

use chrono::NaiveDate;
use homedir::my_home;

use crate::{
    dto::{Banco, Conta, Lancamento, Unico},
    repository::file_repy::arq_externo_ler,
};
impl Lancamento {
    pub fn from_ofx() -> (Vec<Lancamento>, Vec<Banco>) {
        let mut dir = my_home().unwrap().unwrap();
        dir.push("Downloads/importar");

        log::info!("Importando XSD");

        let mut lancamentos: Vec<Lancamento> = Vec::new();
        let mut bancos: Vec<Banco> = Vec::new();

        read_dir(dir)
            .unwrap()
            .map(|r| r.unwrap().path().display().to_string())
            .filter(|s| s.ends_with("ofx"))
            .for_each(|arquivo| {
                importar_lancts(&mut lancamentos, &mut bancos, &arquivo);
            });

        (lancamentos, bancos)
    }
}

fn importar_lancts(lista: &mut Vec<Lancamento>, bancos: &mut Vec<Banco>, arquivo: &str) {
    let mut item = Lancamento::default();
    let mut banco = String::new();
    let mut conta = String::new();

    let mut count: u32 = 0;

    for mut linha in arq_externo_ler(arquivo) {
        if let Some(pos) = linha.find('>') {
            linha = linha[1..].to_string();
            if pos + 1 < linha.len() {
                let chave = &linha[..pos - 1];
                let valor = &linha[pos..linha.find('<').unwrap_or(linha.len())];
                match chave {
                    "BANKID" => banco = valor.to_uppercase(),
                    "ACCTID" => conta = valor.to_lowercase(),
                    "MEMO" => item.descricao = valor.to_ascii_lowercase(),
                    "TRNAMT" => item.valor = valor.parse().unwrap(),
                    "DTPOSTED" => {
                        item.data = NaiveDate::parse_from_str(&valor[..8], "%Y%m%d").unwrap()
                    }
                    _ => {}
                }
            } else if linha.eq("/STMTTRN>") {
                add_lancamento(&mut item, &conta, lista);
                count += 1;
            } else if linha.eq("/BANKACCTFROM>") {
                bancos.push(add_banco(&banco, &conta));
            }
        }
    }
    log::info!(
        "arquivo: {}: {count} itens",
        arquivo.split('/').last().unwrap()
    );

    mover_para_importado(&arquivo);
}

fn add_lancamento(item: &mut Lancamento, conta: &String, lista: &mut Vec<Lancamento>) {
    item.conta = Some(conta.clone());
    item.gerar_id();
    lista.push(item);
    *item = Lancamento::default();
}

fn add_banco(banco: &String, conta: &String)->Banco{
    Banco {
        id: banco.clone(),
        nome: banco.clone(),
        contas: vec![Conta {
            id: conta.clone(),
            id_banco: banco.clone(),
            nome: conta.clone(),
        }],
    }
}

fn mover_para_importado(arquivo: &str) {
    let novo = arquivo.to_string().replace("importar", "importado");
    rename(arquivo, novo).expect("Erro ao mover");
}