/*
fn powiekszona_o_1_v1(x: i32) -> i32{
    x+1
}

fn powiekszona_o_1_v2(mut x: i32) -> i32{
    x+=1;
    x
}
fn powiekszona_o_1_v3(x: &mut i32){
    *x+=1;
}

 */

/*
fn swap(x: &mut i32, y: &mut i32){
    let pom = *x;
    *x=*y;
    *y = pom;
}
*/

/*fn powitaj_v1(imie: &str){ // najwiecej sensu ma to
    println!("Witaj, {}", imie);
}

fn powitaj_v2(imie: &String){ // to gowno nie uzywac
    println!("Witaj, {}", imie);
}
fn powitaj_v3(imie: String){
    println!("Witaj, {}", imie);
}

 */

fn powitaj_v0(tab: [i32; 4]){
    println!("Witaj, {tab:?}");
}
fn powitaj_v1(tab: &[i32]){
    println!("Witaj, {tab:?}");
}

fn powitaj_v2(tab: &Vec<i32>){
    println!("Witaj, {tab:?}");
}
fn powitaj_v3(tab: Vec<i32>){
    println!("Witaj, {tab:?}");
}

fn wyswietl_jeden(t: &[i32], i:usize){
    println!("{}", t[i]);
}

fn main() {

    let tab0 = [1,4,90,34];
    powitaj_v0(tab0);
    powitaj_v1(&[15, 3, 20]);
    let tab1 = vec![3,5,7,4,2,5, 10];
    let tab2 = Vec::from([3,6,7,4,2]);
    powitaj_v2(&tab1);
    powitaj_v3(tab2.clone());
    powitaj_v1(&tab1);
    powitaj_v1(&tab2);
    dbg!(tab1);
    dbg!(tab2);

    println!("{}", tab0[2]);
    wyswietl_jeden(&tab0, 12);
    /*powitaj_v1("Edek");
    let imie1 = "Felek".to_string();
    let imie2 = String::from("Balbina");
    powitaj_v2(&imie1);
    powitaj_v3(imie2.clone());
    powitaj_v1(&imie1);
    powitaj_v1(&imie2);
    dbg!(imie1);
    dbg!(imie2);

     */

    /*
    let mut a = 10;
    let mut b = 20;
    swap(&mut a, &mut b);
    dbg!(a);
    dbg!(b);
    //swap(&mut a, &mut a); nie dziala

     */



    /*let mut a = 12;
    let b = powiekszona_o_1_v1(a);
    println!("{}", a == 12);
    println!("{}", b == 13);
    let c = powiekszona_o_1_v2(a);
    println!("{}", a == 12);
    println!("{}", c == 13);
    powiekszona_o_1_v3(&mut a);
    println!("{}", a == 13);

     */


}

/*
przekazywanie parametrów:
    na własność (przez wartość)
        przez kopiowanie
        przez przeniesienie
    przez pożyczkę/referencję (&)
    przez pożyczkę/referencję mutowalną (&mut)
 */