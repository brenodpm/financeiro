#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Conta {
    pub id: String,
    pub nome: String,
}