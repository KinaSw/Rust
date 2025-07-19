//z1
fn co_drugi_znak(napis: &str) -> String {
    napis.chars().step_by(2).collect()
}

//z2
fn szyfruj(napis: &str, klucz: usize) -> String {
    napis
        .chars().collect::<Vec<_>>()
        .chunks(klucz)
        .map(|chunk| chunk.iter().rev().collect::<String>())
        .collect()
}

//z3
fn wizytowka(imie: &str, nazwisko: &str) -> String {
    let pierwsza_imie = imie.chars().next().unwrap().to_uppercase().to_string();
    let nazwisko = {
        let mut znaki = nazwisko.chars();
        match znaki.next() {
            Some(pierwsza) => format!(
                "{}{}",
                pierwsza.to_uppercase(),
                znaki.as_str().to_lowercase()
            ),
            None => String::new(),
        }
    };
    format!("{}. {}", pierwsza_imie, nazwisko)
}

//z4
fn na_rzymskie(mut liczba: u32) -> String {
    let rzymskie = [
        (1000, "M"),
        (900, "CM"),
        (500, "D"),
        (400, "CD"),
        (100, "C"),
        (90, "XC"),
        (50, "L"),
        (40, "XL"),
        (10, "X"),
        (9, "IX"),
        (5, "V"),
        (4, "IV"),
        (1, "I"),
    ];
    let mut wynik = String::new();

    for &(wartosc, symbol) in rzymskie.iter() {
        while liczba >= wartosc {
            wynik.push_str(symbol);
            liczba -= wartosc;
        }
    }
    wynik
}

//z5
fn dodaj_pisemnie(a: &str, b: &str) -> String {
    let mut wynik = String::new();
    let mut przeniesienie = 0;
    let mut a_iter = a.chars().rev();
    let mut b_iter = b.chars().rev();

    loop{
        let cyfra_a = a_iter.next().and_then(|c| c.to_digit(10));
        let cyfra_b = b_iter.next().and_then(|c| c.to_digit(10));

        if cyfra_a.is_none() && cyfra_b.is_none() && przeniesienie == 0 {
            break;
        }
        let suma = cyfra_a.unwrap_or(0) + cyfra_b.unwrap_or(0) + przeniesienie;
        przeniesienie = suma / 10;
        let cyfra = suma % 10;

        wynik.push(std::char::from_digit(cyfra, 10).unwrap());
    }

    wynik.chars().rev().collect()
}
fn main() {
    let oryginal = "pizdeczka";
    let co_drugi = co_drugi_znak(oryginal);
    println!("Co drugi z '{}': '{}'", oryginal, co_drugi);

    println!("{}", szyfruj("Aladyn", 2));  // "lAdany"
    println!("{}", szyfruj("Aladyn", 3));  // "alAnyd"
    println!("{}", szyfruj("Aladyn", 4));  // "dalAny"
    println!("{}", szyfruj("Aladyn", 5));  // "ydalAn"
    println!("{}", szyfruj("koza", 3));    // "zoka"
    println!("{}", szyfruj("kaszanka", 3));// "saknazak"
    println!("{}", szyfruj("kot Mruczek", 9)); // "zcurM tokke"

    println!("{}", wizytowka("jan", "KOWALSKI"));     // J. Kowalski
    println!("{}", wizytowka("bARBARA", "noWAK"));    // B. Nowak
    println!("{}", wizytowka("ęryk", "gĄska"));       // Ę. Gąska

    assert_eq!(na_rzymskie(3), "III");
    assert_eq!(na_rzymskie(9), "IX");
    assert_eq!(na_rzymskie(19), "XIX");
    assert_eq!(na_rzymskie(1910), "MCMX");

    println!("{}", na_rzymskie(3));
    println!("{}", na_rzymskie(9));

    assert_eq!(dodaj_pisemnie("1", "3"), "4");
    assert_eq!(dodaj_pisemnie("8", "3"), "11");
    assert_eq!(dodaj_pisemnie("10", "23"), "33");
    assert_eq!(dodaj_pisemnie("998", "7"), "1005");
    assert_eq!(
        dodaj_pisemnie("5924729874298749827418582", "6782893629472094209740"),
        "5931512767928221921628322"
    );
}
