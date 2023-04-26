use num::bigint::{BigInt, Sign};
use crate::rvm::opcodes;

pub fn assemble(code: &str) -> Vec<u8> {
    let mut result = Vec::new();
    for line in code.lines() {
        match line.split_whitespace().collect::<Vec<_>>().as_slice() {
            ["sload", ..] => {
                result.push(opcodes::SLOAD);
                let (_, string) = line.split_once(" ").unwrap();
                result.extend(string.len().to_be_bytes().iter());
                result.extend(string.bytes())
            },
            ["iload", number] => {
                result.push(opcodes::ILOAD);
                let number = number.parse::<BigInt>().expect("value is not a number");
                let value = number.to_signed_bytes_be();
                result.push(value.len().try_into().expect("value is sooo big"));
                result.extend(value.iter());
            },
            ["abort"] => result.push(opcodes::ABORT),
            ["debug"] => result.push(opcodes::DEBUG),
            ["store", name] => {
                result.push(opcodes::STORE);
                result.push(name.len().try_into().expect("value is sooo big"));
                result.extend(name.bytes())
            }
            ["fun", name] => {
                result.push(opcodes::FUN);
                result.extend(name.bytes())
            }
            ["end"] => result.push(opcodes::END),
            [] => {}
            x => panic!("a ne ebu chto delat {:?}", x)
        }
    }
    result
}