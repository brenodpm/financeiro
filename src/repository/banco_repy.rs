use itertools::Itertools;

use crate::dto::{Banco, Conta, CSV};

use super::file_repy::{arq_escrever, arq_ler};

const FIN: &str = "financeiro";
const BANC: &str = "bancos.csv";
const CONT: &str = "contas.csv";

impl Banco {
    pub fn listar() -> Vec<Banco> {
        let mut resp: Vec<Banco> = arq_ler(FIN, BANC).map(Banco::from_csv).collect();

        resp.iter_mut().for_each(|b| {
            b.contas.append(
                &mut arq_ler(FIN, CONT)
                    .map(Conta::from_csv)
                    .collect(),
            );
        });

        resp
    }

    pub fn salvar(novos: Vec<Banco>) {
        let mut contas: Vec<Conta> = arq_ler(FIN, CONT).map(Conta::from_csv).collect();
        let mut bancos: Vec<Banco> = arq_ler(FIN, BANC).map(Banco::from_csv).collect();

        novos.into_iter().for_each(|nv_b| {
            if !bancos.iter().any(|b| b.id == nv_b.id) {
                bancos.push(nv_b.clone());
            }
            nv_b.contas.into_iter().for_each(|nv_c| {
                if !contas
                    .iter()
                    .any(|c| c.id == nv_c.id && c.id_banco == nv_c.id_banco)
                {
                    contas.push(nv_c);
                }
            });
        });

        arq_escrever(
            FIN,
            BANC,
            &bancos
                .into_iter()
                .sorted_by(|a, b| a.nome.cmp(&b.nome))
                .map(|i| i.to_csv())
                .collect(),
        );

        arq_escrever(
            FIN,
            CONT,
            &contas
                .into_iter()
                .sorted_by(|a, b| a.nome.cmp(&b.nome))
                .map(|i| i.to_csv())
                .collect(),
        );
    }
}
