use super::{DadosDivida, ParcelaDivida};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct DividaMes{
    pub mes: String,
    pub pago: f64,
    pub aberto: f64,
    pub excesso:f64
}

impl DividaMes {
    pub fn new(parcelas: Vec<ParcelaDivida>, limite: f64, nome:String)->Self{
        let pagas  = parcelas.pagas().valor_total();
        let mut aberto = parcelas.aberta().valor_total();
        let mut excesso = 0f64;

        if aberto > limite {
            excesso = aberto - limite;
            aberto = limite;
        }

        Self { mes: nome, pago: pagas, aberto: aberto, excesso: excesso }
    }
}