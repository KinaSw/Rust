/*
struct Point{
    x: f32,
    y: f32
}

impl Point{
    fn miau(){
        println!("Miau");
    }

    fn new() -> Point{
        Point {x: 00.0, y: 0.0}
    }

    fn print(&self) {  //nie jest prymitywem, nie mozna go kopiowac
        println!("({}, {})", self.x, self.y);
    }

}
*/

#[derive(PartialEq, Clone)]
struct Rgb{
    r: u8,
    g: u8,
    b: u8
}

impl Rgb{
    fn from_3u8(r: u8, g: u8, b: u8) -> Self{
        Self{r, g, b}
    }

    fn is_percent(t: f32) -> bool{
        0.0 <= t && t <= 100.0
    }
    fn from_percent_to_u8(p: f32) -> u8{
        (p*2.55) as u8
    }

    fn from_3percent(r: f32, g: f32, b: f32) -> Option<Self>{
        if !Self::is_percent(r) ||
            !Self::is_percent(g) ||
            !Self::is_percent(b){
            return None;
        }
        Some(Rgb { r:Self::from_percent_to_u8(r),
                   g:Self::from_percent_to_u8(g),
                   b:Self::from_percent_to_u8(b),
        })

    }
    fn gray(t: f32) -> Option<Rgb> {
        Self::from_3percent(t, t, t)
    }

    fn white() -> Rgb{
        Self::gray(100.0).unwrap()
    }

    fn black() -> Rgb{
        Self::gray(0.0).unwrap()
    }

    fn intensity(&self) -> f32{
        let t = (self.r + self.g + self.b) as f32;
        t / (3.0 * 255.0)
    }

    fn invert(&mut self){
        self.r = 255 - self.r;
    }

    fn as_rgb_u8tuple(&self) -> (u8,u8,u8){
        (self.r, self.g, self.b)
    }

    fn as_cmy_u8tuple(&self) -> (u8,u8,u8){
        let mut c = self.clone();
        c.invert();
        (c.r, c.g, c.b)
    }
}
fn main() {
    let szary1 = Rgb::from_3u8(127, 127, 127);
    let szary2 = Rgb::from_3percent(50.0, 50.0, 50.0).unwrap();
    let szary3 = Rgb::gray(50.0).unwrap();
    let fiolet = Rgb::from_3u8(100, 35, 120);
    let bialy1 = Rgb::white();
    let bialy2 = Rgb::from_3u8(255, 255, 255);
    let mut czarny1 = Rgb::black();
    let czarny2 = Rgb::from_3u8(0, 0, 0);
    println!("{} {}", szary1 == szary2, szary1 == szary3);
    println!("{} {}", bialy1 == bialy2, czarny1 == czarny2);
    czarny1.invert();
    println!("{}", bialy1 == czarny1);
    println!("{}", fiolet.intensity() == 1.0/3.0);
    println!("{}", fiolet.as_rgb_u8tuple() == (100, 35, 120));
    println!("{}", fiolet.as_cmy_u8tuple() == (155, 220, 135));
    /*
    let p: Point = Point{x: 2.0, y: 1.0};
    println!("{}, {}", p.x, p.y);
    Point::miau();
    let q = Point::new();
    println!("{}, {}", q.x, q.y);
    q.print();
    q.print();
     */
}
