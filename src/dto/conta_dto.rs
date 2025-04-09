#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Conta {
    pub id: String,
    pub nome: String,
}