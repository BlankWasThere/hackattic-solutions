# [Brute force ZIP](https://hackattic.com/challenges/brute_force_zip)

## Problem
> Grab the `zip_url` from the problem endpoint, download the ZIP file. Inside, among other things that you can rummage through, is a file called `secret.txt` which contains the solution to this challenge. But the ZIP is password protected, and I'm not giving you the password.
>
> The password is between 4-6 characters long, lowercase and numeric. ASCII only.
>
> You'll probably need to brute-force your way to the `secret.txt` file. Oh, and you have 30 seconds until the problem expires.
>
> Go! Use the force!

## How to run?
**Note:** Make sure you have a [Rust compiler](https://rust-lang.org/) installed.
1. Rename `.env.example` to `.env`.
2. Fill the configuration in the `.env` file.
3. Run the command `cargo run --release` (Release mode due to the 30 seconds time limit).
