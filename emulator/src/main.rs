fn main() {
    println!("Hello, world!");
    println!("multiply 3 and 4: {}", mult(3, 4));
    println!("factorial of 5: {}", fac(5));
}

fn mult(x: i32, y: i32) -> i32 {
    let mut sum = 0;
    let mut counter = 0;
    while counter != x {
        sum += y;
        counter += 1;
    }
    sum
}

/// Factorial using only addition and while loop
fn fac(n: i32) -> i32 {
    let mut counter = 1;
    let mut inner_counter = 0;
    let mut sum = 0;
    while counter != n {
        while inner_counter != counter {
            sum += inner_counter;
            inner_counter += 1;
        }
        counter += 1;
    }
    sum
}
