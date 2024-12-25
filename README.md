Recording state via the global scope variable

### Usage:
Using the `define_state` macro, define the state with a unique name and specify its type. Using `set_state` to modify the state, the parameter type in the callback is `Option<T>` where T is the type specified for the state.


````rust
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

define_state! {UniqueState:i32 = {
    println!("init");
    0
}}

fn main() {}

````

