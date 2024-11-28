use super::{
    fluxo_regra_dto::FluxoRegra, gerar_sha1, Categoria, Lazy, LazyFn, OptionalLazy, OptionalLazyFn,
    Unico, CSV,
};

#[derive(Debug, Clone)]
pub struct Regra {
    pub id: String,
    pub fluxo: FluxoRegra,
    pub regex: String,
    pub categoria: Lazy<Categoria>,
}

impl CSV for Regra {
    fn from_csv(value: String) -> Self {
        let values: Vec<String> = value.split(';').map(String::from).collect();
        Regra::from_csv_vec(values)
    }

    fn from_csv_vec(value: Vec<String>) -> Self {
        Regra {
            id: value[0].clone(),
            regex: value[1].clone(),
            fluxo: FluxoRegra::from_string(value[2].clone()),
            categoria: Lazy::Id(value[3].clone()),
        }
    }

    fn to_csv(&self) -> String {
        let mut resp: Vec<String> = Vec::new();

        resp.push(self.id.clone());
        resp.push(self.regex.to_lowercase());
        resp.push(self.fluxo.to_string());
        resp.push(self.categoria.id());

        resp.join(";")
    }
}

impl OptionalLazyFn<Regra> for OptionalLazy<Regra> {
    fn id(&self) -> String {
        match self {
            OptionalLazy::Id(id) => id.clone(),
            OptionalLazy::Some(cat) => cat.id.clone(),
            OptionalLazy::None => String::new(),
        }
    }

    fn some(&self) -> Option<Regra> {
        match self {
            OptionalLazy::None => None,
            OptionalLazy::Id(id) => Some(Regra {
                id: id.clone(),
                fluxo: FluxoRegra::None,
                regex: String::new(),
                categoria: Lazy::Id(String::new()),
            }),
            OptionalLazy::Some(regra) => Some(regra.clone()),
        }
    }
}

impl Unico for Regra {
    fn gerar_id(&mut self) {
        self.id = gerar_sha1(self.to_csv())
    }
}
