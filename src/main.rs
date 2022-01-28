#[cfg(not(tarpaulin))]
fn main() {
    println!("Hello, world!");
}

#[test]
#[cfg(not(tarpaulin))]
fn testy() {
    assert!(2 + 2 == 5)
}