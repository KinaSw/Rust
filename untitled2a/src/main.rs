fn ascii(){
    println!("{:<6}{}", "Kod", "Znak");
    println!("-----------------");

    for kod in 33u8..=126{
        let znak = kod as char;
        println!("{:<6}{}", kod, znak);
    }
}
//------------------------------------------
//3a
fn zamien(a: &mut i32, b: &mut i32){
    let temp = *a;
    *a = *b;
    *b = temp;
}

fn zamien_3(a: &mut i32, b: &mut i32, c: &mut i32){
    if *a > *b {
        zamien(a,b);
    }
    if *a > *c {
        zamien(a,c);
    }
    if *b > *c {
        zamien(b,c);
    }
}

//z3
fn rand(seed: &mut u64, min_rand: i32, max_rand: i32) -> i32{
    const A: u64 = 1664525;
    const C: u64 = 1013904223;
    const M: u64 = u64::MAX;

    *seed = (*seed).wrapping_mul(A).wrapping_add(C) % M;

    let range = (max_rand - min_rand +1) as u64;
    (min_rand as u64 + (*seed % range )) as i32
}

//z4
fn swap_arr<T>(arr: &mut [T], i: usize, j: usize){
    arr.swap(i, j);
}

//z5
fn rand_perm<T>(arr: &mut [T], seed: &mut u64){
    let n = arr.len();
    for i in (1..n).rev(){
        let j = rand(seed, 0, i as i32) as usize;
        swap_arr(arr, i, j);
    }
}
//----------------------------------------
//4a
//z1
fn liczba_wystapien(napis: &str, znak: char) -> usize{
    napis.chars().filter(|x| *x==znak).count()
}
//z2
fn rzymskie(napis: &str) -> u32{
    fn wartosc(c: char) -> u32{
        match c {
            'I' => 1,
            'V' => 5,
            'X' => 10,
            'L' => 50,
            'C' => 100,
            'D' => 500,
            'M' => 1000,
            _ => 0,
        }
    }

    let mut suma = 0;
    let mut poprzednia = 0;
    for c in napis.chars().rev(){
        let obecna = wartosc(c);
        if obecna < poprzednia{
            suma -= obecna;
        }else {
            suma += obecna;
        }
        poprzednia = obecna;
    }
    suma
}
fn main() {
    ascii();
    let mut x = 10;
    let mut y = 2;
    let mut z = 1;
    //zamien(&mut x, &mut y);
    //println!("x={}, y={}", x,y);
    zamien_3(&mut x, &mut y, &mut z);
    println!("{}, {}, {}", x, y, z);

    let mut ziarno = 123456789;

    for _ in 0..10 {
        let los = rand(&mut ziarno, 1, 100);
        println!("Liczba: {}", los);
    }

    let mut tablica = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut ziarno = 123456789;

    println!("Przed: {:?}", tablica);
    rand_perm(&mut tablica, &mut ziarno);
    println!("Po:    {:?}", tablica);

    let tekst = "trzeba być świrem żeby lubić Rusta";
    let ile_a = liczba_wystapien(tekst, 'a');
    println!("Litera 'a' występuje {} razy", ile_a);

    println!("III  -> {}", rzymskie("III"));    // 3
    println!("IX   -> {}", rzymskie("IX"));     // 9
    println!("XIX  -> {}", rzymskie("XIX"));    // 19
    println!("MCMX -> {}", rzymskie("MCMX"));   // 1910
}
