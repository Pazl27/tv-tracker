pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn hello_world() -> String {
    "Hello, world from the logic crate!".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
