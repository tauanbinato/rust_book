struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

#[derive(Debug)] // This is the basic implementation of the Debug trait for the Rectangle struct.
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    // This is a method because it takes a reference to self as the first parameter.
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        // We can use the &self syntax to get a current reference to the struct instance.
        // notice we use . to access width even though its a reference, this is because Rust automatically dereferences the reference for us.
        self.width > other.width && self.height > other.height
    }
}

// We can have multiple impl blocks for the same struct.
impl Rectangle {
    // Because this is an associated function, we don't use the &self syntax, because it doesn't have an instance of the struct to work with.
    // This is similar to a static method in other languages.
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    // We cannot make the members of the struct mutable individually, we have to make the whole struct mutable.
    let mut user1: User = User {
        email: String::from("bob@gemil.com"),
        username: String::from("bob"),
        active: true,
        sign_in_count: 1,
    };

    let name: String = user1.username;
    // let name2: String = user1.username; // Note that this will throw an error because the value has been moved, the ownership rule applies to struct members as well.
    // Because we moved the value of user1.username to name, we cannot use user1.username anymore. Unless we assign a new value to it.
    user1.username = String::from("bobby"); // Now we assign a new value to the username member of the struct.
    println!("The username is: {}", user1.username);
    println!("The username is: {}", name);

    // We can also create a new struct with the same values as user1, using the struct update syntax.
    let _user2: User = User {
        email: String::from("teste@gmail.com"),
        username: String::from("test"),
        ..user1
    };

    // Tuple structs
    // Tuple structs have the added meaning the struct name provides, but don't have named fields.
    struct Color(i32, i32, i32);
    let _black: Color = Color(0, 0, 0);

    let rec: Rectangle = Rectangle {
        width: 30,
        height: 50,
    };
    println!("rect is: {:#?}", rec); // We can print it because we implemented the Debug trait for the Rectangle struct.
    println!("The area of the rectangle is: {} ", rec.area());

    let rec2: Rectangle = Rectangle {
        width: 10,
        height: 40,
    };

    // We can use the associated function to create a new instance of the Rectangle struct.
    let rec3: Rectangle = Rectangle::square(20);

    println!("Can rec hold rec2? {}", rec.can_hold(&rec2)); // Can_Hold expects a reference to a Rectangle struct, this way we don't lose ownership of the struct.
    println!("Can rec hold rec3? {}", rec.can_hold(&rec3));
}
