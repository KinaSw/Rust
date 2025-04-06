fn swap( a: &mut i32,  b: &mut i32){
    let c = *a;
    *a = *b;
    *b = c;
}


fn sort(a : &mut i32, b: &mut i32, c: &mut i32 ){

    if *a > *b{
        swap(a,b);
    }
    if *b > *c {
        swap(c,b);
    }
    if *a > *b {
        swap(a,b);
    }

}
fn co_drugi_znak(napis: String) -> String {
    let mut out = String::new();
    let mut i = 0;
    for c in napis.chars(){
        if i % 2 == 0{
            out.push(c);
        }
        i+=1;
    }
    out
}

fn szyfruj(napis: String, klucz: i32){
    

}

/* praca domowa:
 konwersja z rzymskiego na arabski i z arabskiego na rzymski
 */



fn main() {

    let pusty = String::new();
    let mut napis = String::from("kocurek");
    co_drugi_znak(napis);
    //println!("{} -- {}", pusty, napis);
    let mut a = 54;
    let mut b = 5;
    let mut c = 10;
    println!("przed swapem: {} -- {} -- {}", a, b, c);
    //let t = &mut a;
    sort( &mut a, &mut b, &mut c );

    //*t = 5;
    println!("po swapie: {} -- {} -- {}", a, b, c);
}


/*
{
    let t;
    jakis tam blok kodu
}
t juz tutaj nie istnieje

fn g(x) {
    println!(x);
    }

fn f{
    let x : T = T::new();
    g(x);
    x=8;

}

v2
fn f{
    let x : T = T::new();
    g(x.clone());  <- kopiowanie jawne
    x=8;


    STRING
    let s = "mleko";
    let k = String::new();
    String::from("mleko");
}
 */