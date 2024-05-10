use rust_marco_demo::my_vec;

fn main() {
    println!("Hello, world!");
    let vec:Vec<i32> = my_vec!(1,2,3,4,5,6,7,8,9,10);
    println!("{:?}",vec);
    let vec1 = my_vec!(1;2);
    println!("{:?}",vec1);
}
