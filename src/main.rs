fn sum(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    let result = sum(15, 26);
    println!("The result of addition 15 + 26 = {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum() {
        assert_eq!(sum(0, 0), 0);
        assert_eq!(sum(1, 1), 2);
        assert_eq!(sum(15, 26), 41);
    }
}
