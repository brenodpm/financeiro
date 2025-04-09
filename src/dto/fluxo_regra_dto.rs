use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FluxoRegra {
    Entrada,
    Saida,
    None,
}
