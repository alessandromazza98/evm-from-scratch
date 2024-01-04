/**
 * EVM From Scratch
 * Rust template
 *
 * To work on EVM From Scratch in Rust:
 *
 * - Install Rust: https://www.rust-lang.org/tools/install
 * - Edit `rust/lib.rs`
 * - Run `cd rust && cargo run` to run the tests
 *
 * Hint: most people who were trying to learn Rust and EVM at the same
 * gave up and switched to JavaScript, Python, or Go. If you are new
 * to Rust, implement EVM in another programming language first.
 */
use evm::evm;
use primitive_types::U256;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Evmtest {
    name: String,
    hint: String,
    code: Code,
    tx: Option<TxDataRaw>,
    block: Option<BlockDataRaw>,
    expect: Expect,
}

#[derive(Debug, Deserialize)]
struct Code {
    asm: String,
    bin: String,
}

#[derive(Debug, Deserialize)]
struct TxDataRaw {
    to: Option<String>,
    from: Option<String>,
    origin: Option<String>,
    gasprice: Option<String>,
}

#[derive(Debug, Deserialize)]
struct BlockDataRaw {
    basefee: Option<String>,
    coinbase: Option<String>,
    timestamp: Option<String>,
    number: Option<String>,
    difficulty: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Expect {
    stack: Option<Vec<String>>,
    success: bool,
    // #[serde(rename = "return")]
    // ret: Option<String>,
}

fn main() {
    let text = std::fs::read_to_string("../evm.json").unwrap();
    let data: Vec<Evmtest> = serde_json::from_str(&text).unwrap();

    let total = data.len();

    for (index, test) in data.iter().enumerate() {
        println!("Test {} of {}: {}", index + 1, total, test.name);

        let code: Vec<u8> = hex::decode(&test.code.bin).unwrap();
        let tx = match &test.tx {
            Some(tx) => {
                // [2..] is necessary to delete the initial 0x
                let to = hex::decode(format!(
                    "{:0>64}",
                    &tx.to.as_ref().unwrap_or(&String::from("aa"))[2..]
                ))
                .unwrap();
                let from = hex::decode(format!(
                    "{:0>64}",
                    &tx.from.as_ref().unwrap_or(&String::from("aa"))[2..]
                ))
                .unwrap();
                let origin = hex::decode(format!(
                    "{:0>64}",
                    &tx.origin.as_ref().unwrap_or(&String::from("aa"))[2..]
                ))
                .unwrap();
                let gasprice = hex::decode(format!(
                    "{:0>64}",
                    &tx.gasprice.as_ref().unwrap_or(&String::from("aa"))[2..]
                ))
                .unwrap();

                vec![to, from, origin, gasprice]
            }
            None => vec![],
        };

        let block = match &test.block {
            Some(block) => {
                // [2..] is necessary to delete the initial 0x
                let basefee = hex::decode(format!(
                    "{:0>64}",
                    &block.basefee.as_ref().unwrap_or(&String::from("aa"))[2..]
                ))
                .unwrap();
                let coinbase = hex::decode(format!(
                    "{:0>64}",
                    &block.coinbase.as_ref().unwrap_or(&String::from("aa"))[2..]
                ))
                .unwrap();
                let timestamp = hex::decode(format!(
                    "{:0>64}",
                    &block.timestamp.as_ref().unwrap_or(&String::from("aa"))[2..]
                ))
                .unwrap();
                let number = hex::decode(format!(
                    "{:0>64}",
                    &block.number.as_ref().unwrap_or(&String::from("aa"))[2..]
                ))
                .unwrap();
                let difficulty = hex::decode(format!(
                    "{:0>64}",
                    &block.difficulty.as_ref().unwrap_or(&String::from("aa"))[2..]
                ))
                .unwrap();

                vec![basefee, coinbase, timestamp, number, difficulty]
            }
            None => vec![],
        };

        let result = evm(&code, tx, block);

        let mut expected_stack: Vec<U256> = Vec::new();
        if let Some(ref stacks) = test.expect.stack {
            for value in stacks {
                expected_stack.push(U256::from_str_radix(value, 16).unwrap());
            }
        }

        let mut matching = result.stack.len() == expected_stack.len();
        if matching {
            for i in 0..result.stack.len() {
                if result.stack[i] != expected_stack[i] {
                    matching = false;
                    break;
                }
            }
        }

        matching = matching && result.success == test.expect.success;

        if !matching {
            println!("Instructions: \n{}\n", test.code.asm);

            println!("Expected success: {:?}", test.expect.success);
            println!("Expected stack: [");
            for v in expected_stack {
                println!("  {:#X},", v);
            }
            println!("]\n");

            println!("Actual success: {:?}", result.success);
            println!("Actual stack: [");
            for v in result.stack {
                println!("  {:#X},", v);
            }
            println!("]\n");

            println!("\nHint: {}\n", test.hint);
            println!("Progress: {}/{}\n\n", index, total);
            panic!("Test failed");
        }
        println!("PASS");
    }
    println!("Congratulations!");
}
