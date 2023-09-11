#[firv_harden]
fn f(a: u32, b: u32) -> u32 {
    a + b
}

fn main() {
    print!("{}", f(3,5))
}
