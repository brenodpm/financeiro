use itertools::Itertools;

use crate::dto::{DadosDivida, Divida, CSV};

use super::file_repy::{arq_escrever_linhas, arq_ler};

const FIN: &str = ".financeiro";
const CAT: &str = "dividas.csv";

impl Divida {
    pub fn listar() -> Vec<Divida> {
        arq_ler(FIN, CAT)
            .map(Divida::from_csv)
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

        arq_escrever_linhas(FIN, CAT, &lista.into_iter().map(|i| i.to_csv()).collect())
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

        arq_escrever_linhas(FIN, CAT, &lista.into_iter().map(|i| i.to_csv()).collect())
    }
}
