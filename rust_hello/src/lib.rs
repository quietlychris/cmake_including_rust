
// When `hello_from_rust()` is commented out, we don't have a problem; the return_a_four() function works fine

#[no_mangle]
pub extern "C" fn hello_from_rust() {
    println!("hello, world! -- from a Rust function!");
}

#[no_mangle]
pub extern "C" fn return_a_four() -> i32 {
    4
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_raf() {
        assert_eq!(4,crate::return_a_four());
    }


}
