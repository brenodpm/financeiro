use std::fmt::{self, Formatter, Result};

use chrono::format::Item;

use super::{gerar_sha1, DtoIdentificado, SubVec, TipoFluxo};

#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct Categoria {
    pub id: String,
    pub nome: String,
    pub tipo: TipoFluxo,
}

pub struct Categorias {
    pub receitas: Vec<Categoria>,
    pub despesas: Vec<Categoria>,
}

impl DtoIdentificado for Categoria {
    fn gerar_id(&mut self) {
        let mut itens: Vec<String> = Vec::new();

        itens.push(self.nome.clone());
        itens.push(self.tipo.to_string());

        self.id = gerar_sha1(itens.join(":"));
    }
}

impl From<String> for Categoria {
    #[inline]
    fn from(value: String) -> Categoria {
        let values: Vec<String> = value.split(';').map(String::from).collect();
        Categoria::from(values)
    }
}

impl From<Vec<String>> for Categoria {
    #[inline]
    fn from(value: Vec<String>) -> Categoria {
        Categoria {
            id: value[0].clone(),
            nome: value[1].clone(),
            tipo: TipoFluxo::from(value.sub_vec()),
        }
    }
}

impl fmt::Display for Categoria {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.write_fmt(format_args!("{} {}", self.tipo, self.nome))
    }
}
