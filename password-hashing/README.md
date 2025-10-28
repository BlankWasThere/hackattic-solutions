# [Password hashing](https://hackattic.com/challenges/password_hashing)

## Problem
> Password hashing has come a long way.
>
> The task is straightforward. You'll be given a password, some salt (watch out, it comes base64 encoded, because in this case salt - for extra high entropy - is basically just _/dev/urandom_ bytes), and some algorithms-specific parameters.
>
> Your job is to calculate the required `SHA256`, `HMAC-SHA256`, `PBKDF-SHA256` and finally `scrypt`.
>
> There's a secret step here, though you won't get points for it and the reward is englightenment itself: realize how each step uses the previous one on the way to the final result.

## How to run?
**Note:** Make sure you have a [Rust compiler](https://rust-lang.org/) installed.
1. Rename `.env.example` to `.env`.
2. Fill the configuration in the `.env` file.
3. Run the command `cargo run --release` (Release mode due to CPU intensive nature of task).
