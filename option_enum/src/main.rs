// The option enum only has two variants, Some and None.
// We can wrap any type in the Optional enum.
/* enum Option<T> {
    Some(T),
    None,
} */
fn main() {
    // Defining some variables of type Option<T>
    let _some_number: Option<i32> = Some(5);
    let _some_string: Option<&str> = Some("a string");
    let _absent_number: Option<i32> = None;

    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    // let sum = x + y; // error: mismatched types
    let sum = x + y.unwrap_or(0); // unwrap() returns the value inside the Some variant
    println!("sum: {}", sum);
}
