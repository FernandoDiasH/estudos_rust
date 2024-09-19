#[derive(Debug)]
struct Rectangle {
    width:u32,
    height:u32
}

impl Rectangle {
    fn area(&self) -> u32{
       self.width * self.height
    }

    fn square(size: u32) ->self { 
        self {
            width:size,
            height: size,
        }
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle)-> bool{
        return other.width <  self.width && other.height < self.height
    }
}

fn main() {
    let rect1 = Rectangle {
         width: 30,
         height: 50,
    };
    
    println!("rect1 is {rect1:#?}");
    println!(
        "The area of the retangle is {} square pixels",
        rect1.area()
    );
    dbg!(&rect1);
    if rect1.width() {
        println!("Th rectangle has a nonzero width; it is {}", rect1.width);
    }
    
    let rect2 = Rectangle {
        width:30,
        height:50,
    };

    let rect3 = Rectangle {
        width: 20,
        height: 30,
    };

    let rect4 = Rectangle {
        width:60,
        height:40,
    };

    println!("Can rect1 hold react2 {}", rect2.can_hold(&rect3));
    println!("Can rect1 hold react3 {}", rect2.can_hold(&rect4));
}

