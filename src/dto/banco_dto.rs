use super::{Conta, CSV};

#[derive(Debug, Clone, Default)]
pub struct Banco {
    pub id: String,
    pub nome: String,
    pub contas: Vec<Conta>,
}

impl Banco {
    pub fn novo(id: String) -> Self {
        Self {
            id: id.clone(),
            nome: id,
            contas: Vec::new(),
        }
    }
}

impl CSV for Banco {
    fn from_csv(value: String) -> Self {
        Banco::from_csv_vec(value.split(';').map(String::from).collect())
    }

    fn from_csv_vec(value: Vec<String>) -> Self {
        Banco {
            id: value[0].clone(),
            nome: value[1].clone(),
            contas: Vec::new(),
        }
    }

    fn to_csv(&self) -> String {
        let mut resp: Vec<String> = Vec::new();

        resp.push(self.id.clone());
        resp.push(self.nome.clone());

        resp.join(";")
    }
}
