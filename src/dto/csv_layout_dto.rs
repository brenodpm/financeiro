use serde::{Deserialize, Serialize};

/// Mapeamento de colunas do CSV para campos do Lancamento.
/// Índice 0-based da coluna no arquivo.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CsvLayout {
    pub id: String,
    pub nome: String,
    pub delimitador: char,
    pub col_descricao: usize,
    pub col_valor: usize,
    pub col_data: usize,
    pub formato_data: String, // ex: "%d/%m/%Y"
    #[serde(default)]
    pub banco: String,        // id do banco
    pub conta: String,        // id fixo da conta (usado se col_conta for None)
    pub col_conta: Option<usize>, // se Some, lê a conta desta coluna do CSV
    #[serde(default)]
    pub inverter_sinal: bool,
}

impl CsvLayout {
    pub fn new(nome: &str) -> Self {
        Self {
            id: nome.to_lowercase().replace(' ', "_"),
            nome: nome.to_string(),
            delimitador: ';',
            col_descricao: 0,
            col_valor: 1,
            col_data: 2,
            formato_data: "%d/%m/%Y".to_string(),
            banco: String::new(),
            conta: String::new(),
            col_conta: None,
            inverter_sinal: false,
        }
    }
}
