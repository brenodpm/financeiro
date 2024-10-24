
pub struct Regra {
    pub regex: String,
    pub categoria: String,
}

impl Regra {
    pub fn to_line(&self) -> String {
        let mut resp: Vec<String> = Vec::new();

        resp.push(self.regex.clone());
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
            categoria: attrs[1].clone(),
        }
    }
}