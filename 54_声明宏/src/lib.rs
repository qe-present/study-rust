// my_vec
#[macro_export]
macro_rules! my_vec {
    ($($x:expr),+$(,)?) => {
        {
            <[_]>::into_vec(Box::new([$($x),*]))
        }
    };
    () => {
        Vec::new()
    };
    ($ele:expr; $n:expr) => {
        std::vec::from_elem($ele, $n)
    }

}
