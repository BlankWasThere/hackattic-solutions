# [Tales of SSL](https://hackattic.com/challenges/tales_of_ssl)

## Problem
> Your task is to programmatically generate a [self-signed] certificate according to the data you receive from the challenge endpoint.
>
> Things you may be asked to include in the certificate:
>
>     a specific country as the organization's country
>     a specific certificate serial number
>     the domains the certificate should be valid for
>     specific valid from & to dates
>
> Encode the certificate in DER format with base64 and POST it to the solution endpoint.

## How to run?
> **Note:** This program _may_ fail some times because I could not find any crate to map _all_ the possible country names to ISO country codes. You can just rerun the code till the input country works.

**Note:** Make sure you have a [Rust compiler](https://rust-lang.org/), [OpenSSL](https://docs.rs/openssl/0.10.74/openssl/index.html#automatic) libraries and header files, and C++ Build tools installed.
1. Rename `.env.example` to `.env`.
2. Fill the configuration in the `.env` file.
3. Run the command `cargo run`.

## For Windows users
- I have provided _vcpkg_ manifest, so you can just use the command `vcpkg install` to download and build OpenSSL.
- Make sure to set `OPENSSL_DIR` environment variable to the directory where the library files are located (it must contain _lib_ and _include_ directories).
- Make sure to put OpenSSL DLLs in your PATH (e.g. inside *target/[debug|release]* folder).

## Why not use the [rcgen](https://github.com/rustls/rcgen) crate?
The backends of the crate (ring/aws-lc-rs) do not support key size less than 2048 bits, while hackattic provides a 1024 bit RSA PKCS#1 key. So I had to pull out the `openssl` crate for this...
