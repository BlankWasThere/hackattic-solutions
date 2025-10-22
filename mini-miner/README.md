# [Mini miner](https://hackattic.com/challenges/mini_miner)

## Problem
> With the Bitcoin thing going strong, I figured it would be interesting to do some simplified mining.
>
> Connect to the problem endpoint. You'll receive a JSON with two attributes. One is block, which is in essence an object with a nonce value (initially empty) and a data key which contains some arbitrary data. The other attribute is difficulty - we'll get back to it in a moment.
>
> Your goal is find a nonce value that will cause the SHA256 hash of the block object to begin with difficulty zero bits. E.g. a difficulty of 14 means that the SHA256 digest needs to start with at least 14 zero bits.
>
> The hash should be calculated from a JSON-serialized block value without any whitespace. The keys needs to be in alphabetical order.
>
> Let's illustrate this on a really simple case. For a block with an empty data array and a given difficulty of 8 (so the first 8 bits of the SHA256 hash need to be all 0), a nonce value of 45 is one perfectly valid solution:
>
> `SHA256('{"data":[],"nonce":45}')` -> `00d696db4...cfb19ec2e0141`
>
> Keep in mind, difficulty is the number of 0 bits, not bytes.

## How to run?
**Note:** Make sure you have a [Rust compiler](https://rust-lang.org/) installed.
1. Rename `.env.example` to `.env`.
2. Fill the configuration in the `.env` file.
3. Run the command `cargo run`.
