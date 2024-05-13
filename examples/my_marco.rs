use anyhow::Result;

fn main() -> Result<()> {
    println!("Hello, world!");
    let vec: Vec<i32> = my_vec!(1, 2, 3, 4, 5, 6, 7, 8, 9, 10);
    println!("{:?}", vec);
    let vec1 = my_vec!(1;2);
    println!("{:?}", vec1);
    my_try!(foo("str"));
    Ok(())
}
fn foo(str: &str) -> Result<String> {
    if str.eq("hello world") {
        Ok(str.to_owned())
    } else {
        Err(anyhow::anyhow!(" not expecting string"))
    }
}
// 定义一个声明式宏，用来生成vec
#[macro_export]
macro_rules! my_vec {
    () => {Vec::new()};
    ($elem:expr;$n:expr)=>{
        std::vec::from_elem($elem,$n)
    };
    ($($ex:expr),+ $(,)?) =>{
        {
            let mut temp = Vec::new();
            $(
                temp.push($ex);
            )+
            temp
        }
    };
}
//定义一个声明宏来,实现异常try
#[macro_export]
macro_rules! my_try {
    ($expr:expr) => {
        match $expr {
            Ok(val) => val,
            Err(err) => return Err(err.into()),
        }
    };
}
