use reqwest::Error;
use std::thread;
use std::time::Instant;

use std::path::PathBuf;
use tokio::fs::File as AsyncFile;
use tokio::io::AsyncReadExt;
use tokio::sync::watch;
use tokio::time::{sleep, Duration};

async fn watch_file_changes(tx: watch::Sender<bool>) {
    let path = PathBuf::from("data.txt");
    let mut last_modified = None;
    loop {
        if let Ok(metadata) = path.metadata() {
            let modified = metadata.modified().unwrap();

            if last_modified != Some(modified) {
                last_modified = Some(modified);
                let _ = tx.send(true);
            }
        }
        sleep(Duration::from_millis(100)).await;
    }
}

async fn read_file(filename: &str) -> Result<String, std::io::Error> {
    let mut file = AsyncFile::open(filename).await?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).await?;
    Ok(contents)
}

#[derive(Debug, Clone)]
pub struct DisplayError {
    pub x: u32,
    pub y: u32,
    pub color: Color,
}

//impl std::error::Error for String {}
#[derive(Debug, Clone)]
pub struct RgbColor(u8, u8, u8);
#[derive(Debug, Clone)]
pub enum Color {
    Monochrome,
    Foreground(RgbColor),
}

/// Integer value from an IANA-controlled range.
#[derive(Clone, Copy, Debug)]
pub struct IanaAllocated(pub u64);

impl From<u64> for IanaAllocated {
    fn from(value: u64) -> Self {
        Self(value)
    }
}
/// Indicate whether value is reserved.
pub fn is_iana_reserved<T>(s: T) -> bool
where
    T: Into<IanaAllocated>,
{
    let s = s.into();
    s.0 == 0 || s.0 == 65535
}

type UserId = u32;
pub fn find_user(username: &str) -> Result<UserId, String> {
    let f = std::fs::File::open("/etc/passwd")
        .map_err(|e| format!("Failed to open passsword file{:?}", e))?;

    Ok(1)
}

trait Name {
    // add code here
}

struct Direction(pub u32);
pub struct PoundForceSeconds(pub f64);

/// Fire the thrusters. Returns generated impulse.
pub fn thruster_impulse(direction: Direction) -> PoundForceSeconds {
    // ...
    return PoundForceSeconds(42.0);
}

/// Units for force.
pub struct NewtonSeconds(pub f64);

/// Update trajectory model for impulse.
pub fn update_trajectory(force: NewtonSeconds) {
    // ...
}

impl From<PoundForceSeconds> for NewtonSeconds {
    fn from(val: PoundForceSeconds) -> NewtonSeconds {
        NewtonSeconds(4.448222 * val.0)
    }
}
struct DoubleSided(pub bool);

struct ColorOutput(pub bool);

fn print_page(sides: DoubleSided, color: ColorOutput) {
    // ...
}
struct Modulo(pub u64);
trait Calculate {
    fn add(&self, l: u64, r: u64) -> u64;
    fn mul(&self, l: u64, r: u64) -> u64;
}
impl Calculate for Modulo {
    fn add(&self, l: u64, r: u64) -> u64 {
        (l + r) % self.0
    }
    fn mul(&self, l: u64, r: u64) -> u64 {
        (l * r) % self.0
    }
}
fn fibonacci(n: u64) -> u64 {
    if n == 0 || n == 1 {
        return n;
    }
    fibonacci(n - 1) + fibonacci(n - 2)
}

fn consume_with_relish<F>(func: F)
where
    F: FnOnce() -> String,
{
    // `func` consumes its captured variables, so it cannot be run more
    // than once.
    println!("Consumed: {}", func());

    println!("Delicious!");

    // Attempting to invoke `func()` again will throw a `use of moved
    // value` error for `func`.
}

fn caculate_string(s: String) -> usize {
    s.len()
}
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
    let x = String::from("x");
    let consume_and_return_x = move || x;
    consume_with_relish(consume_and_return_x);

    //assert_eq!(100_u8.checked_add(200), Some(100));

    // let t = (1, "hello".to_string());
    // let b = Box::new(t);
    // dbg!(b.0);
    // let arr: Vec<u32> = (1..4).collect();
    // assert_eq!(arr, vec![1, 2, 3]);
    let s = vec!["udon".to_string(), "ramen".to_string(), "soba".to_string()];
    let t = &s;
    let u = s;

    Ok(())
}
