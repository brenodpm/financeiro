use super::{Categoria, Lancamento};

#[derive(Debug, Clone)]
pub struct NovaRegra {
    pub regex: String,
    pub fluxo: char,
    pub lancamentos: Vec<Lancamento>,
    pub categoria: Option<Categoria>,
    pub info: String,
}

impl NovaRegra {
    pub fn new(texto: String, fluxo: char, lancamentos: Vec<Lancamento>) -> Self {
        Self {
            categoria: None,
            fluxo: fluxo,
            regex: texto.clone(),
            info: texto,
            lancamentos: lancamentos,
        }
    }
}
