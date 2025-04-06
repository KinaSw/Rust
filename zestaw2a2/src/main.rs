fn f(x: f64) -> f64 {
    (x - (3.0_f64.sqrt() - 1.0) / 2.0) * (x + 20.0)
}
fn fp(x: f64) -> f64 {
    2.0 * x + 20.0 - (3.0_f64.sqrt() - 1.0) / 2.0
}
fn met_newt_loop(mut x: f64, eps: f64, n: u128) -> f64 {
    let mut iter = 0;
    loop {
        let fx = f(x);
        let fpx = fp(x);
        if fpx.abs() < eps || iter >= n {
            break;
        }
        let x_next = x - fx / fpx;
        if (x_next - x).abs() < eps {
            return x_next;
        }
        x = x_next;
        iter += 1;
    }
    x
}

fn main() {
    let x0 = 1.0;
    let eps = 0.25;
    let n = 100;

    println!("{}", met_newt_loop(x0, eps, n));

}
