use super::fluxo_regra_dto::FluxoRegra;


#[derive(Debug)]
pub struct Regra {
    pub fluxo: FluxoRegra,
    pub regex: String,
    pub categoria: String,
}

impl Regra {
    pub fn to_line(&self) -> String {
        let mut resp: Vec<String> = Vec::new();

        resp.push(self.regex.to_lowercase());
        resp.push(self.fluxo.to_line());
        resp.push(self.categoria.clone());

        resp.join(";")
    }
}

impl From<String> for Regra {
    #[inline]
    fn from(s: String) -> Regra {
        let attrs: Vec<String> = s.split(';').map(String::from).collect();
        Regra {
            regex: attrs[0].clone(),
            fluxo: FluxoRegra::from_line(attrs[1].clone()),
            categoria: attrs[2].clone(),
        }
    }
}