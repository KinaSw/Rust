//z1
fn czy_przestepny(rok: u32) -> u32{
    if rok % 4 == 0 && rok % 100 !=0 || rok % 400 == 0{
         0
    } else {
        1
    }
}

//z2
fn liczba_dni(miesiac: u32, rok: u32) -> u32{
    if miesiac == 2{
        if czy_przestepny(rok) == 0{
             29

        }else{
             28
        }

    }else if miesiac % 2 == 1 || miesiac == 8{
        return 31;
    }else {
        return 30;
    }

}

//z3
fn c_na_f( c: f32) -> f32{
    let f = 32.0 + 9.0/5.0 * c;
    f
}

//z4
fn f_na_c(f: f32)-> f32{

    (f - 32.0) * 5.0 / 9.0


}
//z5
fn czas(){
    let h1 = 13;
    let m1 = 28;
    let s1= 45;
    let h2 = 15;
    let m2 = 13;
    let s2= 50;

    let t1 = 3600* h1 + 60 * m1 + s1;
    let t2 = 3600 * h2 + 60 * m2 + s2;
    let mut diff;
    if t1 > t2{diff = t1 - t2;}else {diff = t2 -t1;}
    let g = diff / 3600;
    diff%=3600;
    let m = diff / 60;
    let s = diff % 60;

    println!("{:02}:{:02}:{:02}", g, m, s);
}

//z6
fn silnia(num: u32) -> u32{
    let mut wynik=1;
    for i in 2..=num{
        wynik*=i;
    }
    wynik
}

//z7
fn cyfry( mut num: i32){
    if num > 0{
        loop {
            let cyfra = num %10;
            num/=10;
            if cyfra == 0 {break}
            println!("{}", cyfra);

        }
    }else {
        num*=-1;
        loop {
            let cyfra = num %10;
            num/=10;
            if cyfra == 0 {break}
            println!("{}", cyfra);

        }
        println!("-");
    }

}
//z8
fn suma(mut num : i32) -> i32{
    let mut suma=0;
    if num < 0{
        num*=-1;
        loop {
            suma+=num%10;
            num/=10;
            if num == 0 {break}
        }
    }else {
        loop {
            suma+=num%10;
            num/=10;
            if num == 0 {break}
        }
    }
    suma
}

fn main(){

    //let rok = 2023;
    //let dni = liczba_dni(2, rok);
    //czas();
    //println!("{}",f_na_c(86.0));
    //println!("{}",c_na_f(30.0));
    //println!("{}", dni);
    //println!("{}", silnia(6));
    println!("{}", suma(-12345));

    //cyfry(-3672);
}