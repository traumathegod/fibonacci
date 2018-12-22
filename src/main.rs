use std::io;

fn main() {
    println!("Fibonacci nTh printer");

    fibonacci(get_n());
}

fn get_n() -> u128 {
    loop {
        let _n: u128;

        println!("Insert n");
        let mut n: String = String::new();
        io::stdin().read_line(&mut n)
            .expect("Failed to read line");

        _n = match n.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        if _n > 184 {
            println!("n needs to be smaller than 185");
            continue;
        }
        else {return _n}
    }
}

fn fibonacci(mut int_n: u128) {
    let mut f0: u128;
    let mut f1: u128 = 0;
    let mut buff: u128 = 1;
    print!("{} ", f1);
    f1 = 1;
    while int_n > 0  {
        print!(",{} ", f1);
        f0 = f1;
        f1 = f1 + buff;
        buff = f0;
        int_n -= 1;
    }
}
