# Link Boot

This is a macros crate to link mod to `.text.boot` or `.data.boot` section.

# Example

```rust
#[link_boot::link_boot]
mod boot {
    pub fn fooo() {
        println!("foo");
    }

    pub static BAR: i32 = 1;

    struct Cat {}
}
```

expand to:

```rust
mod boot {
    #[unsafe(link_section = ".text.boot")]
    pub fn fooo() {
        {
            std::io::_print(std::format_args_nl!("foo"));
        };
    }
    #[unsafe(link_section = ".data.boot")]
    pub static BAR: i32 = 1;
    struct Cat {}
}
```