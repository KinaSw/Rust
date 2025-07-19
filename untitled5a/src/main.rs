//z1
fn zamien_syst8_na_syst2(z: &str) -> Option<String> {
    if z.is_empty() {
        return None;
    }
    let mut wynik = String::new();

    for znak in z.chars() {
        let cyfra = znak.to_digit(8)?;
        let bin = format!("{:03b}", cyfra);
        wynik.push_str(&bin);
    }

    let wynik = wynik.trim_start_matches('0');
    Some( if wynik.is_empty() { "0".to_string()} else {wynik.to_string()})
}

//z2
fn wartosc_syst2(z: &str) -> Option<u8> {
    if z.is_empty() || z.len() > 8{
        return None;
    }
    let mut wynik: u8 =0;

    for znak in z.chars() {
        let bit = match znak {
            '0' => 0,
            '1' => 1,
            _ => return None,
        };

        wynik = wynik.checked_mul(2)?;
        wynik = wynik.checked_add(bit)?;
    }
    Some(wynik)
}

//z3
fn wartosc_syst8(z: &str) -> Option<u8> {
    if z.is_empty(){
        return None;
    }

    let mut wynik: u8 = 0;
    for znak in z.chars() {
        if !znak.is_digit(8){
            return None;
        }

        let cyfra = znak.to_digit(8).unwrap() as u8;

        match wynik.checked_mul(8){
            Some(wartosc) => wynik = wartosc,
            None => return None,
        }

        match wynik.checked_add(cyfra){
            Some(wartosc) => wynik = wartosc,
            None => return None,
        }
    }
    Some(wynik)
}

fn wartosc_syst8_v2(z: &str) -> Option<u8> {
    if z.is_empty(){
        return None;
    }
    let mut wynik: u8 = 0;
    for znak in z.chars() {
        let cyfra = znak.to_digit(8)? as u8;
        wynik = wynik.checked_mul(8)?;
        wynik = wynik.checked_add(cyfra)?;
    }
    Some(wynik)
}

fn main() {

    assert_eq!(zamien_syst8_na_syst2("17"), Some("1111".to_string())); // 1→001, 7→111 → 001111 → "1111"
    assert_eq!(zamien_syst8_na_syst2("0"), Some("0".to_string()));
    assert_eq!(zamien_syst8_na_syst2("123"), Some("1010011".to_string()));
    assert_eq!(zamien_syst8_na_syst2(""), None);                        // pusty string
    assert_eq!(zamien_syst8_na_syst2("89"), None);                    
    assert_eq!(zamien_syst8_na_syst2("1a"), None);                     

    assert_eq!(wartosc_syst2("0"), Some(0));
    assert_eq!(wartosc_syst2("1"), Some(1));
    assert_eq!(wartosc_syst2("1010"), Some(10));
    assert_eq!(wartosc_syst2("11111111"), Some(255));
    assert_eq!(wartosc_syst2("100000000"), None);
    assert_eq!(wartosc_syst2(""), None);
    assert_eq!(wartosc_syst2("10102"), None);

    assert_eq!(wartosc_syst8("10"), Some(8));
    assert_eq!(wartosc_syst8("377"), Some(255));
    assert_eq!(wartosc_syst8("400"), None);
    assert_eq!(wartosc_syst8("xyz"), None);
    assert_eq!(wartosc_syst8(""), None);

    assert_eq!(wartosc_syst8_v2("10"), Some(8));
    assert_eq!(wartosc_syst8_v2("377"), Some(255));
}
