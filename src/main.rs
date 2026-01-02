mod converter;
mod letter_pool;

use image::ImageReader;
use std::env;
use std::io;
use std::io::Write;

#[derive(Debug, Clone)]
struct Config {
    pub cols: u32,
    pub font_path: String,
    pub font_px: f32,
    pub out_path: String,
    pub charset: String,
}
fn read_line(prompt: &str) -> io::Result<String> {
    println!("{prompt}");
    print!("~> ");
    io::stdout().flush()?;
    let mut s = String::new();
    io::stdin().read_line(&mut s)?;
    println!();
    Ok(s.trim().to_string())
}

fn parse_option(opt: &String) -> Option<(&str, &str)> {
    let (opt_text, opt_val) = opt.split_once("=")?;
    Some((opt_text.trim().strip_prefix("--")?, opt_val.trim()))
}
fn parse_name(path: &String) -> Result<String, String> {
    let name = path
        .rsplit('/')
        .next()
        .ok_or_else(|| "empty path".to_string())?;
    Ok(name.to_string())
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
        let inp_string = read_line("Enter settings or enter def if you want default")?;
        settings = inp_string
            .split_whitespace()
            .filter_map(|t| t.strip_prefix("--"))
            .map(|t| t.to_string())
            .collect();
    }

    let file_name = match parse_name(&path) {
        Ok(file_name) => file_name,
        Err(err) => err,
    };
    // Default settings
    let mut cfg = Config {
        cols: 220,
        font_path: "/home/vlm326/.local/share/fonts/JetBrainsMonoNLNerdFont-Regular.ttf".to_string(),
        font_px: 14.0,
        out_path: format!("./{file_name}_ascii.png").to_string(),
        charset: " .,:;i1tfLCG08@".to_string(),
    };

    for opt in settings.iter() {
        if let Some((name, val)) = parse_option(opt) {
            match name {
                "cols" => cfg.cols = val.parse()?,
                "font" => cfg.font_path = val.to_string(),
                "font_px" => cfg.font_px = val.parse()?,
                "out_name" => {
                    cfg.out_path = format!("./{}_ascii.png", val.to_string()).to_string()
                }
                "charset" => cfg.charset = val.to_string(),
                "def" => break,
                _ => eprintln!("Unsupported setting: {name}"),
            }
        }
    }
    let image = ImageReader::open(&path)?.decode()?;
    let out_img = converter::convert_simple(&image, &cfg)?;
    out_img.save(&cfg.out_path)?;
    println!("Saved: {}", cfg.out_path);

    Ok(())
}
