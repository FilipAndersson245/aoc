use anyhow::Result;
use helper::AOCError;
use hex::ToHex;
use md5::{Digest, Md5};
use std::fmt;

pub fn p1(input: &str) -> Result<impl ToString, AOCError> {
    let mut counter = 0;
    let sh = Md5::new();
    loop {
        let mut sh = sh.clone();
        let test_key = fmt::format(format_args!("{}{}", input, counter));
        sh.update(test_key);
        let result = sh.finalize();

        let s = result.encode_hex::<String>();

        if s.starts_with("00000") {
            break;
        }

        counter += 1;
    }
    Ok(counter)
}

pub fn p2(input: &str) -> Result<impl ToString, AOCError> {
    let mut counter = 0;
    let sh = Md5::new();
    loop {
        let mut sh = sh.clone();
        let test_key = fmt::format(format_args!("{}{}", input, counter));
        sh.update(test_key);
        let result = sh.finalize();

        let s = result.encode_hex::<String>();

        if s.starts_with("000000") {
            break;
        }

        counter += 1;
    }
    Ok(counter)
}
