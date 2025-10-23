# [Help me unpack](https://hackattic.com/challenges/mini_miner)

## Problem
> The challenge is to receive bytes and extract some numbers from those bytes.
>
> Connect to the problem endpoint, grab a base64-encoded pack of bytes, unpack the required values from it and send them back.
>
> The pack contains, always in the following order:
>
>   - a regular int (signed), to start off
>   - an unsigned int
>   - a short (signed) to make things interesting
>   - a float because floating point is important
>   - a double as well
>   - another double but this time in big endian (network byte order)
>
> In case you're wondering, we're using 4 byte ints, so everything is in the context of a 32-bit platform.
>
> Extract those numbers from the byte string and send them back to the solution endpoint for your reward. See the > solution section for a description of the expected JSON format.

## How to run?
**Note:** Make sure you have a [Rust compiler](https://rust-lang.org/) installed.
1. Rename `.env.example` to `.env`.
2. Fill the configuration in the `.env` file.
3. Run the command `cargo run`.
