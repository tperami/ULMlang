#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub mod ir;

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
