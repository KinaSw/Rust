//z1
fn znaki(){
    for i in 33..=126{
        let char = i as u8 as char;
        println!("{}", char);
    }

}

//z2
fn collatz(mut n: u32)->u32{
    let mut steps=0;
    while n!=1{
        if n % 2 == 0 {
            n/=2;
        }else{
            n=3*n+1;
        }
        steps+=1;
    }
    steps
}

fn is_armstrong(n: u64) -> bool {
    let mut temp = n;
    let mut digits = Vec::new();

    while temp > 0 {
        digits.push(temp % 10);
        temp /= 10;
    }

    let num_digits = digits.len() as u32;

    let mut sum = 0;
    for digit in &digits {
        sum += digit.pow(num_digits);
    }

    sum == n
}

fn main() {
    //znaki();
    //println!("{}",collatz(12));
}
