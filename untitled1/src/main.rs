//z1
fn czy_przestepny(rok: u32){
    if((rok % 4 == 0) && (rok % 100 != 0)) || (rok % 400 == 0){
        println!("Przestepny");
    } else {
        println!("Nieprzestepny");
    }
}

//z6
fn silnia(liczba: u64) -> u64{
    let mut wynik = 1;
    for i in 2..=liczba{
        wynik *= i;
    }
    wynik
}

//z6 rekurencyjnie
fn silnia_rek(liczba:u64) -> u64{
    if liczba <= 1 {
        1
    }else {
        liczba * silnia_rek(liczba - 1)
    }
}

//z7
fn cyfry(mut liczba: u64){
    let mut wynik = 0;
    while liczba > 0{
        let cyfra = liczba % 10;
        wynik *= 10;
        wynik += cyfra;
        liczba /= 10;

    }
    println!("Wynik: {}", wynik);
}

//z8

fn suma_cyfr(mut liczba: u64){
    let mut wynik = 0;
    while liczba > 0{
        let cyfra = liczba % 10;
        wynik += cyfra;
        liczba /= 10;
    }
    println!("Wynik: {}", wynik);
}

fn main() {
    czy_przestepny(2025);
    println!("{}",silnia(6));
    println!("{}", silnia_rek(6));
    cyfry(2025);
    suma_cyfr(2025);
}
