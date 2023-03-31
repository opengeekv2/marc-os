use std::env;

fn main() {
  if env::var("TARGET").unwrap() == "riscv64gc-unknown-none-elf" {
    // Use the linker script.
    println!("cargo:rustc-link-arg=-Tsrc/lds/virt.lds");
    // Don't do any magic linker stuff.
    println!("cargo:rustc-link-arg=--omagic");
  }
}