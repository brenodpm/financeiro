use serde::{Deserialize, Serialize};

use super::{
    fluxo_regra_dto::FluxoRegra, gerar_sha1, Categoria, Lazy, OptionalLazy, OptionalLazyFn,
    Unico,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Regra {
    pub id: String,
    pub fluxo: FluxoRegra,
    pub regex: String,
    pub categoria: Lazy<Categoria>,
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
        self.id = gerar_sha1(format!("{}-{:?}", self.regex.clone(), self.fluxo.clone()))
    }
}
