//borrowing reference

fn main() {
    let v = vec![1,2,3];
    display(&v);
    println!("{}", v[0])
}

fn display(v:&Vec<i32>) {
    println!("inside display {:?}", v)
}

//mutable reference

fn main2() {
    let mut i = 3;
    add_one(&mut i);
    println!("{}", i);
}


fn add_one(e: &mut i32) {
    *e+= 1;
 }