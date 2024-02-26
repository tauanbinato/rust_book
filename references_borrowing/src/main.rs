// References are non-owning pointers, because they do not own the data they point to.
// References are immutable by default, but can be made mutable by using the mut keyword.
// The scope of a reference starts from the point at which it is introduced and ends in the last time that reference is used.
fn main() {
    // This is an example of the move behavior, but can be inconvenient
    // this will work fine because the ownership of the string is moved to the greet function
    // and then moved back to the main function with tuple destructuring
    // but m1 and m2 are no longer valid after the greet function is called.
    let m1 = String::from("Hello");
    let m2 = String::from("world");
    let (m1_again, m2_again) = greet(m1, m2);
    let s = format!("{} {}", m1_again, m2_again);
    println!("{}", s);

    // This is an example of the borrow behavior, which is more convenient
    // this will work fine because the greet_with_ref function takes references to the strings
    // and the strings are still valid after the greet_with_ref function is called
    // m1 and m2 points to a heap-allocated string, and the ownership of the string is not moved
    // g1 and g2 points to m1 and m2, and the ownership of the string is not moved
    let m1 = String::from("Hello");
    let m2 = String::from("world");
    greet_with_ref(&m1, &m2); // note the ampersands
    let s = format!("{} {}", m1, m2);
    println!("{}", s);

    // This is an example of the borrow behavior with mutable references
    // We can have only one mutable reference to a particular piece of data in a particular scope
    // this is to prevent data races at runtime
    // but we can have multiple immutable references to the same data in a particular scope
    // we also cannot have a mutable reference and an immutable reference to the same data in a particular scope
    let mut s: String = String::from("Mutable Hello");
    let r1: &String = &s;
    let r2: &String = &s;
    // let r3: &mut String = &mut s; // this will not work unless we only use it after the lifetime of r1 and r2
    println!("{} {}", r1, r2);
    // r1 and r2 are no longer valid after this point, because the last time they are used is on the previous line
    let r3: &mut String = &mut s; // This will work because we only use it after the lifetime of r1 and r2, now we can have a mutable reference even though we had immutable references before.
    println!("{}", r3);
    // r3 is no longer valid after this point if we don't use it again, because the last time it is used is on the previous line
}

fn greet(g1: String, g2: String) -> (String, String) {
    (g1, g2)
}

// note the ampersands
fn greet_with_ref(g1: &String, g2: &String) {
    println!("Inside greet with ref: {} {}!", g1, g2);
}
