use rand;

pub fn add_one(x: i32) -> i32 {
    x + rand::random::<i32>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert!(add_one(2) > 2);
    }
}