#[derive(Debug, Clone)]
pub enum Lazy<T>{
    Id(String),
    Some(T)
}

pub trait LazyFn<T>{
    fn id(&self) -> String;
    fn some(&self) -> T;
}