fn main() {
    // ----- Variables and Mutability
    let x: i32 = 5;
    let x: i32 = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    // ----- Scalar types (integers, floating-point numbers, Booleans, and characters)
    let _x = 2.0; // f64
    let _y: f32 = 3.0; // f32
    let _t = true;
    let _c = 'z';
    let _z: char = 'ℤ'; // with explicit type annotation
    let _heart_eyed_cat = '😻';

    // -----  Compound types (tuples and arrays)
    // Compound types can group multiple values into one type.

    // Arrays have fixed length: once declared, they cannot grow or shrink in size.
    // and must hold the same type.
    let _array: [i32; 5] = [1, 2, 3, 4, 5];

    // Tuples has fixed length: once declared, they cannot grow or shrink in size.
    // but can hold different types.
    let _tup: (i32, f64, u8) = (500, 6.4, 1);

    // Destructuring to get the values out of a tuple
    let (_x, _y, _z) = _tup;
    // Or access the values directly by using a period (.) followed by the index of the value we want to access.
    let _five_hundred = _tup.0;

    println!("The value of y is: {_y}");
}
