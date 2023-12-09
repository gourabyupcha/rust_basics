fn main() {
    let mut v= Vec::new();
    // push into vector
    v.push(20);
    v.push(30);
    v.push(40);

    // using vec! macro
    let mut v2 = vec![10,20,30,40,50];
    

    println!("\n");
    println!("--------------");
    println!("Length of the vector is {}", v.len());
    println!("Length of the vector is {}", v2.len());
    println!("The vector is{:?}", v);
    println!("The vector is{:?}", v2);



    //remove from v2
    v2.remove(0);
    println!("The vector is{:?}", v2);
    if v2.contains(&90) {
        println!("Found 20");
    } else {
        println!("Not found");
    }

    for i in &v2 {
        println!("{}", i);
    }
    println!("{:?}", v2)






}