use serde::{Deserialize, Serialize};
use std::io::Cursor;

const API_KEY: &str = dotenvy_macro::dotenv!("HACKATTIC_API_KEY");

#[derive(Deserialize)]
struct Problem {
    zip_url: String,
}

#[derive(Debug, Serialize)]
struct Solution {
    secret: String,
}

fn main() -> anyhow::Result<()> {
    println!("> Fetching problem...");
    let problem = fetch_problem()?;

    println!("> Finding solution (this may take some time)...");
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

fn fetch_problem() -> anyhow::Result<Cursor<bytes::Bytes>> {
    let client = reqwest::blocking::Client::new();
    let response: Problem = client
        .get(format!(
            "https://hackattic.com/challenges/brute_force_zip/problem?access_token={}",
            API_KEY
        ))
        .send()?
        .json()?;

    let file = client.get(response.zip_url).send()?.bytes()?;
    let cursor = Cursor::new(file);

    Ok(cursor)
}

fn upload_solution(solution: Solution) -> anyhow::Result<reqwest::blocking::Response> {
    let serialized_solution = serde_json::to_string(&solution)?;
    let client = reqwest::blocking::Client::new();
    let response = client
        .post(format!(
            "https://hackattic.com/challenges/brute_force_zip/solve?access_token={}",
            API_KEY
        ))
        .body(serialized_solution)
        .send()?;

    Ok(response)
}

fn solve(reader: Cursor<bytes::Bytes>) -> anyhow::Result<Solution> {
    use itertools::Itertools as _;
    use std::io::Read as _;

    let mut zip_reader = zip::ZipArchive::new(reader)?;

    let valid_characters = b"abcdefghijklmnopqrstuvwxyz0123456789";

    for len in 4..=6 {
        for password in (0..len)
            .map(|_| valid_characters.iter().map(|&n| n))
            .multi_cartesian_product()
        {
            if let Ok(mut decrypted_file) = zip_reader.by_name_decrypt("secret.txt", &password) {
                let mut content = String::new();

                // As of writing this code, the decrypt function may accept invalid password,
                // so we try other passwords if this decrypted file can't be read properly,
                // i.e. password was incorrectly accepted. This may still fail,
                // but the chances are very low.
                if decrypted_file.read_to_string(&mut content).is_err() {
                    continue;
                }

                let solution = Solution {
                    secret: content.trim().to_string(),
                };
                return Ok(solution);
            }
        }
    }

    Err(anyhow::anyhow!("Couldn't find the password."))
}
