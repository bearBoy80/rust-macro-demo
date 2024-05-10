use anyhow::Result;
use rust_marco_demo::{my_try, my_vec};

fn main() ->Result<()> {
    println!("Hello, world!");
    let vec:Vec<i32> = my_vec!(1,2,3,4,5,6,7,8,9,10);
    println!("{:?}",vec);
    let vec1 = my_vec!(1;2);
    println!("{:?}",vec1);
    my_try!(foo("str"));
    Ok(())
}
fn foo(str: &str) ->Result<String> {
    if str.eq("hello world") {
        Ok(str.to_owned())
    }else{
        Err(anyhow::anyhow!(" not expecting string"))
    }
}
fn foo1(str: &str) -> core::result::Result<String, &'static str> {
    Err("expected string")
}
