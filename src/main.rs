fn add(a:i32,b:i32) -> i32 {
    a+b
}

#[test]
fn test_addition(){
    assert_eq!(add(2,3),5);
}

fn main() {
    let result = add(3,5);
    println!("the result is: {}",result);
}
