use super::CSV;

#[derive(Debug, Clone)]
pub struct Conta {
    pub id: String,
    pub id_banco: String,
    pub nome: String,
}

impl CSV for Conta {
    fn from_csv(value: String) -> Self {
        Conta::from_csv_vec(value.split(';').map(String::from).collect())
    }

    fn from_csv_vec(value: Vec<String>) -> Self {
        Conta {
            id: value[0].clone(),
            id_banco: value[1].clone(),
            nome: value[2].clone(),
        }
    }

    fn to_csv(&self) -> String {
        let mut resp: Vec<String> = Vec::new();

        resp.push(self.id.clone());
        resp.push(self.id_banco.clone());
        resp.push(self.nome.clone());

        resp.join(";")
    }
}
