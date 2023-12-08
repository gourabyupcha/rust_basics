#[derive(Debug)]

struct Users {
    id: u16,
    name: String,
    company: String,
    age: u32
}

fn main() {

    let c:&str = "Yupcha Softwares";

    let users_object:Vec<Users> = vec![
        Users {
            id: 1,
            company: c.to_string(),
            name:String::from("Gourab Majumder "),
            age:24,
        },

        Users {
            id: 2,
            company: c.to_string(),
            name: String::from("Niladri Bhattacharjee"),
            age:27,
        },

    ];

    for obj in users_object.iter() {
        print!("\n");
        println!("{}. This is {} of age {} is currently working at {}", obj.id, obj.name, obj.age, obj.company);
        print!("\n");
    }
    
    who_is_elder(&users_object);

}


fn who_is_elder(vector: &Vec<Users>) {
    
    let elder_user = vector.iter().max_by_key(|user| user.age);

    match elder_user {
        Some(user) => {
            println!("The elder user is {:?}", user.name);
        }
        None => {
            println!("Unable to determine elder user");
        }
    }

}


