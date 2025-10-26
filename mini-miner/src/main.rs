use serde::{Deserialize, Serialize};
use sha256::digest as sha256;

const API_KEY: &str = dotenvy_macro::dotenv!("HACKATTIC_API_KEY");

#[derive(Deserialize)]
struct Problem {
    difficulty: u16,
    block: Block,
}

#[derive(Deserialize, Serialize)]
struct Block {
    data: Vec<BlockData>,
    nonce: Option<u64>,
}

#[derive(Deserialize, Serialize)]
struct BlockData(String, i32);

#[derive(Debug, Serialize)]
struct Solution {
    nonce: u64,
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
    println!("{}", response.text().unwrap());

    Ok(())
}

fn fetch_problem() -> anyhow::Result<Problem> {
    let client = reqwest::blocking::Client::new();
    let response = client
        .get(format!(
            "https://hackattic.com/challenges/mini_miner/problem?access_token={}",
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
            "https://hackattic.com/challenges/mini_miner/solve?access_token={}&playground=1",
            API_KEY
        ))
        .body(serialized_solution)
        .send()?;

    Ok(response)
}

fn solve(problem: Problem) -> anyhow::Result<Solution> {
    let Problem {
        difficulty,
        mut block,
    } = problem;

    let nonce = (0..=u64::MAX).into_iter().find(|&value| {
        block.nonce = Some(value);
        let serialized_block = serde_json::to_string(&block).unwrap();
        let hash = sha256(&serialized_block);
        let Ok(bytes) = hex_to_bytes(&hash) else {
            return false;
        };

        check_minimum_zero_bits(&bytes, difficulty)
    });

    let Some(nonce) = nonce else {
        return Err(anyhow::anyhow!("Could not find nonce!"));
    };

    let solution = Solution { nonce };
    Ok(solution)
}

fn hex_to_bytes(hex_str: &str) -> anyhow::Result<Vec<u8>> {
    if !hex_str.len().is_multiple_of(2) {
        return Err(anyhow::anyhow!("Hex string length must be even."));
    }

    (0..hex_str.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&hex_str[i..i + 2], 16).map_err(From::from))
        .collect()
}

fn check_minimum_zero_bits(bytes: &[u8], mut n: u16) -> bool {
    let mut counter: usize = 0;

    while n >= 8 {
        if bytes[counter] != 0 {
            return false;
        }

        counter += 1;
        n -= 8;
    }

    (bytes[counter] << n >> n) == bytes[counter]
}
