mod banco_dto;
mod categoria_dto;
mod configuracao_dto;
mod conta_dto;
mod divida_dto;
mod divida_mes_dto;
mod fluxo_regra_dto;
mod grupo_despesa_dto;
mod lancamento_dto;
mod lazy;
mod meta_dto;
mod nova_regra_dto;
mod optional_lazy;
mod parcela_divida_dto;
mod regra_dto;
mod tipo_despesa_dto;
mod tipo_fluxo_dto;
mod dash;

pub use banco_dto::Banco;
pub use categoria_dto::Categoria;
pub use configuracao_dto::Configuracao;
pub use conta_dto::Conta;
pub use divida_dto::Divida;
pub use fluxo_regra_dto::FluxoRegra;
pub use grupo_despesa_dto::GrupoDespesa;
use hex;
pub use lancamento_dto::Lancamento;
pub use lazy::{Lazy, LazyFn};
pub use meta_dto::Meta;
pub use optional_lazy::{OptionalLazy, OptionalLazyFn};
pub use parcela_divida_dto::ParcelaDivida;

pub use divida_dto::DadosDivida;
pub use divida_mes_dto::DividaMes;
pub use nova_regra_dto::NovaRegra;
pub use regra_dto::Regra;
pub use tipo_despesa_dto::TipoDespesa;
pub use tipo_fluxo_dto::TipoFluxo;

pub use dash::*;

use sha1::{Digest, Sha1};
fn gerar_sha1(valor: String) -> String {
    let mut hasher = Sha1::new();
    hasher.update(valor.trim().to_lowercase());
    hex::encode(hasher.finalize())
}

pub trait Unico {
    fn gerar_id(&mut self);
}
