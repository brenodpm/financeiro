#[derive(Debug, Clone)]
pub struct Categoria {
    pub _nome: String,
    pub tipo: TipoFluxo,
}

#[derive(Debug, Clone)]
pub enum TipoFluxo {
    Receitas(String),
    Despesas(GrupoDespesa),
    Investimento,
    Retorno,
    Vazio,
}

#[derive(Debug, Clone)]
pub struct GrupoDespesa {
    pub nome: String,
    pub tipo: TipoDespesa,
}

#[derive(Debug, Clone)]
pub enum TipoDespesa {
    Fixa,
    Variavel,
    Perdas,
    Vazio,
}
