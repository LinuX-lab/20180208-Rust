use std::num::Wrapping;



fn main(){
    let mut i = Wrapping::<u32>(0);
    let j = Wrapping(1);
    i=i-j;
    println!("{}",i);
}
