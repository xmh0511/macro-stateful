use macro_stateful::{define_state, set_state};

set_state! {UniqueState,|v|{
    let v = v.as_mut().unwrap();
    println!("invoke dynamically 1 times, previous: {:?}",v);
    *v = 1;
}}

set_state! {UniqueState,|v|{
    let v = v.as_mut().unwrap();
    println!("invoke dynamically 2 times, previous: {:?}",v);
    *v = 2;
}}

set_state! {UniqueState,|v|{
    let v = v.as_mut().unwrap();
    println!("invoke dynamically 3 times, previous: {:?}",v);
    *v = 3;
}}

define_state! {UniqueState,i32,{
    println!("init");
    0
}}

fn main() {}
