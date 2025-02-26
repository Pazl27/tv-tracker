mod config;

pub fn hello_world() -> String {
    "Hello, world from the logic crate!".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hello_world() {
        assert_eq!(hello_world(), "Hello, world from the logic crate!");
    }
}
