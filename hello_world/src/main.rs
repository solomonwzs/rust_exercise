pub mod t_enum;
pub mod t_generic;
pub mod t_hashmap;
pub mod t_lifetime;
pub mod t_slices;
pub mod t_string;
pub mod t_struct;
pub mod t_vectors;

fn main() {
    println!("hello world!");

    let v = vec![1, 2, 3];
    v[99];
}
