use serde::Serialize;

#[derive(Serialize, Debug, PartialEq)]
pub struct Orientacao {
    pub prioridade: u8,
    pub icone: String,
    pub texto: String,
}
