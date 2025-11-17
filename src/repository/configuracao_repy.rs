use crate::dto::Configuracao;

use super::file_repy::{arq_escrever, arq_ler};

const DIR: &str = ".financeiro";
const FILE: &str = "config.json";

impl Configuracao {
    pub fn salvar(&self) {
        let json = serde_json::to_string_pretty(self).unwrap();
        arq_escrever(DIR, FILE, json);
    }

    pub fn buscar() -> Self {
        let json: String = arq_ler(DIR, FILE).collect();

        if json.is_empty() {
            let config = Configuracao::default();
            config.salvar();
            config
        } else {
            let resp: Configuracao = serde_json::from_str(&json).unwrap();
            resp
        }
    }
}

impl Default for Configuracao {
    fn default() -> Self {
        Self {
            salario: Default::default(),
            endividamento_max: Default::default(),
            contracheque_entradas: Default::default(),
            contracheque_saidas: Default::default(),
            contracheque: true,
            contracheque_empresa: String::new(),
        }
    }
}
