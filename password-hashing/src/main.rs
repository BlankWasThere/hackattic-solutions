use base64::prelude::*;
use serde::{Deserialize, Serialize};

const API_KEY: &str = dotenvy_macro::dotenv!("HACKATTIC_API_KEY");

macro_rules! hexlify {
    ($item:expr) => {
        $item
            .iter()
            .map(|x| ::std::format!("{x:02x}"))
            .collect::<Vec<_>>()
            .join("")
    };
}

#[derive(Deserialize)]
struct Problem {
    password: String,
    salt: String,
    pbkdf2: Pbkdf2,
    scrypt: Scrypt,
}

#[derive(Deserialize)]
struct Pbkdf2 {
    rounds: u32,
    hash: String,
}

#[allow(non_snake_case)]
#[derive(Deserialize)]
struct Scrypt {
    N: u32,
    p: u32,
    r: u32,
    buflen: usize,
    _control: String,
}

#[derive(Debug, Serialize)]
struct Solution {
    sha256: String,
    hmac: String,
    pbkdf2: String,
    scrypt: String,
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
            "https://hackattic.com/challenges/password_hashing/problem?access_token={}",
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
            "https://hackattic.com/challenges/password_hashing/solve?access_token={}",
            API_KEY
        ))
        .body(serialized_solution)
        .send()?;

    Ok(response)
}

fn solve(problem: Problem) -> anyhow::Result<Solution> {
    let salt = BASE64_STANDARD.decode(problem.salt)?;
    let password = problem.password;

    let sha256 = hexlify!(sha256(&password));
    let hmac = hexlify!(hmac(&password, &salt)?);
    let pbkdf2 = hexlify!(pbkdf2::<32>(&password, &salt, &problem.pbkdf2)?);
    let scrypt = hexlify!(scrypt(&password, &salt, &problem.scrypt)?);

    let solution = Solution {
        sha256,
        hmac,
        pbkdf2,
        scrypt,
    };

    Ok(solution)
}

fn sha256(password: &str) -> Vec<u8> {
    use sha2::Digest as _;

    sha2::Sha256::digest(password).iter().copied().collect()
}

fn hmac(password: &str, salt: &[u8]) -> anyhow::Result<Vec<u8>> {
    use hmac::Mac as _;
    use sha2::digest::KeyInit;
    use sha2::digest::Update;

    let mut hasher = <hmac::Hmac<sha2::Sha256> as KeyInit>::new_from_slice(salt)?;
    Update::update(&mut hasher, password.as_bytes());
    let hmac = hasher.finalize().into_bytes().iter().copied().collect();

    Ok(hmac)
}

fn pbkdf2<const N: usize>(password: &str, salt: &[u8], config: &Pbkdf2) -> anyhow::Result<[u8; N]> {
    Ok(match &*config.hash {
        "sha256" => pbkdf2::pbkdf2_array::<pbkdf2::hmac::Hmac<sha2::Sha256>, N>(
            password.as_bytes(),
            salt,
            config.rounds,
        )?,
        _ => unimplemented!(), // As of writing this code, the endpoint only returned "sha256"
    })
}

fn scrypt(password: &str, salt: &[u8], config: &Scrypt) -> anyhow::Result<Vec<u8>> {
    let &Scrypt {
        N, p, r, buflen, ..
    } = config;
    let log_n = f32::log(N as f32, 2.0) as u8;
    let mut digest = vec![0u8; buflen];
    let params = scrypt::Params::new(log_n, r, p, buflen)?;

    scrypt::scrypt(password.as_bytes(), salt, &params, &mut digest).unwrap();

    Ok(digest)
}
