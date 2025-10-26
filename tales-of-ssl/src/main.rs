use base64::prelude::*;
use openssl::{hash::MessageDigest, x509::X509Builder};
use serde::{Deserialize, Serialize};
use std::str::FromStr as _;

const API_KEY: &str = dotenvy_macro::dotenv!("HACKATTIC_API_KEY");

#[derive(Deserialize)]
struct Problem {
    private_key: String,
    required_data: RequiredData,
}

#[derive(Deserialize)]
struct RequiredData {
    domain: String,
    serial_number: String,
    country: String,
}

#[derive(Serialize)]
struct Solution {
    certificate: String,
}

fn main() -> anyhow::Result<()> {
    println!("> Fetching problem...");
    let problem = fetch_problem()?;

    println!("> Finding solution...");
    let solution = solve(problem)?;
    println!("[!] Found solution!");

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
            "https://hackattic.com/challenges/tales_of_ssl/problem?access_token={}",
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
            "https://hackattic.com/challenges/tales_of_ssl/solve?access_token={}",
            API_KEY
        ))
        .body(serialized_solution)
        .send()?;

    Ok(response)
}

fn solve(problem: Problem) -> anyhow::Result<Solution> {
    let Problem {
        private_key,
        required_data,
    } = problem;

    println!("> Generating certiificate...");
    let certificate = generate_certificate(private_key, required_data)?;
    let encoded_certificate = encode_certificate(certificate)?;

    let solution = Solution {
        certificate: encoded_certificate,
    };

    Ok(solution)
}

fn generate_certificate(
    private_key: String,
    required_data: RequiredData,
) -> anyhow::Result<openssl::x509::X509> {
    let country_name = required_data.country;
    let country_name_squashed = country_name.replace(" ", "").to_ascii_lowercase();
    let country_code = match celes::Country::from_str(&country_name_squashed) {
        Ok(country) => country,
        Err(_) => {
            if country_name_squashed == "tokelauislands" {
                celes::Country::tokelau()
            } else {
                return Err(anyhow::anyhow!(
                    "Could not find country code for {} ({:?}). Try again.",
                    country_name,
                    country_name_squashed
                ));
            }
        }
    }
    .alpha2;

    let mut x509_name = openssl::x509::X509NameBuilder::new()?;
    x509_name.append_entry_by_text("C", country_code)?;
    x509_name.append_entry_by_text("CN", &required_data.domain)?;

    let x509_name = x509_name.build();
    let pkey = openssl::pkey::PKey::private_key_from_der(&BASE64_STANDARD.decode(&private_key)?)?;
    let mut x509 = X509Builder::new()?;

    x509.set_pubkey(&pkey)?;
    x509.set_not_after(&*openssl::asn1::Asn1Time::days_from_now(10)?)?;
    x509.set_not_before(&*openssl::asn1::Asn1Time::days_from_now(0)?)?;
    x509.set_subject_name(&x509_name)?;
    x509.set_serial_number(&*openssl::asn1::Asn1Integer::from_bn(
        &*openssl::bn::BigNum::from_hex_str(&required_data.serial_number[2..])?,
    )?)?;

    // Self sign the certificate
    x509.sign(&pkey, MessageDigest::sha256())?;
    let cert = x509.build();

    Ok(cert)
}

fn encode_certificate(certificate: openssl::x509::X509) -> anyhow::Result<String> {
    let der_encoded = certificate.to_der()?;
    let b64_der_encoded = BASE64_STANDARD.encode(&der_encoded);

    Ok(b64_der_encoded)
}
