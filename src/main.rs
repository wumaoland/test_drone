use std::io::{stdin, Read};

fn main() {
    println!("Hello, world!");
    let arr = &mut [0;20];
    stdin().read(arr).unwrap();
    println!("{:?}",arr);
}
