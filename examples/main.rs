#![allow(unused)]

#[link_boot::link_boot]
mod boot {
    pub fn fooo() {
        println!("foo");
    }

    pub static BAR: i32 = 1;

    struct Cat {}
}

fn main() {
    println!("Hello, world!");
}
