use base64::prelude::*;
use serde::{Deserialize, Serialize};
use std::io::{Cursor, Read};

const API_KEY: &str = dotenvy_macro::dotenv!("HACKATTIC_API_KEY");

#[derive(Deserialize)]
struct Problem {
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
    let problem = fetch_problem()?;

    println!("> Finding solution...");
    let solution = solve(problem)?;
    println!("[!] Found solution!");
    println!("{:#?}", solution);

    println!("> Uploading solution...");
    let response = upload_solution(solution)?;
    if response.status().is_success() {
        println!("> Solution uploaded successfully!");
    } else {
        println!("[!] Error while uploading solution.");
    }
    println!("{}", response.text()?);

    Ok(())
}

fn fetch_problem() -> anyhow::Result<Problem> {
    let client = reqwest::blocking::Client::new();
    let response = client
        .get(format!(
            "https://hackattic.com/challenges/help_me_unpack/problem?access_token={}",
            API_KEY
        ))
        .send()?
        .json()?;

    Ok(response)
}

fn upload_solution(solution: Solution) -> anyhow::Result<reqwest::blocking::Response> {
    let serialized_solution = serde_json::to_string(&solution)?;
    let client = reqwest::blocking::Client::new();
    let response = client
        .post(format!(
            "https://hackattic.com/challenges/help_me_unpack/solve?access_token={}",
            API_KEY
        ))
        .body(serialized_solution)
        .send()?;

    Ok(response)
}

fn solve(problem: Problem) -> anyhow::Result<Solution> {
    let decoded = BASE64_STANDARD.decode(problem.bytes)?;
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
    let float = f32::from_le_bytes(buffer);

    let mut buffer = [0_u8; 8];
    reader.read_exact(&mut buffer)?;
    let double = f64::from_le_bytes(buffer);

    let mut buffer = [0_u8; 8];
    reader.read_exact(&mut buffer)?;
    let big_endian_double = f64::from_be_bytes(buffer);

    let solution = Solution {
        int,
        uint,
        short,
        float,
        double,
        big_endian_double,
    };

    Ok(solution)
}
