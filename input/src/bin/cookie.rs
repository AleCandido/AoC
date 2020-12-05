use ..::io
fn main() {
    let message = "Hello World!";

    let key: [u8; 32] = [0; 32];
    let iv: [u8; 16] = [0; 16];

    let encrypted_data = encrypt(message.as_bytes(), &key, &iv).ok().unwrap();
    println!("{:#?}", encrypted_data);
}
