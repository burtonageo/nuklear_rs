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
    
    if cfg!(feature = "vertex_buffers") {
        config.define("NK_INCLUDE_VERTEX_BUFFER_OUTPUT", None);
    }
    
    if cfg!(feature = "font_baking") {
        config.define("NK_INCLUDE_FONT_BAKING", None);
    }

    config.compile("libnuklear.a")
}
