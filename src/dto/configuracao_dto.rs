#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Configuracao {
    pub salario: f64,
    pub endividamento_max: f64,
}
