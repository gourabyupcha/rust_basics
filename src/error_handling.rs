use std::fs::File;



fn main() {
    println!("Here");
    // panic!("Hello");

    let a=[1,2,3,4,5];
    // a[10];

    // creating an error
    let f = File::open("image.jpg");
    // println!("{:?}", f);

    // handling the error
    match f {
        Ok(f) => {
            println!("This is the file, {:?}", f);
        }
        Err(e) => {
            println!("NO File found \n{:?}", e);
        }
    }


    // unwrap implementation
    // without unwrap this returns -> OK(true)
    // with unwrap this returns -> true

    let result = is_even(10).unwrap();
    println!("result is {}",result);
    println!("end of main");
     
    fn is_even(no:i32)->Result<bool,String> {
        if no%2==0 {
            return Ok(true);
        } else {
            return Err("NOT_AN_EVEN".to_string());
        }
    }

    //expect
    let f2 = File::open("image.jpg").expect("File is not found");
    println!("{:?}", f2);


}