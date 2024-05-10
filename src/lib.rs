#[macro_export]
macro_rules! my_vec {
    () => {Vec::new()};
    ($elem:expr;$n:expr)=>{
        std::vec::from_elem($elem,$n);
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