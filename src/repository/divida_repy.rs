use itertools::Itertools;

use crate::dto::{DadosDivida, Divida, CSV};

use super::file_repy::{arq_escrever, arq_ler};

const FIN: &str = ".financeiro";
const CAT: &str = "dividas.json";

impl Divida {
    pub fn listar() -> Vec<Divida> {
        let mut json: String = arq_ler(FIN, CAT).collect();
        if json.is_empty() {
            json = "[]".to_string();
        }
        let resp: Vec<Divida> = serde_json::from_str(&json).unwrap();

        resp
            .into_iter()
            .sorted_by(|a, b| {
                a.prox_parcela()
                    .data_vencimento
                    .partial_cmp(&b.prox_parcela().data_vencimento)
                    .unwrap()
            })
            .collect::<Vec<Divida>>()
    }

    pub fn salvar(&self) {
        let mut lista = Divida::listar();

        if let Some(i) = lista.iter().position(|a| a.id == self.id) {
            lista[i] = self.clone();
        } else {
            lista.push(self.clone());
        }

        arq_escrever(FIN, CAT, serde_json::to_string(&lista).unwrap());
    }

    pub fn atualizar() {
        let mut lista = Divida::listar()
            .into_iter()
            .filter(|d| d.parcelas.aberta().len() > 0)
            .collect::<Vec<Divida>>();

        for divida in lista.iter_mut() {
            if divida.cobranca_automatica {
            for parcela in divida.parcelas.iter_mut() {
                if parcela.data_vencimento < chrono::Local::now().naive_local().date() {
                parcela.pago = true;
                }
            }
            }
        }

        arq_escrever(FIN, CAT, serde_json::to_string(&lista).unwrap());
    }
}
