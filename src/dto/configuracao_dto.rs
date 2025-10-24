#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Configuracao {

    #[serde(default)]
    pub salario: f64,
    
    #[serde(default)]
    pub endividamento_max: f64,

    #[serde(default)]
    pub contracheque_entradas: Vec<String>,

    #[serde(default)]
    pub contracheque_saidas: Vec<String>,
}
