use serde_json;
use crate::dto::DividaMes;

use super::file_repy::arq_escrever;

const DIR: &str = "Financeiro/data";
const FILE: &str = "dividas.js";


impl DividaMes{
    pub fn salvar(dividas: Vec<DividaMes>){
        let json = serde_json::to_string_pretty(&dividas).unwrap();
        let valor = format!("var dividas = {};", json);
        arq_escrever(DIR, FILE, valor);
    }
}