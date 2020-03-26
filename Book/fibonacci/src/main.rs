fn main() {
    println!("Fib:{}", fib(1, 0, 1));
    println!("Fib:{}", fib(2, 0, 1));
    println!("Fib:{}", fib(3, 0, 1));
    println!("Fib:{}", fib(4, 0, 1));
    println!("Fib:{}", fib(5, 0, 1));
    println!("Fib:{}", fib(9, 0, 1));
    println!("Fib:{}", fib(10, 0, 1));
}

fn fib(n: u64, mut a: u64, mut b: u64) -> u64 {
    if n == 1 {
        return a
    };
    if n == 2 {
        return b
    };
    let mut counter = 3;
    let mut c = a + b;
    while counter < n {
        a = b;
        b = c;
        c = a + b;
        counter += 1;
    }
    c
}
