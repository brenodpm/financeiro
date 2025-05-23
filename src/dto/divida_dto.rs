use chrono::{Datelike, NaiveDate};

use super::{gerar_sha1, ParcelaDivida, Unico};

#[derive(Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Divida {
    pub id: String,
    pub nome: String,
    pub cobranca_automatica: bool,
    pub parcelas: Vec<ParcelaDivida>,
}

pub trait DadosDivida {
    fn primeira(&self) -> ParcelaDivida;
    fn ultima(&self) -> ParcelaDivida;

    fn valor_total(&self) -> f64;
    fn quant(&self) -> i32;

    fn antes_de(&self, data: NaiveDate) -> Self;
    fn data_igual_ou_maior_que(&self, data: NaiveDate) -> Self;
    fn mes_e_ano(&self, data: NaiveDate) -> Self;

    fn pagas(&self) -> Self;
    fn aberta(&self) -> Self;
}

impl Divida {
    pub fn new(
        nome: String,
        cobranca_automatica: bool,
        quant: i32,
        valor: f64,
        dt_inicio: NaiveDate,
        quant_ja_pago: i32,
    ) -> Self {
        let mut divida = Divida {
            id: String::new(),
            nome,
            cobranca_automatica,
            parcelas: Vec::new(),
        };

        for i in 1..=quant {
            divida.parcelas.push(ParcelaDivida {
                num_parcela: i,
                valor,
                pago: i <= quant_ja_pago,
                data_vencimento: dt_inicio
                    .clone()
                    .checked_add_months(chrono::Months::new((i as u32) - 1))
                    .unwrap(),
            });
        }

        divida.gerar_id();
        divida
    }

    pub fn prox_parcela(&self) -> ParcelaDivida {
        if self.parcelas.aberta().quant() > 0 {
            self.parcelas.aberta().primeira()
        } else {
            self.parcelas.pagas().ultima()
        }
    }
}

impl DadosDivida for Vec<ParcelaDivida> {
    fn primeira(&self) -> ParcelaDivida {
        self.first().unwrap().clone()
    }

    fn ultima(&self) -> ParcelaDivida {
        self.last().unwrap().clone()
    }

    fn valor_total(&self) -> f64 {
        self.iter().map(|v| v.valor).sum()
    }

    fn quant(&self) -> i32 {
        self.len() as i32
    }

    fn antes_de(&self, data: NaiveDate) -> Self {
        self.iter()
            .filter(|p| p.data_vencimento < data)
            .cloned()
            .collect()
    }

    fn data_igual_ou_maior_que(&self, data: NaiveDate) -> Self {
        self.iter()
            .filter(|p| p.data_vencimento >= data)
            .cloned()
            .collect()
    }

    fn mes_e_ano(&self, data: NaiveDate) -> Self {
        self.iter()
            .filter(|p| {
                p.data_vencimento.year_ce() == data.year_ce()
                    && p.data_vencimento.month0() == data.month0()
            })
            .cloned()
            .collect()
    }

    fn pagas(&self) -> Self {
        self.iter().filter(|p| p.pago).cloned().collect()
    }

    fn aberta(&self) -> Self {
        self.iter().filter(|p| !p.pago).cloned().collect()
    }
}

impl Unico for Divida {
    fn gerar_id(&mut self) {
        self.id = gerar_sha1(
            vec![
                self.nome.clone(),
                self.parcelas.quant().to_string(),
                self.parcelas.valor_total().to_string(),
            ]
            .join("-"),
        )
    }
}
