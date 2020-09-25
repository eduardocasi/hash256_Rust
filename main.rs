extern crate crypto;

use std::io::Read;
use std::io::Write;
use crypto::digest::Digest;
use crypto::sha2::Sha256;

fn main() {
	print!("Input a string: "); 
	std::io::stdout().flush().unwrap();
	let mut input: String = String::new();
   	let _a = std::io::stdin().read_line(&mut input).unwrap();
   	input = input.trim().parse::<String>().unwrap().to_string();
   	println!(); 
    let mut sha = Sha256::new();
    sha.input_str(&input);
    println!(" Ascii Text ..: {}", input);
    println!(" Caracteres ..: {}", input.len());
    println!(" Hash256 .....: {}", sha.result_str());
    print!("Tecle <Enter> para encerrar...");
    std::io::stdout().flush().unwrap();
    std::io::stdin().read(&mut [0u8]).unwrap();
}
