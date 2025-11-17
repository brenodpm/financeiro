#[derive(serde::Serialize, serde::Deserialize)]
pub struct DashGastoPorConta {
    pub conta: String,
    pub valor: f64,
}