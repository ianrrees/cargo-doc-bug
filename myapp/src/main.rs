
use mylib_1;
use mylib_2;

/// Does some divisions using [`mylib_1::divide`] and [`mylib_2::divide`]
fn main() {
    println!("Old 1/2 is {}", mylib_1::divide(1, 2));
    println!("New 1/2 is {}", mylib_2::divide(1, 2));
}
