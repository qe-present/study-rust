
#[cfg(test)]
mod tests {
    use std::cell::OnceCell;
    use std::sync::atomic::{AtomicU32, Ordering};
    use std::sync::OnceLock;
    use crossbeam::scope;
    use std::thread;
    struct  OnceList<T>{
        data:OnceLock<T>,
        next:OnceLock<Box<OnceList<T>>>

    }
    impl <T> OnceList<T> {
        const fn new()->OnceList<T>{
            OnceList{
                data:OnceLock::new(),
                next:OnceLock::new(),
            }
        }
        fn push(&self,val:T){
            if let Err(val) = self.data.set(val) {
                // Current node is already occupied, move to next node
                let next_node = self.next.get_or_init(|| {
                    Box::new(OnceList::new())
                });
                // Recursively push to the next node
                next_node.push(val);
            }
        }
        fn contains(&self,val:&T)->bool
        where T : PartialEq
        {
            self.data
                .get()
                .map(|item| item == val)
                .filter(|v|*v)
                .unwrap_or_else(||{
                    self.next
                        .get()
                        .map(|next_node|next_node.contains(val))
                        .unwrap_or(false)
                })

        }
    }


    #[test]
    fn test_once_cell(){
        let cell = OnceCell::new();
        assert!(cell.get().is_none());
        let result=cell.set(String::from("hello world"));
        assert!(result.is_ok());
        let result=cell.set(String::from("hello world2"));
        assert!(result.is_err());
    }
    #[test]
    fn test_once_cell_get_init(){
        let cell = OnceCell::new();
        assert!(cell.get().is_none());
        let result=cell.get_or_init(||{
            "hello world".to_string()
        });
        assert_eq!("hello world", result);
    }
    #[test]
    fn test_once_cell_get_mut(){
        let mut cell = OnceCell::new();
        cell.get_or_init(||{
            "hello world".to_string()
        });
        if let Some(v) = cell.get_mut() {
            v.push_str("!");
        }
        let _ =cell.set(String::from("hello world2"));
        println!("{}", cell.get_mut().unwrap());

    }
    #[test]
    fn test_once_lock(){

        static LOCK:OnceLock<usize>=OnceLock::new();
        assert!(LOCK.get().is_none());
        let _=scope(|scope| {
            scope.spawn(|_| {
                let value=LOCK.get_or_init(||12345);
                assert_eq!(*value, 12345);
            });
        });
        assert_eq!(LOCK.get(), Some(&12345));
    }
    #[test]
    fn test_once_lock_get(){
        static LIST:OnceList<u32>=OnceList::new();
        static COUNTER:AtomicU32=AtomicU32::new(0);
        const LEN:u32=1000;
        let _=scope(|scope| {
            for _ in 0..thread::available_parallelism().unwrap().get()  {
                scope.spawn(|_| {
                    while let i @0..LEN=COUNTER.fetch_add(1,Ordering::Relaxed) {
                        LIST.push(i)
                    }
                });
            }
        });
        for i in 0..LEN {
            assert!(LIST.contains(&i))
        }

    }

}