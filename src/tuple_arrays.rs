
fn main() {

    // tuples
    
    let tuple:(i32, f64, &str, bool) = (450, 80.32, "Gourab Majumder", true);
    destruct_tuple(tuple);

    // arrays using tuples
    let arr:[i32;6] = [10,20,30,40,50,5];
    let x: (i32, i16) = find_smallest_(arr);
    println!("Smallest of the array, {} with index {}", x.0, x.1);

    //arrays using Option and Some
    let new_arr = [10,20,30,40,50,5];
    if let Some((index, smallest_val)) = find_smallest__(&new_arr) {
        println!("Smallest Val: {}", smallest_val);
        println!("Index: {}", index);
    } else {
        println!("Empty array !!")
    }
    
}

fn destruct_tuple(x:(i32, f64, &str, bool)) {
    println!("---Tuple---");
    let (marks, percentage, name, passed) = x;
    println!("Name: {}", name);
    println!("Total marks obtained: {}", marks);
    println!("Percentage obtained: {}", percentage);
    println!("Status: {}", passed);
    println!("-----------");
}


// using tuple
fn find_smallest_(arr:[i32;6]) -> (i32, i16) {

    let mut smallest:i32 = arr[0];
    let mut i:usize = 0;
    for (index, val) in arr.iter().enumerate(){
        if val < &smallest {
            smallest = *val;
            i = index;
        }
    }
    (smallest, i.try_into().unwrap())
}

// using Option & Some
fn find_smallest__(new_arr:&[i32]) -> Option<(usize, i32)> { 

    if new_arr.is_empty() {
        return None;
    }

    let mut smallest = new_arr[0];
    let mut i = 0;

    for (index, val) in new_arr.iter().enumerate() {
        if val < &smallest {
            smallest = *val;
            i = index;
        }
    }
    Some((i, smallest))
}