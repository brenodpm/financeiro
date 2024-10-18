use super::{Categoria, SubVec};

pub struct Regra {
    pub regex: String,
    pub categoria: Categoria,
}

impl Regra {
    pub fn to_string(&self) -> String {
        let mut resp: Vec<String> = Vec::new();

        resp.push(self.regex.clone());

        resp.join(";")
    }
}

impl From<String> for Regra {
    #[inline]
    fn from(s: String) -> Regra {
        let attrs: Vec<String> = s.split(';').map(String::from).collect();
        Regra {
            regex: attrs[0].clone(),
            categoria: Categoria::from(attrs.sub_vec()),
        }
    }
}