use super::{fluxo_regra_dto::FluxoRegra, Categoria, Lancamento};

#[derive(Debug, Clone)]
pub struct NovaRegra {
    pub texto: String,
    pub regex: String,
    pub fluxo: FluxoRegra,
    pub lancamentos: Vec<Lancamento>,
    pub categoria: Option<Categoria>,
}

impl NovaRegra {
    pub fn new(texto: String, fluxo: FluxoRegra, lancamentos: Vec<Lancamento>) -> Self {
        Self {
            texto: texto.clone(),
            categoria: None,
            fluxo: fluxo,
            regex: texto.clone(),
            lancamentos: lancamentos,
        }
    }
}
