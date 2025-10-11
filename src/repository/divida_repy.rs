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

        resp.into_iter()
            .sorted_by(|a, b| {
                if a.prioritaria != b.prioritaria && b.prox_parcela().data_vencimento <= corte {
                    b.prioritaria.partial_cmp(&a.prioritaria).unwrap()
                } else {
                    a.prox_parcela()
                        .data_vencimento
                        .partial_cmp(&b.prox_parcela().data_vencimento)
                        .unwrap()
                }
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
        let corte = chrono::Local::now().naive_local().date() - chrono::Duration::days(30);

        let mut lista = Divida::listar()
            .into_iter()
            .filter(|d| {
                d.parcelas.aberta().len() > 0 || d.parcelas.ultima().data_vencimento >= corte
            })
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
