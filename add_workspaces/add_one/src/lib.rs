use rand;

pub fn add_one_first(x: i32) -> i32 {
    x + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(11, add_one_first(10));
    }
}
