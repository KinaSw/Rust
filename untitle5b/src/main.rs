//z1
fn wartosc_cyfry(c: char) -> Result<u8, String>{
    if c.is_digit(10){
        Ok(c.to_digit(10).unwrap() as u8)
    } else {
        Err(format!("{} is not a digit", c))
    }
}

//z2
fn dodaj_pisemnie(a: &str, b: &str) -> Result<String, String>{
    if a.is_empty() || b.is_empty() {
        return Err("string is empty lol".to_string());
    }
    let mut wynik = String::new();
    let mut iter_a = a.chars().rev();
    let mut iter_b = b.chars().rev();
    let mut przeniesienie = 0;

    loop {
        let znak_a = iter_a.next();
        let znak_b = iter_b.next();

        if znak_a.is_none() && znak_b
    }
}
fn main() {
    println!("{:?}", wartosc_cyfry('5')); // Ok(5)
    println!("{:?}", wartosc_cyfry('0')); // Ok(0)
    println!("{:?}", wartosc_cyfry('a')); // Err("'a' to nie jest cyfra, geniuszu.")
    println!("{:?}", wartosc_cyfry('#')); // Err("'#' to nie jest cyfra, geniuszu.")
}
