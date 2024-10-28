#[derive(Debug, Clone, PartialEq)]
pub enum FluxoRegra {
    Entrada,
    Saida,
    None,
}

impl FluxoRegra {
    pub fn to_line(&self) -> String {
        match self {
            FluxoRegra::Entrada => "E".to_string(),
            FluxoRegra::Saida => "S".to_string(),
            FluxoRegra::None => String::new(),
        }
    }
    pub fn from_line(value: String) -> Self {
        match value.as_str() {
            "E" => FluxoRegra::Entrada,
            "S" => FluxoRegra::Saida,
            _ => FluxoRegra::None,
        }
    }
}
