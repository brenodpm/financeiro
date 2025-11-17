use std::collections::HashSet;


use crate::dto::Banco;

use super::file_repy::{arq_escrever, arq_ler};

const FIN: &str = ".financeiro";
const BANC: &str = "bancos.json";

impl Banco {
    pub fn listar() -> Vec<Banco> {
        let mut json: String = arq_ler(FIN, BANC).collect();
        if json.is_empty() {
            json = "[]".to_string();
        }
        serde_json::from_str(&json).unwrap()
    }

    pub fn salvar(novos: Vec<Banco>) {
        let mut bancos: Vec<Banco> = Banco::listar();

        merge_bancos(&mut bancos, novos);

        arq_escrever(FIN, BANC, serde_json::to_string_pretty(&bancos).unwrap());
    }
}

fn merge_bancos(atual: &mut Vec<Banco>, novos: Vec<Banco>) {
    for banco_novo in novos {
        if let Some(banco_atual) = atual.iter_mut().find(|b| b.id == banco_novo.id) {
            let contas_existentes: HashSet<String> =
                banco_atual.contas.iter().map(|c| c.id.clone()).collect();

            for conta in banco_novo.contas {
                if !contas_existentes.contains(&conta.id) {
                    banco_atual.contas.push(conta);
                }
            }
        } else {
            atual.push(banco_novo);
        }
    }
}
