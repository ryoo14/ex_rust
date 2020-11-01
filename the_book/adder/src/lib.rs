pub fn add_two(a: i32) -> i32 {
    a + 2
}

pub fn greeting(s: &str) -> String {
    format!("Hello, {}", s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_adds_two() {
        assert_eq!(3, add_two(2));
    }

    #[test]
    fn contain_greet() {
        let greet = greeting("hogehoge");
        assert!(greet.contains("hoge"));
    }
}
