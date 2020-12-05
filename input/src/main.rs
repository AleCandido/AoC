use crypto::buffer::{BufferResult, ReadBuffer, WriteBuffer};
use crypto::{aes, blockmodes, buffer, scrypt, symmetriccipher};

use reqwest::header::COOKIE;

fn decrypt(
    encrypted_data: &[u8],
    key: &[u8],
    iv: &[u8],
) -> Result<Vec<u8>, symmetriccipher::SymmetricCipherError> {
    let mut decryptor =
        aes::cbc_decryptor(aes::KeySize::KeySize256, key, iv, blockmodes::PkcsPadding);

    let mut final_result = Vec::<u8>::new();
    let mut read_buffer = buffer::RefReadBuffer::new(encrypted_data);
    let mut buffer = [0; 4096];
    let mut write_buffer = buffer::RefWriteBuffer::new(&mut buffer);

    loop {
        let result = decryptor.decrypt(&mut read_buffer, &mut write_buffer, true)?;
        final_result.extend(
            write_buffer
                .take_read_buffer()
                .take_remaining()
                .iter()
                .map(|&i| i),
        );
        match result {
            BufferResult::BufferUnderflow => break,
            BufferResult::BufferOverflow => {}
        }
    }

    Ok(final_result)
}

fn main() {
    let client = reqwest::blocking::Client::new();
    let resp = client
        .get("https://adventofcode.com/2020/day/1/input")
        .header(COOKIE, "")
        .send()
        .unwrap();
    println!("{:#?}", resp.text().unwrap());
}
