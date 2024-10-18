mod catetogia_dto;
mod grupo_despesa_dto;
mod lancamento_dto;
mod nova_regra_dto;
mod regra_dto;
mod tipo_despesa_dto;
mod tipo_fluxo_dto;

pub use catetogia_dto::Categoria;
pub use grupo_despesa_dto::GrupoDespesa;
use hex;
pub use lancamento_dto::Lancamento;
pub use nova_regra_dto::NovaRegra;
pub use regra_dto::Regra;
use sha1::{Digest, Sha1};
pub use tipo_despesa_dto::TipoDespesa;
pub use tipo_fluxo_dto::TipoFluxo;

fn gerar_sha1(valor: String) -> String {
    let mut hasher = Sha1::new();
    hasher.update(valor);
    hex::encode(hasher.finalize())
}

pub trait DtoIdentificado {
    fn gerar_id(&mut self);
}

trait SubVec<T> {
    fn sub_vec(self) -> Self;
}

impl<T> SubVec<T> for Vec<T> {
    fn sub_vec(mut self) -> Self {
        self.remove(0);
        self
    }
}
