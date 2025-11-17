use crate::dto::{DadosDivida, ParcelaDivida};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct DashDivida {
    pub mes: String,
    pub pago: f64,
    pub aberto: f64,
    pub excesso: f64,
}

pub trait DashDividaExt {
    fn to_dash(&self, nome: &str, limite: f64) -> DashDivida;
}

impl DashDividaExt for Vec<ParcelaDivida> {
    fn to_dash(&self, nome: &str, mut limite: f64) -> DashDivida {
        let pagas = self.pagas().valor_total();
        let mut aberto = self.aberta().valor_total();
        let mut excesso = 0f64;

        if limite < 0.0{
            limite = 0.0;
        }

        if aberto > limite {
            excesso = aberto - limite;
            aberto = limite;
        }

        DashDivida {
            mes: nome.to_string(),
            pago: pagas,
            aberto: aberto,
            excesso: excesso,
        }
    }
}
