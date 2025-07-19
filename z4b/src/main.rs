fn co_drugi_znak(napis: &str) -> String{
    let mut wynik = String::new();
    for (i,c) in napis.chars().enumerate(){
        if i % 2 == 0{
            wynik.push(c);
        }
    }
    wynik
}

//z2

fn szyfruj(napis: &str, klucz: usize) -> String{
    if klucz < 1 {
        return napis.to_string();
    }
    let znaki: Vec<char> = napis.chars().collect();
    let mut wynik = String::with_capacity(napis.len());

    let mut i = 0;

    while i < znaki.len(){
        let koniec = std::cmp::min(i + klucz, znaki.len());

        for j in (i..koniec).rev(){
            wynik.push(znaki[j]);
        }

        i+=klucz;
    }
    wynik
}

//z3

fn wizytowka(imie: &str, nazwisko: &str) -> String{
    let imie_first = imie.chars().next().unwrap_or(' ').to_uppercase().to_string();
    let nazwisko_processed = nazwisko. to_lowercase();
    let nazwisko_cap = nazwisko_processed.chars().next().unwrap_or(' ').to_uppercase().to_string() + &nazwisko_processed[1..];

    format!("{}. {}", imie_first, nazwisko_cap);
}
fn podzielne_przez_3(tab: Vec<usize>){
    for i in tab{
        if i % 3 == 0{
            println!("{}", i);
        }
    }
}
fn main() {
    //println!("{}", co_drugi_znak("kokosanka"));

    //podzielne_przez_3(vec!(1,2,3,4,5,6,7,8,9));

    println!("{}", szyfruj("alpaka", 2));
}
