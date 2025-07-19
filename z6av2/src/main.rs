fn krotsze_niz_4(napisy: Vec<String>) -> Vec<String> {
    napisy.iter().filter(|s| s.len() < 4).cloned().collect()
}

fn nie_zawiera_aA(napisy: Vec<String>) -> Vec<String> {
    napisy.iter().filter(|s| !s.contains('a') && !s.contains('A')).cloned().collect()
}

fn zawiera_cyfr(napisy: Vec<String>) -> Vec<String> {
    napisy.iter().filter(|s| s.chars().any(|x| x.is_digit(10))).cloned().collect()
}


fn main() {
    //małe litery alfabetu angielskiego;
    let z1: Vec<char> = ('a'..='z').collect();
    println!("{:?}", z1);

    //kwadraty 10. kolejnych liczb całkowitych począwszy od 1;
    let z2: Vec<u16> = (1..=10).map(|x| x*x).collect();
    println!("{:?}", z2);

    //10 kolejnych potęg dwójki;
    let z3:Vec<u16> = (1..=10).map(|x| 2u16.pow(x)).collect();
    println!("{:?}", z3);

    //odwrotności wszystkich liczb od 1 do 20;
    let z4: Vec<f64> = (1..=20).map(|x|1f64/x as f64).collect();
    println!("{:?}", z4);

    //liczby od 1 do 100 podzielne przez 3, ale niepodzielne przez 4.
    let z5: Vec<u16> = (1..=100).filter(|x| x % 3 == 0 && x % 4 != 0).collect();
    println!("{:?}", z5);

    //wektor zawierającą napisy krótsze niż 4 znaki;
    let napisy: Vec<String> = ["kot", "mleko", "andaluzja", "widłoróg", "megawonsz9"]
        .iter().map(|s| String::from(*s)).collect();
    println!("{:?}", krotsze_niz_4(napisy.clone()));
    println!("{:?}", nie_zawiera_aA(napisy.clone()));
    println!("{:?}", zawiera_cyfr(napisy.clone()));






}
