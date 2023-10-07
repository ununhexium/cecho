#[macro_export]
macro_rules! vecs {
    ($($x:expr),*) => (vec![$($x.to_string()),*]);
}

pub(crate) use vecs;
