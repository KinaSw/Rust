#[derive(Debug, Clone, PartialEq)]
enum Jednostka{
    Sztuki,
    Litry,
    Kilogramy,
}

#[derive(Debug, Clone, PartialEq)]
enum Warunki{
    Zamrazarka,
    Chlodziarka,
    Normalne,
}

#[derive(Debug, Clone, PartialEq)]
struct Towar{
    nazwa: String,
    jednostka: Jednostka,
    waga: f64,
    warunki:Warunki,
}

#[derive(Debug, Clone, PartialEq)]
struct PozycjaZamowienia{
    towary: Towar,
    ilosc: f64,
}

#[derive(Debug, Clone, PartialEq)]
struct Zamowienie{
    pozycje_zamowienia: Vec<PozycjaZamowienia>,
}

impl Towar{
    fn nowy(nazwa: String, jednostka: Jednostka, mut waga: f64, warunki: Warunki) -> Result<Self, String>{
        match jednostka{
            Jednostka::Kilogramy => {
                waga = 1.0;
            }
            _ => {
                if waga <= 0.0{
                    return Err("Waga powinna byc wieksza od 0".to_string());
                }
            }
        }
        Ok( Towar {nazwa, jednostka, waga, warunki})
    }
}

impl Zamowienie {
    fn nowy() -> Self{
        Zamowienie { pozycje_zamowienia: Vec::new() }
    }

    fn suma_wag(&self) -> f64 {
        self.pozycje_zamowienia.iter()
            .map(|p| p.towary.waga * p.ilosc)
            .sum()
    }

    fn suma_wag_przechowywanie(&self, warunki: Warunki) -> f64 {
        self.pozycje_zamowienia.iter()
            .filter(|p| p.towary.warunki == warunki)
            .map(|p| p.towary.waga * p.ilosc)
            .sum()
    }

    fn dodaj_zamowienie(&mut self, towar: Towar, ilosc: f64) -> Result<(), String>{
        if ilosc <= 0.0 {
            return Err("Ilosc powinna byc wieksza od 0".to_string());
        }
        if let Jednostka::Sztuki = towar.jednostka {
            if ilosc.fract() != 0.0 {
                return Err("Ilosc sztuk powinna byc calkowita".to_string());
            }
        }

        if let Some(pozycja) = self.pozycje_zamowienia.iter_mut().find(|p| p.towary.nazwa == towar.nazwa){
            pozycja.ilosc += ilosc;
        } else {
            self.pozycje_zamowienia.push(PozycjaZamowienia { towary: towar, ilosc });
        }

        Ok(())
    }
}
fn main() {

    let mut zamowienie = Zamowienie::nowy();

    let t1 = Towar::nowy("mleko".to_string(), Jednostka::Litry, 3.5, Warunki::Chlodziarka).unwrap();
    let t2 = Towar::nowy("Pizza".to_string(), Jednostka::Kilogramy, 0.5, Warunki::Zamrazarka).unwrap();
    let t3 = Towar::nowy("Ciastka".to_string(), Jednostka::Sztuki, 3.0, Warunki::Normalne).unwrap();


    zamowienie.dodaj_zamowienie(t1, 3.0);
    zamowienie.dodaj_zamowienie(t2, 2.0);
    zamowienie.dodaj_zamowienie(t3, 1.0);

    println!("Calkowita waga {}", zamowienie.suma_wag());
    println!("Waga w chlodziarce {}", zamowienie.suma_wag_przechowywanie(Warunki::Chlodziarka));
    println!("Waga w zamrazarce {}", zamowienie.suma_wag_przechowywanie(Warunki::Zamrazarka));
    println!("Waga normalne {}", zamowienie.suma_wag_przechowywanie(Warunki::Normalne));

    println!("Pelne zamowienie {:?}", zamowienie);

}
