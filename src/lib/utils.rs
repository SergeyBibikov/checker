
use std::io::Read;
use std::io::BufReader;

pub fn print_response<T>(stream:T) where T: std::io::Read{
    let mut breader = BufReader::new(stream);
    let mut response = vec![0;20480];
    println!("{} bytes read\n",breader.read(&mut response).unwrap());
    println!("The response is\n\n{}\n==============================", std::str::from_utf8(&response).unwrap());
}
