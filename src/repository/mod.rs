mod ofx;
mod file;
mod categorizador;
mod lancamento;
mod regra;
mod categoria;

pub use file::arq_ler_windows_1252;

trait SubVec<T> {
    fn sub_vec(self) -> Self;
}

impl<T>  SubVec<T> for Vec<T> {
    fn sub_vec(mut self) -> Self{
        self.remove(0);
        self
    }
} 