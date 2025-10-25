use base64::prelude::*;
use serde::{Deserialize, Serialize};
use std::io::{Cursor, Read};

const API_KEY: &str = dotenvy_macro::dotenv!("HACKATTIC_API_KEY");

#[derive(Deserialize)]
struct Response {
    bytes: String,
}

#[derive(Debug, Serialize)]
struct Solution {
    int: i32,
    uint: u32,
    short: i16,
    float: f32,
    double: f64,
    big_endian_double: f64,
}

fn main() -> anyhow::Result<()> {
    println!("> Fetching problem...");
    let resp: Response = reqwest::blocking::get(format!(
        "https://hackattic.com/challenges/help_me_unpack/problem?access_token={}",
        API_KEY
    ))?
    .json()?;

    println!("> Finding solution...");
    let decoded = BASE64_STANDARD.decode(resp.bytes)?;
    println!("> Size of data: {}", decoded.len());
    let mut reader = Cursor::new(decoded);

    // Parsing...
    let mut buffer = [0_u8; 4];
    reader.read_exact(&mut buffer)?;
    let int = i32::from_le_bytes(buffer);

    let mut buffer = [0_u8; 4];
    reader.read_exact(&mut buffer)?;
    let uint = u32::from_le_bytes(buffer);

    let mut buffer = [0_u8; 2];
    reader.read_exact(&mut buffer)?;
    let short = i16::from_le_bytes(buffer);

    // Discarding 2 bytes due to padding
    reader.read_exact(&mut [0; 2])?;

    let mut buffer = [0_u8; 4];
    reader.read_exact(&mut buffer)?;
    println!("{:?}", buffer);
    let float = f32::from_le_bytes(buffer);

    let mut buffer = [0_u8; 8];
    reader.read_exact(&mut buffer)?;
    let double = f64::from_le_bytes(buffer);

    let mut buffer = [0_u8; 8];
    reader.read_exact(&mut buffer)?;
    let big_endian_double = f64::from_be_bytes(buffer);

    // Combine and upload
    let solution = Solution {
        int,
        uint,
        short,
        float,
        double,
        big_endian_double,
    };

    println!("[!] Found solution!");
    println!("{:#?}", solution);

    // println!("[!] Press enter to upload solution...");
    // _ = std::io::stdin().read(&mut [0]);

    let solution_json = serde_json::to_string(&solution)?;
    println!("> Uploading solution...");

    let client = reqwest::blocking::Client::new();
    let resp = client
        .post(format!(
            "https://hackattic.com/challenges/help_me_unpack/solve?access_token={}",
            API_KEY
        ))
        .body(solution_json)
        .send()?;

    if resp.status().is_success() {
        println!("> Solution uploaded successfully!");
    } else {
        println!("[!] Error while uploading solution.");
    }
    println!("{}", resp.text().unwrap());

    Ok(())
}
