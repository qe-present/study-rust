/*
 2、没有足够的权限
 */
fn stringify_name_with_title(name:&Vec<String>)->String {
    name.push(String::from("hello"));
    let full=name.join(" ");
    full
}
// 修改
// fn stringify_name_with_title_alter(name:&Vec<String>)->String {
//     let full=name.join(" ");
//     full.push(String::from("hello"));
//     full
// }