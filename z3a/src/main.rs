//z1
fn swap(a: &mut i32, b: &mut i32){
    let temp = *a;
    *a = *b;
    *b = temp;

}

fn sort(a: &mut i32, b: &mut i32, c: & mut i32){
    if *a > *b{
        swap(a, b);
    }
    if *b > *c{
        swap(b,c);
    }
    if *a>*b{
        swap(a,b);
    }
}
//z4
fn swap_arr<T>(arr: &mut [T], i: usize, j: usize) {
    if i != j && i < arr.len() && j < arr.len() {
        arr.swap(i, j);
    }
}

fn main() {
    let mut a = 6;
    let mut b = 7;
    let mut c = 1;
    let mut nums = [1, 2, 3, 4, 5];

    //sort(& mut a, & mut b, & mut c);
    //println!("{}, {}, {}", a, b,c);
    swap_arr(& mut nums, 1, 3);

    for i in 0..nums.len(){
        println!("{}", nums[i]);
    }
}
