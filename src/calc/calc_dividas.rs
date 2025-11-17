use chrono::{Local, Months};

use crate::dto::{DadosDivida, DashDivida, DashDividaExt, ParcelaDivida};

pub fn calcular_dividas(dividas: Vec<ParcelaDivida>, limite: f64) -> Vec<DashDivida> {
    let mut resp: Vec<DashDivida> = Vec::new();
    let mut data = Local::now().date_naive();

    let atrasado = dividas.aberta().antes_de(data).to_dash("aberto", 0.0);
    
    let atual = dividas
        .data_igual_ou_maior_que(data)
        .mes_e_ano(data)
        .to_dash("corrente", limite - atrasado.excesso);

    resp.push(atrasado);
    resp.push(atual);

    for _ in 0..11 {
        data = data.checked_add_months(Months::new(1)).unwrap();
        resp.push(
            dividas
                .mes_e_ano(data)
                .to_dash(data.format("%m/%Y").to_string().as_str(), limite),
        );
    }

    resp
}
