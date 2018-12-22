extern crate communicator;

pub mod t_enum;
pub mod t_generic;
pub mod t_hashmap;
pub mod t_lifetime;
pub mod t_pointers;
pub mod t_rc;
pub mod t_slices;
pub mod t_string;
pub mod t_struct;
pub mod t_vectors;
pub mod t_trait;

fn foo(s: &mut String) {
    s.push_str("b")
}

fn main() {
    println!("hello world!");

    // let v = vec![1, 2, 3];
    // v[99];

    let mut s = String::from("a");
    let s0 = &mut s;
    foo(s0);
    println!("{}", s0);

    communicator::network::connect();
}
