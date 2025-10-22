use serde::{Deserialize, Serialize};
use serde_json::json;
use sha256::digest as sha256;
use std::io::Read;

const API_KEY: &'static str = dotenvy_macro::dotenv!("HACKATTIC_API_KEY");

#[derive(Debug, Deserialize)]
struct Response {
    difficulty: u16,
    block: Block,
}

#[derive(Debug, Deserialize, Serialize)]
struct Block {
    data: Vec<BlockData>,
    nonce: Option<u64>,
}

#[derive(Debug, Deserialize, Serialize)]
struct BlockData(String, i32);

#[derive(Debug, Serialize)]
struct Solution {
    nonce: u64,
}

fn main() -> anyhow::Result<()> {
    println!("> Fetching problem...");
    let resp: Response = reqwest::blocking::get(format!(
        "https://hackattic.com/challenges/mini_miner/problem?access_token={}",
        API_KEY
    ))?
    .json()?;
    let Response { difficulty, block } = resp;

    println!("> Finding nonce...");
    let nonce = match find_nonce(difficulty, block) {
        Some(v) => v,
        None => return Err(anyhow::anyhow!("Could not find nonce.")),
    };

    println!("> nonce: {}", nonce);

    let soln = Solution { nonce };
    let soln_json = json!(soln);

    println!("{:?}", soln_json);

    println!("[!] Press enter to upload solution...");
    _ = std::io::stdin().read(&mut [0]);

    println!("> Uploading solution...");

    let client = reqwest::blocking::Client::new();
    let _ = client
        .post(format!(
            "https://hackattic.com/challenges/mini_miner/solve?access_token={}",
            API_KEY
        ))
        .body(soln_json.to_string())
        .send()?;

    println!("> Solution uploaded successfully!");
    Ok(())
}

fn find_nonce(difficulty: u16, mut block: Block) -> Option<u64> {
    for value in 0..=u64::MAX {
        block.nonce = Some(value);
        let json_content = serde_json::to_string(&block).unwrap();
        let hash = sha256(&json_content);

        if check_zero_bits(&hex_to_bytes(&hash), difficulty) {
            return Some(value);
        }
    }

    None
}

/// This will panic if invalid hex string, or length is not even.
fn hex_to_bytes(hex_str: &str) -> Vec<u8> {
    (0..hex_str.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&hex_str[i..i + 2], 16).unwrap())
        .collect()
}

fn check_zero_bits(bytes: &[u8], mut n: u16) -> bool {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_nonce() {
        let block = Block {
            nonce: None,
            data: vec![],
        };
        let difficulty = 8;

        assert_eq!(find_nonce(difficulty, block), Some(45));
    }

    #[test]
    fn check_zeros() {
        let hex_string = "00deadc0deffff";
        let difficulty = 8;

        assert!(check_zero_bits(&hex_to_bytes(&hex_string), difficulty))
    }
}
