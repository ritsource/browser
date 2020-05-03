mod cssom;
mod dev_utils;
mod dom;

fn main() {
    dev_utils::with_piston::render();
    // NEXT: try GTK toolchain
    // NOTE: also checkout ~/Src/gtk-rs-examples
}
