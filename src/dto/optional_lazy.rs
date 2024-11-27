#[derive(Debug, Clone, Default)]
pub enum OptionalLazy<T>{
    #[default]
    None,
    Id(String),
    Some(T)
}

pub trait OptionalLazyFn<T>{
    fn id(&self) -> Option<String>;
    fn some(&self) -> Option<T>;
}