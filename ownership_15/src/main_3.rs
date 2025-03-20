/*
3 别名和可变性
 */
fn add_big_strings(dst:&mut Vec<String>,src:&[String]){
    let largest=dst.iter().max_by_key(|x|x.len()).unwrap();
    for s in src {
        if(s.len() > largest.len()){
            dst.push(s.clone());
        }
    }
}
/*
fn add_big_strings_alter(dst:&mut Vec<String>,src:&[String]){
    let largest_len=dst.iter().max_by_key(|x|x.len()).unwrap().len();
    for s in src {
        if(s.len() > largest_len){
            dst.push(s.clone());
        }
    }
}
 */