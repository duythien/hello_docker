use reqwest::Error;

#[macro_export]
macro_rules! prod_or_fast {
    ($prod:expr, $test:expr) => {
        if cfg!(feature = "fast-runtime") {
            $test
        } else {
            $prod
        }
    };
}
#[tokio::main]
async fn main() -> Result<(), Error> {
    let new_line = b'\n';
    //let new_line = vec![1, 2];
    println!("This is test new line {:?} abc", new_line);

    //assert_eq!(100_u8.checked_add(200), Some(100));

    // let t = (1, "hello".to_string());
    // let b = Box::new(t);
    // dbg!(b.0);
    // let arr: Vec<u32> = (1..4).collect();
    // assert_eq!(arr, vec![1, 2, 3]);
    //let s = vec!["udon".to_string(), "ramen".to_string(), "soba".to_string()];

    //let u = s;

    Ok(())
}

fn add(x: u32, y: u32) -> u32 {
    x + y
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_add() {
        assert_eq!(add(1, 2), 3);
    }
}
