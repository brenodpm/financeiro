use std::fs::{read_dir, rename};

use chrono::NaiveDate;

use crate::{
    dto::{Banco, Conta, Lancamento, Unico},
    get_home_dir,
    repository::file_repy::arq_externo_ler,
};
impl Lancamento {
    pub fn from_ofx() -> (Vec<Lancamento>, Vec<Banco>) {
        let mut dir = get_home_dir();
        dir.push("Downloads/importar");

        log::info!("Importando XSD");

        let mut lancamentos: Vec<Lancamento> = Vec::new();
        let mut bancos: Vec<Banco> = Vec::new();

        match read_dir(dir) {
            Ok(read_dir) => {
                importar_arquivos_ofx_do_diretorio(&mut lancamentos, &mut bancos, read_dir);
            }
            Err(erro) => log::error!("Erro ao ler o diretório: {}", erro),
        }

        (lancamentos, bancos)
    }
}

fn importar_arquivos_ofx_do_diretorio(
    lancamentos: &mut Vec<Lancamento>,
    bancos: &mut Vec<Banco>,
    read_dir: std::fs::ReadDir,
) {
    read_dir
        .map(|r| validar_dir_entry(r))
        .filter(|s| s.ends_with("ofx"))
        .for_each(|arquivo| {
            importar_lancts(lancamentos, bancos, &arquivo);
        });
}

fn validar_dir_entry(r: Result<std::fs::DirEntry, std::io::Error>) -> String {
    match r {
        Ok(r) => r.path().display().to_string(),
        Err(erro) => {
            log::error!("Erro ao ler o diretório: {}", erro);
            String::new()
        }
    }
}

fn importar_lancts(lista: &mut Vec<Lancamento>, bancos: &mut Vec<Banco>, arquivo: &str) {
    let mut item = Lancamento::default();
    let mut banco = String::new();
    let mut conta = String::new();

    let mut count: u32 = 0;

    for linha in arq_externo_ler(arquivo) {
        if let Some(pos) = linha.find('>') {
            interpretar_linha(lista, bancos, &mut item, &mut banco, &mut conta, &mut count, linha, pos);
        }
    }
    
    log::info!(
        "arquivo: {}: {count} itens",
        arquivo.split('/').last().unwrap()
    );

    mover_para_importado(&arquivo);
}

fn interpretar_linha(lista: &mut Vec<Lancamento>, bancos: &mut Vec<Banco>, item: &mut Lancamento, banco: &mut String, conta: &mut String, count: &mut u32, mut linha: String, pos: usize) {
    linha = linha[1..].to_string();

    if pos + 1 < linha.len() {
        let chave = &linha[..pos - 1];
        let valor = &linha[pos..linha.find('<').unwrap_or(linha.len())];
        preencher_atributo_pela_tag(item, banco, conta, chave, valor);
    } else if linha.eq("/STMTTRN>") {
        add_lancamento(item, &*conta, lista);
        *count += 1;
    } else if linha.eq("/BANKACCTFROM>") {
        bancos.push(add_banco(&*banco, &*conta));
    }
}

fn preencher_atributo_pela_tag(item: &mut Lancamento, banco: &mut String, conta: &mut String, chave: &str, valor: &str) {
    match chave {
        "BANKID" => *banco = valor.to_uppercase(),
        "ACCTID" => *conta = valor.to_lowercase(),
        "MEMO" => item.descricao = valor.to_ascii_lowercase(),
        "TRNAMT" => item.valor = valor.parse().unwrap(),
        "DTPOSTED" => {
            item.data = NaiveDate::parse_from_str(&valor[..8], "%Y%m%d").unwrap()
        }
        _ => {}
    }
}

fn add_lancamento(item: &mut Lancamento, conta: &String, lista: &mut Vec<Lancamento>) {
    item.conta = Some(conta.clone());
    item.gerar_id();
    lista.push(item.clone());
    *item = Lancamento::default();
}

fn add_banco(banco: &String, conta: &String) -> Banco {
    Banco {
        id: banco.clone(),
        nome: banco.clone(),
        contas: vec![Conta {
            id: conta.clone(),
            nome: conta.clone(),
        }],
    }
}

fn mover_para_importado(arquivo: &str) {
    let novo = arquivo.to_string().replace("importar", "importado");
    rename(arquivo, novo).expect("Erro ao mover");
}
