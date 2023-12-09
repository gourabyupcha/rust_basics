#[derive(Debug)]
struct Data<T> {
    value: T,
}


fn main() {
    let t1:Data<i32> = Data{value:32};
    println!("{:?}", t1.value);

    let t2:Data<String> = Data{value: String::from("Tom Tom")};
    println!("{:?}", t2.value);

}