use super::Lancamento;

#[derive(Debug)]
pub struct NovaRegra {
    pub regex: String,
    pub lancamentos: Vec<Lancamento>,
    pub categoria: Option<String>,
    pub info: String,
}

impl NovaRegra {
    pub fn new(todo: String, lancamentos: Vec<Lancamento>) -> Self {
        Self {
            categoria: None,
            regex: todo.clone(),
            info: todo,
            lancamentos: lancamentos,
        }
    }
}
