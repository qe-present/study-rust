use std::io::stdin;
// loop  循环 break停止loop，continue跳出本次迭代 
// 从loop循环 返回值:break返回值
// loop 标签  比如  'count:loop     

fn main() {

    let mut msg= String::new();
    println!("Please input a number: ");
    stdin().read_line(&mut msg).unwrap();
    let x: i32 = msg.trim().parse().unwrap();
    func1(x);
    println!("number is {}",func2());
    println!("res is {}",func3());
    println!("counter is {}",func4());
    func5();
    func6()
    

}
fn func1(number: i32) {
    if number%4==0{
        println!("number {} is divisible by 4", number);
    }else if  number%3==0  {
        println!("number {} is divisible by 3", number);
    }else if number%2==0  {
        println!("number {} is divisible by 2",number);
    }
}
fn func2()->i32 {
    let condition= true;
    if condition { 5} else { 6 } // 返回类型需要一样
}
fn  func3()->i32 {
    let mut counter = 0;
    let res= loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    res
}
fn func4()->i32{
    let mut counter = 0;
    'counter_up: loop {
        println!("counter is {}",counter);
        let mut remaining = 10;
        loop {
            println!("Remaining is {}",remaining);
            if remaining == 9{
                break;
            }
            if counter==2{
                break 'counter_up;
            }
            remaining -= 1;
        }
        counter += 1;
    }
    counter
}
fn func5(){
    let a=[1,2,3,4,5,6,7,8,9,10];
    for element in a{
        println!("{}",element);
    }
}
fn func6(){
    for number in (1..4).rev(){
        println!("{}",number);
    }
}
