
#[derive(serde::Serialize, serde::Deserialize)]
pub struct DashResumo {
    pub media_entradas: f64,
    pub media_saidas: f64,
    pub media_custo_vida: f64,
    pub atual_entradas: f64,
    pub atual_saidas: f64,
}
