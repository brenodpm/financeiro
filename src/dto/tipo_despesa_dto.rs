use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum TipoDespesa {
    Fixa,
    Variavel,
    Perda,
    Vazio,
}