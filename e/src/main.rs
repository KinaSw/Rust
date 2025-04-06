#[derive(Debug)]
enum Szczescie {
    Chleb(u8),
    Bulka(u8),
    Igrzyska,
    Kot(String, u8)
}

fn char2bin(c: char) -> Option<String>{
    let out;
    out = match c {
        '0' => Some(String::from("000")),
        '1' => Some(String::from("001")),
        '2' => Some(String::from("010")),
        '3' => Some(String::from("011")),
        '4' => Some(String::from("100")),
        '5' => Some(String::from("101")),
        '6' => Some(String::from("110")),
        '7' => Some(String::from("111")),
        _ => None

    };
    out
}
fn oct2bin(z: &str) -> Option<String>{
    let mut out = String::new();
    for c in z.chars(){
        /*if let Some(s) = char2bin(c){
            out.push_str(s.as_str());
        } else {
            return None;
        }*/
        // ^^^rownowazne
        out.push_str(char2bin(c)?.as_str())
    }
    Some(out)
}
fn zadanieA1(){
    //if let Some(s) = oct2bin("165"){ ...}   to jest do zrobienia ale nie ma sensu
    println!("{}", oct2bin("165").unwrap());
    println!("{}", oct2bin("777").unwrap());
    println!("{}", oct2bin("997").unwrap());
}

fn main() {
    let t = Szczescie::Kot(String::from("ił"), 4);
    println!("{:?}", t);
    //if t == Szczescie::Kot {      tak nie robimy
    if let Szczescie::Kot(ref s, i) = t {
        println!("kot {} ma {} łapy",s,i);
    } else if let Szczescie::Bulka(_) = t {
        println!("bardzo bulka");
    }

    match t {
        Szczescie::Chleb(_) => println!("1"),
        //Szczescie::Bulka(_) => println!("2"),
        Szczescie::Igrzyska => println!("3"),
        Szczescie::Kot(_,i) => println!("{i}"),
        _ => println!("default")
    }

    let u = (7, 2, "kokosanka");  //krotki :(
    println!("{:?}", u);
    println!("{}, {}, {}", u.0, u.1, u.2);



    // Option<T> -> Some(T) | None
    // Result<T, E> -> Ok(T) | Err(E)
    // fn dziel(x: u8, y: u8) -> Option<u8>
    // rozmiar str jest nieznany w czasie kompilacji, nie da sie go zwrocic, nalezy uzyc String
}
