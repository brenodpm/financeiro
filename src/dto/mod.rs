mod banco_dto;
mod categoria_dto;
mod conta_dto;
mod fluxo_regra_dto;
mod grupo_despesa_dto;
mod optional_lazy;
mod lancamento_dto;
mod lazy;
mod nova_regra_dto;
mod regra_dto;
mod tipo_despesa_dto;
mod tipo_fluxo_dto;

pub use banco_dto::Banco;
pub use categoria_dto::Categoria;
pub use conta_dto::Conta;
pub use fluxo_regra_dto::FluxoRegra;
pub use grupo_despesa_dto::GrupoDespesa;
use hex;
pub use lancamento_dto::Lancamento;
pub use lazy::{Lazy, LazyFn};
pub use optional_lazy::{OptionalLazy, OptionalLazyFn};

pub use nova_regra_dto::NovaRegra;
pub use regra_dto::Regra;
use sha1::{Digest, Sha1};
pub use tipo_despesa_dto::TipoDespesa;
pub use tipo_fluxo_dto::TipoFluxo;
fn gerar_sha1(valor: String) -> String {
    let mut hasher = Sha1::new();
    hasher.update(valor.trim().to_lowercase());
    hex::encode(hasher.finalize())
}

pub trait Unico {
    fn gerar_id(&mut self);
}

pub trait CSV {
    fn from_csv(value: String) -> Self;
    fn from_csv_vec(value: Vec<String>) -> Self;
    fn to_csv(&self) -> String;
}
