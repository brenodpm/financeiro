use std::fs::{read_dir, rename};

use chrono::NaiveDate;

use crate::{
    dto::{CsvLayout, Lancamento, Unico},
    get_home_dir,
    repository::file_repy::arq_externo_ler,
};

/// Lista os arquivos CSV no diretório de importação.
pub fn listar_csvs() -> Vec<String> {
    let mut dir = get_home_dir();
    dir.push("Downloads/importar");

    match read_dir(&dir) {
        Ok(rd) => rd
            .filter_map(|r| r.ok())
            .map(|e| e.path().display().to_string())
            .filter(|s| s.to_lowercase().ends_with(".csv"))
            .collect(),
        Err(e) => {
            log::error!("Erro ao listar CSVs: {e}");
            vec![]
        }
    }
}

/// Lê as primeiras `n` linhas do arquivo para prévia.
/// Retorna os lançamentos e um erro de parse se nenhum foi gerado.
pub fn preview(arquivo: &str, layout: &CsvLayout, n: usize) -> (Vec<Lancamento>, Option<String>) {
    let resultado = parse_arquivo(arquivo, layout, Some(n));
    if resultado.is_empty() {
        let erro = format!(
            "Nenhum lançamento parseado. Verifique as configurações do layout:\n  delimitador: {:?}\n  col. data: {} / formato: {}\n  col. descrição: {}\n  col. valor: {}",
            layout.delimitador,
            layout.col_data + 1, layout.formato_data,
            layout.col_descricao + 1,
            layout.col_valor + 1,
        );
        (resultado, Some(erro))
    } else {
        (resultado, None)
    }
}

/// Importa todos os lançamentos do arquivo e move para 'importado'.
pub fn importar(arquivo: &str, layout: &CsvLayout) -> Vec<Lancamento> {
    let lancamentos = parse_arquivo(arquivo, layout, None);
    mover_para_importado(arquivo);
    lancamentos
}

fn parse_arquivo(arquivo: &str, layout: &CsvLayout, limite: Option<usize>) -> Vec<Lancamento> {
    let linhas = arq_externo_ler(arquivo);
    let delim = layout.delimitador.to_string();
    linhas
        .into_iter()
        .take(limite.unwrap_or(usize::MAX))
        .filter_map(|linha| parse_linha(&linha, &delim, layout))
        .collect()
}

fn parse_linha(linha: &str, delim: &str, layout: &CsvLayout) -> Option<Lancamento> {
    let cols: Vec<&str> = linha.split(delim).collect();

    let max_col = layout.col_descricao.max(layout.col_valor).max(layout.col_data);

    if cols.len() <= max_col {
        return None;
    }

    let descricao = cols[layout.col_descricao].trim().to_lowercase();
    let valor_str = cols[layout.col_valor].trim().replace(',', ".").replace("\"", "");
    let valor: f64 = {
        let v: f64 = valor_str.parse::<f64>().ok()?;
        if layout.inverter_sinal { -v } else { v }
    };
    let data_str = cols[layout.col_data].trim().replace("\"", "");
    let data = NaiveDate::parse_from_str(&data_str, &layout.formato_data).ok()?;

    if descricao.is_empty() {
        return None;
    }

    let conta = match layout.col_conta {
        Some(col) => cols.get(col).map(|s| s.trim().to_string()).unwrap_or_else(|| layout.conta.clone()),
        None => layout.conta.clone(),
    };

    let mut l = Lancamento {
        descricao,
        valor,
        data,
        conta: Some(conta),
        ..Default::default()
    };
    l.gerar_id();
    Some(l)
}

fn mover_para_importado(arquivo: &str) {
    let novo = arquivo.replace("importar", "importado");
    if let Err(e) = rename(arquivo, &novo) {
        log::error!("Erro ao mover CSV {arquivo}: {e}");
    }
}
