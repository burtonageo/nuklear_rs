extern crate gcc;

fn main() {
    let mut config = gcc::Config::new();
    config.file("nuklear/nuklear.c");

    if cfg!(feature = "malloc_allocator") {
        config.define("NK_INCLUDE_DEFAULT_ALLOCATOR", None);
    }

    if cfg!(feature = "io") {
        config.define("NK_INCLUDE_STANDARD_IO", None);
    }

    config.compile("libnuklear.a")
}
