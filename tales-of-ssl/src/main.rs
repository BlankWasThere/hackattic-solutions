use base64::prelude::*;
use openssl::{hash::MessageDigest, x509::X509Builder};
use serde::{Deserialize, Serialize};
use std::str::FromStr as _;

const API_KEY: &'static str = dotenvy_macro::dotenv!("HACKATTIC_API_KEY");

#[derive(Deserialize)]
struct Response {
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
    let response: Response = reqwest::blocking::get(format!(
        "https://hackattic.com/challenges/tales_of_ssl/problem?access_token={}",
        API_KEY
    ))?
    .json()?;

    let Response {
        private_key,
        required_data,
    } = response;

    println!("> Generating certiificate...");
    let cert = generate_certificate(private_key, required_data)?;
    let solution = Solution { certificate: cert };
    let solution_json = serde_json::to_string(&solution).unwrap();

    println!("> Uploading solution...");
    let client = reqwest::blocking::Client::new();
    let resp = client
        .post(format!(
            "https://hackattic.com/challenges/tales_of_ssl/solve?access_token={}",
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

fn generate_certificate(
    private_key: String,
    required_data: RequiredData,
) -> anyhow::Result<String> {
    let pkey = openssl::pkey::PKey::private_key_from_der(&BASE64_STANDARD.decode(&private_key)?)?;

    let mut x509 = X509Builder::new()?;
    x509.set_serial_number(
        &openssl::asn1::Asn1Integer::from_bn(
            &openssl::bn::BigNum::from_hex_str(&required_data.serial_number[2..]).unwrap(),
        )
        .unwrap(),
    )?;

    let country_name = required_data.country.replace(" ", "").to_ascii_lowercase();
    let country_code = match celes::Country::from_str(&country_name) {
        Ok(country) => country,
        Err(_) => {
            if country_name == "tokelauislands" {
                celes::Country::tokelau()
            } else {
                panic!(
                    "Could not find country code for {:?}. Try again...",
                    country_name
                );
            }
        }
    }
    .alpha2;

    let mut x509_name = openssl::x509::X509NameBuilder::new()?;
    x509_name.append_entry_by_text("C", country_code)?;
    x509_name.append_entry_by_text("CN", &required_data.domain)?;
    let x509_name = x509_name.build();

    x509.set_subject_name(&x509_name)?;
    x509.set_pubkey(&pkey)?;
    x509.set_not_before(&openssl::asn1::Asn1Time::days_from_now(0).unwrap())?;
    x509.set_not_after(&openssl::asn1::Asn1Time::days_from_now(10).unwrap())?;

    x509.sign(&pkey, MessageDigest::sha1())?;

    let cert = x509.build();
    let encoded = cert.to_der()?;
    let encoded = BASE64_STANDARD.encode(&encoded);

    Ok(encoded)
}
