use super::{gerar_sha1, Unico, CSV};

#[derive(PartialEq, Default, Clone)]
pub struct Meta {
    pub id: String,
    pub nome: String,
    pub desc: String,
    pub tipo_meta: String,
    pub filtro: String,
    pub metrica: String,
    pub fluxo: String,
    pub periodo: String,
    pub valor: f64,
}

impl Unico for Meta {
    fn gerar_id(&mut self) {
        let mut itens: Vec<String> = Vec::new();

        itens.push(self.nome.clone());

        self.id = gerar_sha1(itens.join("-"));
    }
}

impl CSV for Meta {
    fn from_csv(value: String) -> Self {
        let values: Vec<String> = value.split(';').map(String::from).collect();
        Meta::from_csv_vec(values)
    }

    fn from_csv_vec(value: Vec<String>) -> Self {
        Meta {
            id: value[0].clone(),
            nome: value[1].clone(),
            desc: value[2].replace("\\n", "\n"),
            tipo_meta: value[3].clone(),
            filtro: value[4].clone(),
            metrica: value[5].clone(),
            fluxo: value[6].clone(),
            periodo: value[7].clone(),
            valor: value[8].parse().unwrap(),
        }
    }

    fn to_csv(&self) -> String {
        let mut resp: Vec<String> = Vec::new();

        resp.push(self.id.clone());
        resp.push(self.nome.clone());
        resp.push(self.desc.replace("\n", "\\n"));
        resp.push(self.tipo_meta.clone());
        resp.push(self.filtro.clone());
        resp.push(self.metrica.clone());
        resp.push(self.fluxo.clone());
        resp.push(self.periodo.clone());
        resp.push(self.valor.to_string());

        resp.join(";")
    }
}
