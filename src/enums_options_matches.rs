#[derive(Debug)]
enum SelectGender {
    Male,
    Female,
}
#[derive(Debug)]
enum GenderCatagory {
    Name(String),
    UserId(i32)
}

enum CarType {
    Sedan,
    SUV,
    HatchBack
}


#[derive(Debug)]
enum CustomeOption<T> {
    Some(T),
}

#[derive(Debug)]
struct Person {
    name: String,
    gender: SelectGender,
}


fn main() {

    println!("-----------------");
    println!("******");
    println!("Struct and Enum Implementation");
    let person1:Person = Person {
        name: String::from("Gourab Majumder"),
        gender: SelectGender::Male
    };
    let person2:Person = Person{
        name: String::from("Niladri BHattacharjee"),
        gender: SelectGender::Male,
    };
    println!("{:?}", person1);
    println!("{:?}", person2);
    println!("******");

    println!("\n");

    println!("******");
    println!("Custom Options Implementation");
    let res: CustomeOption<bool> = check_even(5);
    println!("{:?}", res);
    println!("******");

    println!("\n");

    println!("******");
    println!("Match and Enum Implementation");
    match_car_size(CarType::HatchBack);
    match_car_size(CarType::SUV);
    match_car_size(CarType::Sedan);
    println!("******");

    println!("\n");

    println!("******");
    println!("Match and Option Implementation");
    let num:usize = 2;
    match check_even(num) {
        CustomeOption::Some(data) => {
            if data == true {
                println!("{} is an Even number", num);
            } else if data == false {
                println!("{} is an Odd number", num);
            }
        }
    }
    println!("******");

    println!("\n");

    println!("******");
    println!("Match and Enum Implementation");
    let p1 = GenderCatagory::Name(String::from("Gourab Majumder"));
    let p2 = GenderCatagory::UserId(100);
    println!("{:?}", p1);
    println!("{:?}", p2);

    match p1 {
        GenderCatagory::Name(val) => {
            println!("{}", val)
        }
        GenderCatagory::UserId(val) => {
            println!("{}", val)
        }
    }
    println!("******");

    println!("-----------------");

}


fn check_even(no: usize) -> CustomeOption<bool> {
    if no%2 == 0 {
        CustomeOption::Some(true)
    } else {
        // Some("Not an even number")
        CustomeOption::Some(false)
    }

}

fn match_car_size(car: CarType) {
    match car {
        CarType::HatchBack => {
            println!("The car size is small")
        }
        CarType::Sedan => {
            println!("The car size is medium")
        }
        CarType::SUV => {
            println!("The car size is Large")
        }
    }
}