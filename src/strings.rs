
fn main() {
    let _static: &str = "This is a static typed string";
    let converted_string: String = convert_to_string(12345);
    let company: String = fn_input_string();
    let employee1:String = String::from("Gourab Majumder");
    let mut employee2:String = String::from("Niladri Bhatt");
    // employee2 = String::from(update_string());
    employee2 = String::from(replace_string(employee2));
    let mut location:String = String::from("Parimal Bhavan");
    location = String::from(push_string(location));
    // trim_string(_static);
    // split_wshitespace(_static);
    split_string();
    let cofounders:String = concat_twostrings(String::from("Palash sir"), String::from("Shimul Sir"));
    println!("--------");
    println!("{}", cofounders);
    println!("1. {}", _static);
    println!("2. Name of the Company is: {} and is located at {}, length of location {} ", company, location, location.len());
    println!("3. Converted string is: {}", converted_string);
    println!("4. The employees are: i.{}, ii.{}", employee1, employee2);
    println!("--------");
}


fn fn_input_string() -> String {
    let content_string = String::from("Yupcha Softwares");
    content_string
}

fn convert_to_string(a:i32) -> String {
    let converted_string:String = a.to_string();
    converted_string
}

// fn update_string() -> String {
//     "Niladri BHattacharjee".to_string()
// }

fn replace_string(s: String) -> String {
    let correct = s.replace("Bhatt", "Bhattacharjee");
    correct
}

fn push_string(s:String) -> String {
    let mut complete_string:String = s.to_string();
    complete_string.push_str(", Banamalipur, Agartala");
    complete_string
}

fn trim_string(s:&str) {
    println!("-----Before trim-----");
    println!("Length {}", s.len());
    println!("-----After trim-----");
    println!("Length {}", s.trim().len());
}

fn split_wshitespace(s:&str) {
    let string_val:String = s.to_string();
    let mut i = 1;

    for t in string_val.split_whitespace(){
        println!("Token {} {}", i, t);
        i+=1;
    }
}

fn split_string() {
    let string_val = "Gourab, Neil, Ankit";
    for token in string_val.split(" "){
        println!("Token {}", token)
    }

    //store in a vector
    println!("\n");
    let tokens:Vec<&str> = string_val.split(" ").collect();
    println!("First Emp, {}", tokens[0]);
    println!("Second Emp, {}", tokens[1]);
    println!("Third EMp, {}", tokens[2]);
    println!("\n")
}


fn concat_twostrings(a:String, b:String) -> String {
    let c1:String = a;
    let c2:String = b;
    // let c3:String = c1 + " " + &c2;    // using concat
    let c3:String = format!("The confounders are CTO:{} CEO:{}", c1, c2);
    c3
}



