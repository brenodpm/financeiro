use std::cmp::Ordering;

use itertools::Itertools;

use crate::dto::{DadosDivida, Divida};

use super::file_repy::{arq_escrever, arq_ler};

const FIN: &str = ".financeiro";
const CAT: &str = "dividas.json";

impl Divida {
    pub fn listar() -> Vec<Divida> {
        let corte = chrono::Local::now().naive_local().date() + chrono::Duration::days(10);
        let mut json: String = arq_ler(FIN, CAT).collect();
        if json.is_empty() {
            json = "[]".to_string();
        }
        let resp: Vec<Divida> = serde_json::from_str(&json).unwrap();

        let sorted: Vec<Divida> = resp
            .into_iter()
            .sorted_by(|a, b| {
                let a_p = a.prox_parcela();
                let b_p = b.prox_parcela();

                let a_urgente = a.prioritaria && a_p.data_vencimento <= corte;
                let b_urgente = b.prioritaria && b_p.data_vencimento <= corte;

                if a_urgente != b_urgente {
                    return if a_urgente {
                        Ordering::Less
                    } else {
                        Ordering::Greater
                    };
                }

                a_p.data_vencimento.cmp(&b_p.data_vencimento)
            })
            .collect();

        sorted
    }

    pub fn salvar(&self) {
        let mut lista = Divida::listar();

        if let Some(i) = lista.iter().position(|a| a.id == self.id) {
            lista[i] = self.clone();
        } else {
            lista.push(self.clone());
        }

        arq_escrever(FIN, CAT, serde_json::to_string_pretty(&lista).unwrap());
    }

    pub fn atualizar() {
        let corte = chrono::Local::now().naive_local().date() - chrono::Duration::days(30);

        let mut lista: Vec<Divida> = Vec::new();
        Divida::listar().into_iter().for_each(|divida| {
            if divida
                .parcelas
                .iter()
                .any(|p| !p.pago || p.data_vencimento >= corte)
            {
                lista.push(divida);
            }
        });

        for divida in lista.iter_mut() {
            if divida.cobranca_automatica {
                for parcela in divida.parcelas.iter_mut() {
                    if parcela.data_vencimento < chrono::Local::now().naive_local().date() {
                        parcela.pago = true;
                    }
                }
            }
        }

        arq_escrever(FIN, CAT, serde_json::to_string_pretty(&lista).unwrap());
    }
}
