#[cfg(test)]
mod tests {
    use std::cell::LazyCell;
    use std::sync::LazyLock;
    use crossbeam::scope;
    #[test]
    fn lazy_cell_works() {
        let lazy=LazyCell::new(||{
            println!("hello world");
            12
        });
        println!("+++++++++++");
        println!("{:?}", *lazy);
        println!("{:?}", *lazy);
    }
    #[test]
    fn lazy_lock_works() {
        static NUMBERS:LazyLock<i32>=LazyLock::new(||{
            println!("hello world");
            100
        });
        println!("+++++++++++");
        let _=scope(|scope| {
            let _=(0..5).map(|_|{
                scope.spawn(|_| {
                    println!("the number {}",*NUMBERS);
                });
            }).collect::<Vec<_>>();
        });



    }
}
