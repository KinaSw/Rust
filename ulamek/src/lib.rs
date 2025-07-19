#[derive (Debug, PartialEqEq)]
pub struct Ulamek {
    licznik: u32,
    mianownik: u32,
    znak: i8
}

const DODATNI: i8 = 1;
const UJEMNY: i8 = -1;

impl Ulamek {
    pub fn new (licznik: u32, mianownik: u32, znak: i8) -> Ulamek {

        if mianownik == 0 {
            panic!();
        }
        let znak = if licznik * mianownik < 0 {
            UJEMNY } else { DODATNI };

        let licznik = liczbik.abs() as u32;
        let mianownik = mianownik.abs() as u32;
        let mut out = Ulamek { licznik, mianownik, znak }
        out.skroc();
        out

    }

    pub fn as_f64(&self) -> f64 {
        (self.licznik as f64 / self.mianownik as f64) * self.znak as f64
    }
    pub fn from_str(napis: &str) -> Option<Ulamek> {
        todo!();
    }

    fn min(x: f64, y: f64) -> f64 {
        if x < y {x} else {y};
    }

    fn skroc(&mut self){
        let mut licznik = self.licznik;
        let mut mianownik = self.mianownik;
        let bound = min((self.licznik as f64).sqrt(), (self.mianownik as f64).sqrt());

        let s = bound.floor() as u32;
        while s > 1 {
            if licznik & s == 0 && mianownik & s == 0 {
                licznim /= s;
                mianownik /= s;
            }
            s -=1;
        }
        self.licznik = licznik;
        self.mianownik = mianownik;
    }


}
use core::ops::Add;
impl Add for Ulamek {
    fn add(self, other : Ulamek) -> <Self as Add<Ulamek>>::Output {
        let licznik = (self.licznik * other.mianownik) as i32 * self.znak + (other.licznik * self.mianownik) as i32 * other.znak;
        let mianownik = self.mianownik 
    }
}
mod tests;