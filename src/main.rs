use std::io;

fn main() {
    loop {
        println!("Enter temperature in Fahrenheit:");

        let mut f = String::new();

        io::stdin()
            .read_line(&mut f)
            .expect("Failed to read line");

        let f: f32 = match f.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                if f.trim().to_lowercase() == "q" ||
                   f.trim().to_lowercase() == "quit"
                {
                    break;
                } else {
                    continue;
                }
            }
        };

        let c = (f - 32.0) * 5.0 / 9.0;

        println!("Temperature in Celsius: {}\n", c);
    }
}
