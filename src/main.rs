use image::ImageReader;
use std::env;
use std::io;
use std::io::Write;

fn read_line(prompt: &str) -> io::Result<String> {
    println!("{prompt}");
    io::stdout().flush();
    print!("~>");
    let mut input_string = String::new();
    io::stdin().read_line(&mut input_string)?;
    println!();
    return Ok(input_string);
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let mut path: Option<String> = None;
    let mut settings: Vec<String> = Vec::new();
    if args.len() > 1 {
        for arg in args.iter().skip(1) {
            if arg.starts_with("--") {
                settings.push(String::from(arg));
            } else {
                path = Some(String::from(arg));
            }
        }
    }
    let path = match path {
        Some(p) => p,
        None => read_line("Enter path to file: ")?,
    };

    if settings.is_empty() {
        let inp_string = read_line("Enter settings, for exemple scale = 2 etc")?;
        let settings: Vec<_> = inp_string
            .split_whitespace()
            .filter_map(|t| t.strip_prefix("--"))
            .map(|t| t.to_string())
            .collect();
    }
    
    let image = ImageReader::open(path)?.decode()?;
    

    Ok(())
}
