fn main() {
    println!("Value of finbonacci {}", fibonacci(4));
}

fn fibonacci(mut n: u64) -> u64 {
    let mut x = 1;
    let mut y = 1;
    let mut z;

    while n != 1 {
        n = n - 1;

        z = x + y;
        x = y;
        y = z;

    }

    y
}
