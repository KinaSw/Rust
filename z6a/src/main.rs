// tablice, Wektory, ITERATORY
fn zad1(){
    /*let mut vec: Vec<char> = Vec::new();
    for c in 'a'..='z'{
        vec.push(c);
    }*/
    let vec: Vec<char> = ('a'..='z').collect();
    /*println!("{:?}", vec);

    let mut vec: Vec<u32> = Vec::new();
    for i in 1..=10{
        vec.push(i*i);
    }*/
    //let vec: Vec<u32> = (1..=10).map(|x| x*x).collect();
    //println!("{:?}", vec);

    let powof2: Vec<u32> = (1..=10).map(|x| { 2_u32.pow(x)}).collect();
    println!("{:?}", powof2);

    let vec1: Vec<f64>=(1..=20).map(|x| 1.0/ x as f64).collect();
    println!("{:?}", vec1);

    let vec: Vec<u32> = (1..=100).filter(|x| x % 3 == 0 && x% 4!= 0).collect();

    println!("{:?}", vec);
}

fn z2_f1(v: & mut  Vec<String>) -> Vec<String> {
    v.retain(|s| s.len() > 4);
    v

}

fn zadanie2(){
    //let napisy = Vec::from([String::from("mleko")])

    let mut napisy: Vec<String> =
    ["mleko", "kot", "panda", "nietoperz", "netto", "sowoniedzwiedz"]
        .iter()
        .map(|x| String::from(*x))
        .collect();

    println!("{:?}",z2_f1(& mut napisy));
    println!("{:?}",z2_f2(napisy));
}

fn z2_f2(mut v: Vec<String>) -> Vec<String> {
    v.retain(|s| !(s.contains("a") || s.contains("A")));

    v
}
fn main() {
    let arr: [u32; 4] = [1,2,3,4];
    /*for i in 0..4{
        println!("{}", arr[i]);
    }*/
    let arr = [1; 10]; // tablica dziesięciu jedynek
    //println!("arr: {:?}", arr);

    let mut vec: Vec<u32> = Vec::new();
    vec.push(7);
    vec.push(2);
    //println!("vec: {:?}", vec);

    /*for t in &vec {
        println!("{:?} ", t);
        println!("{:?}", vec[0]);
    }*/

    let vec = Vec::from([1,2,3]);
    zadanie2();

    //zad1();
}
