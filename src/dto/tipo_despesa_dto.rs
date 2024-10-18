#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub enum TipoDespesa {
    Fixa,
    Variavel,
    Perda,
    Vazio,
}