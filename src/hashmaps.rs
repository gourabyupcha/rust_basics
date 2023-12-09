use std::collections::HashMap;



fn main() {
    let mut key_val_pair = HashMap::new();

    // let key_val_1 = ["GM", "Gourab Majumder"];
    // let key_val_2 = ["NB", "Niladri BHattacharjee"];

    // key_val_pair.insert(key_val_1, key_val_2);

    //insert into hashmap
    key_val_pair.insert("GM", "Gourab Majumder");
    key_val_pair.insert("NB", "Niladri BHattacharjee");
    key_val_pair.insert("NB", "Niladri BHattacharjee"); //this doesn't gets added to the collection
    



    // get and match from hashmap
    match key_val_pair.get(&"GM") {
        Some(val) => {
            println!("Value for key GM is {}", val);
        }
        None => {
            println!("No values found");
        }
    }

    // iter function of hashmaps
    for (key, val) in key_val_pair.iter() {
        println!("{} {}", key, val);
    }


    // check if hashmap contains a key
    if key_val_pair.contains_key(&"NB") {
        println!("Found key")
    } else {
        println!("Key not found")
    }

    //remove from hashmap -> pass the key
    key_val_pair.remove(&"NB");
    println!("{}", key_val_pair.len());
    println!("{:?}", key_val_pair);

    println!("\n");
    println!("-------------");

    println!("{:?}", key_val_pair);
    println!("{}", key_val_pair.len());

    println!("-------------");
    println!("\n");

}