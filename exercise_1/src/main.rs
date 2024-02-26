fn main() {
    println!("Convert Fahrenheit to Celsius!");
    let fahrenheit: f64 = 32.0;
    let celsius: f64 = convert_fahrenheit_to_celsius(fahrenheit);
    println!("{}°F is {}°C", fahrenheit, celsius);

    println!("Generate the nth Fibonacci number!");
    let n: u32 = 4;
    let fibonacci_number: u32 = generate_the_nth_fibonacci_number(n);
    println!("The {}th Fibonacci number is {}", n, fibonacci_number);
}

fn convert_fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}

fn generate_the_nth_fibonacci_number(n: u32) -> u32 {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }
    let mut a = 0;
    let mut b = 1;
    for _ in 2..n {
        let c = a + b;
        a = b;
        b = c;
    }
    b
}
