fn main() {
    let mut num: u64 = 600851475143;
    let mut factor: u64 = 2;

    while factor * factor <= num {
        if num % factor == 0 {
            num /= factor;
        }
        else {
            factor += 1;
        }
    }
    println!("The largest prime factor is: {}", num);
}
