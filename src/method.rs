// method in structure
struct Rectangle {
    width: u32,
    height: u32,
}

// calculate area
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let small = Rectangle {
        width: 10,
        height: 10,
    };

    println!("Width is {} \nHeight is {}, \nArea of the rectangle is {}", small.width, small.height, small.area())
}


// Another way (Static Method) -> Point{Rectangle here} Structure

impl Rectangle {
    // static method that creates objects of the Point structure
    fn get_dimensions(a:u32, b:u32) -> Rectangle {
        Rectangle { width: a, height:b }
    }

    //display values of the structure's field
    fn display(&self) -> u32 {
        let area = self.width * self.height;
        area
    }
}

fn main() {
    let r1 = Rectangle::get_dimensions(10, 20);
    println!("\nThe area of the rectangle is {} \n", r1.display())
}

