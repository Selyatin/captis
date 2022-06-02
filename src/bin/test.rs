use captis::*;
use std::env;

fn main() {
    #[cfg(target_os = "windows")]
    {
        let mut args = env::args();
        args.next().unwrap();
        let capturer = init_capturer().unwrap();
        println!("Found Displays: {:?}", capturer.displays());
        while let Some(num) = args.next() {
            let num: usize = num.parse().unwrap();
            let now = std::time::Instant::now();
            let image = capturer.capture(num).unwrap();
            println!("Elapsed: {}, Captured: {}", now.elapsed().as_millis(), num);
        }
    }
}
