//z1
fn liczba_wystapien( napis: &str, znak: char) -> i32{
    let mut licznik = 0;

    for c in napis.chars(){
        if c == znak{
            licznik += 1;
        }
    }
    licznik
}
//z2
fn rzymskie(napis: & str ) -> u32{

    let mut wynik = 0;
    let mut poprzednia_wartosc = 0;

    for c in napis.chars().rev(){
        let aktualna_wartosc = match c{
            'I' => 1,
            'V' => 5,
            'X' => 10,
            'L' => 50,
            'C' => 100,
            'D' => 500,
            'M' => 1000,
            _ => 0,
        };

        if aktualna_wartosc < poprzednia_wartosc{
            wynik -= aktualna_wartosc;
        } else {
            wynik += aktualna_wartosc;
        }

        poprzednia_wartosc = aktualna_wartosc;
    }


    wynik

}

fn main() {
    //println!("{}", liczba_wystapien("ala ma kota", 'a'));

    println!("{}", rzymskie("IX"));
}
