use rust_marco_demo::EnumFromDaring;

fn main() {
    let left: Direction = 120.into();
    let up: Direction = DirectionUp::new(100).into();
    println!("left {:?},up :{:?}", left, up);
}
#[derive(Debug, EnumFromDaring)]
#[allow(dead_code)]
enum Direction {
    Up(DirectionUp),
    Down,
    Left(i32),
    Right { a: i32 },
}
#[derive(Debug)]
#[allow(dead_code)]
struct DirectionUp {
    speed: i32,
}
#[allow(dead_code)]
impl DirectionUp {
    fn new(speed: i32) -> Self {
        Self { speed }
    }
}
