#![allow(unused)]

#[link_boot::link_boot]
mod boot {
    pub fn fooo() {
        println!("foo");
    }

    pub static BAR: i32 = 1;

    struct Cat {}

    impl Cat {
        pub fn new() -> Self {
            Self {}
        }
        #[unsafe(no_mangle)]
        pub fn meow(&self) {
            println!("meow");
        }
    }
}

fn main() {
    fooo();
    let cat = Cat::new();
    cat.meow();

    println!("Hello, world!");
}
