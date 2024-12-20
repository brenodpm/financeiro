
use super::{gerar_sha1, ParcelaDivida, Unico, CSV};

#[derive(Clone)]
pub struct Divida {
    pub id: String,
    pub nome: String,
    pub parcelas: Vec<ParcelaDivida>,
}

pub trait DadosDivida {
    fn primeira(&self) -> ParcelaDivida;
    fn ultima(&self) -> ParcelaDivida;
    fn valor_total(&self) -> f64;
    fn quant(&self) -> usize;
}

impl Divida {
    pub fn pagas(&self) -> Vec<ParcelaDivida> {
        self.parcelas
            .clone()
            .into_iter()
            .filter(|p| p.pago)
            .collect()
    }
    pub fn aberta(&self) -> Vec<ParcelaDivida> {
        self.parcelas
            .clone()
            .into_iter()
            .filter(|p| !p.pago)
            .collect()
    }
    pub fn prox_parcela(&self) -> ParcelaDivida {
        if self.aberta().quant() > 0 {
            self.aberta().primeira()
        } else {
            self.pagas().ultima()
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

    fn quant(&self) -> usize {
        self.len()
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
impl CSV for Divida {
    fn from_csv(value: String) -> Self {
        let values: Vec<String> = value.split(';').map(String::from).collect();
        Divida::from_csv_vec(values)
    }

    fn from_csv_vec(value: Vec<String>) -> Self {
        Divida {
            id: value[0].clone(),
            nome: value[1].clone(),
            parcelas: get_parcelas_csv(value.clone().drain(2..).collect()),
        }
    }

    fn to_csv(&self) -> String {
        let mut resp: Vec<String> = Vec::new();

        resp.push(self.id.clone());
        resp.push(self.nome.clone());

        self.parcelas.iter().for_each(|p| {
            resp.push(p.to_csv());
        });

        resp.join(";")
    }
}

fn get_parcelas_csv(value: Vec<String>) -> Vec<ParcelaDivida> {
    let mut parcelas: Vec<ParcelaDivida> = Vec::new();

    value
        .iter()
        .for_each(|v| parcelas.push(ParcelaDivida::from_csv(v.to_string())));

    parcelas
}
