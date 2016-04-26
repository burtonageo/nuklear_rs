extern crate gcc;

fn main() {
    gcc::compile_library("libnuklear.a", &["nuklear/nuklear.c"])
}
