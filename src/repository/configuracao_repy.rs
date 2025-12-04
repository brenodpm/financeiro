use crate::dto::Configuracao;

use super::file_repy::{arq_escrever, arq_ler};

const DIR: &str = ".financeiro";
const FILE: &str = "config.json";

impl Configuracao {
    pub fn salvar(&self) {
        match serde_json::to_string_pretty(self) {
            Ok(json) => arq_escrever(DIR, FILE, json),
            Err(erro) => log::error!("Erro ao salvar configuração: {}", erro),
        };
    }

    pub fn buscar() -> Self {
        let json: String = arq_ler(DIR, FILE).collect();

        if json.is_empty() {
            let config = Configuracao::default();
            config.salvar();
            config
        } else {
            let resp: Configuracao = match serde_json::from_str(&json) {
                Ok(conf) => conf,
                Err(erro) => {
                    println!("Erro ao ler configuração: {}", erro);
                    Configuracao::default()
                }
            };
            resp
        }
    }

    pub fn atualizar_contracheque(empresa: String, entradas: Vec<String>, saidas: Vec<String>) {
        let mut conf = Configuracao::buscar();

        conf.contracheque_entradas = entradas;
        conf.contracheque_saidas = saidas;
        conf.contracheque_empresa = empresa;

        conf.salvar();
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
