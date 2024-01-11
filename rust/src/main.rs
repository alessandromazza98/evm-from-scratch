use std::collections::HashMap;

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
use evm::{evm, Log};
use primitive_types::U256;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Evmtest {
    name: String,
    hint: String,
    code: Code,
    tx: Option<TxDataRaw>,
    block: Option<BlockDataRaw>,
    #[serde(default)]
    state: StateRaw,
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
    value: Option<String>,
    data: Option<String>,
}

#[derive(Debug, Deserialize)]
struct BlockDataRaw {
    basefee: Option<String>,
    coinbase: Option<String>,
    timestamp: Option<String>,
    number: Option<String>,
    difficulty: Option<String>,
    gaslimit: Option<String>,
    chainid: Option<String>,
}

#[derive(Debug, Deserialize, Default)]
struct StateRaw {
    #[serde(flatten)]
    state: HashMap<String, AddressData>,
}

#[derive(Debug, Deserialize)]
struct AddressData {
    nonce: Option<String>,
    balance: Option<String>,
    code: Option<Code>,
}

#[derive(Debug, Deserialize)]
struct Expect {
    stack: Option<Vec<String>>,
    success: bool,
    logs: Option<Vec<LogRaw>>,
    #[serde(rename = "return")]
    ret: Option<String>,
}

#[derive(Debug, Deserialize)]
struct LogRaw {
    address: String,
    data: String,
    topics: Vec<String>,
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
                let value = hex::decode(format!(
                    "{:0>64}",
                    &tx.value.as_ref().unwrap_or(&String::from("aa"))[2..]
                ))
                .unwrap();
                let data = hex::decode(format!(
                    "{:0>64}",
                    &tx.data.as_ref().unwrap_or(&String::from("aa"))[2..]
                ))
                .unwrap();

                vec![to, from, origin, gasprice, value, data]
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
                let gaslimit = hex::decode(format!(
                    "{:0>64}",
                    &block.gaslimit.as_ref().unwrap_or(&String::from("aa"))[2..]
                ))
                .unwrap();
                let chainid = hex::decode(format!(
                    "{:0>64}",
                    &block.chainid.as_ref().unwrap_or(&String::from("aa"))[2..]
                ))
                .unwrap();

                vec![
                    basefee, coinbase, timestamp, number, difficulty, gaslimit, chainid,
                ]
            }
            None => vec![],
        };

        let state = if !&test.state.state.is_empty() {
            let state = &test.state;
            let mut state_map = HashMap::new();
            for (address, data) in &state.state {
                let address = hex::decode(format!("{:0>64}", &address[2..])).unwrap();
                let nonce = &data
                    .nonce
                    .clone()
                    .unwrap_or(0.to_string())
                    .parse::<usize>()
                    .unwrap();
                let balance = hex::decode(format!(
                    "{:0>64}",
                    &data.balance.as_ref().unwrap_or(&String::from("aa"))[2..]
                ))
                .unwrap();
                let code = hex::decode(
                    &data
                        .code
                        .as_ref()
                        .unwrap_or(&Code {
                            asm: "".to_string(),
                            bin: "".to_string(),
                        })
                        .bin,
                )
                .unwrap();
                state_map.insert(address, (nonce.clone(), balance, code));
            }
            state_map
        } else {
            HashMap::default()
        };

        let result = evm(&code, tx, block, state);

        let mut expected_stack: Vec<U256> = Vec::new();
        if let Some(ref stacks) = test.expect.stack {
            for value in stacks {
                expected_stack.push(U256::from_str_radix(value, 16).unwrap());
            }
        }

        let mut expected_logs: Vec<Log> = Vec::new();
        if let Some(ref logs) = test.expect.logs {
            for log in logs {
                let LogRaw {
                    address,
                    data,
                    topics,
                } = log;
                let address = U256::from_str_radix(address, 16).unwrap();
                let data = hex::decode(format!("{}", &data)).unwrap();
                let topics = topics
                    .iter()
                    .map(|topic| U256::from_str_radix(topic, 16).unwrap())
                    .collect();
                let log = Log {
                    address,
                    data,
                    topics,
                };
                expected_logs.push(log);
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
            for i in 0..result.logs.len() {
                if result.logs[i] != expected_logs[i] {
                    matching = false;
                    break;
                }
            }
        }

        matching = matching && result.success == test.expect.success;

        let mut expected_ret = vec![];
        match &test.expect.ret {
            Some(ret) => {
                let ret = hex::decode(format!("{}", &ret)).unwrap();
                expected_ret = ret;
            }
            None => {}
        };

        matching = matching && result.ret == expected_ret;

        if !matching {
            println!("Instructions: \n{}\n", test.code.asm);

            println!("Expected success: {:?}", test.expect.success);
            println!("Expected stack: [");
            for v in expected_stack {
                println!("  {:#X},", v);
            }
            println!("]\n");
            println!("Expected logs: [");
            for l in expected_logs {
                println!("  {:#?},", l);
            }
            println!("]\n");

            println!("Expected return data:");
            println!("{:#?}", expected_ret);

            println!("Actual success: {:?}", result.success);
            println!("Actual stack: [");
            for v in result.stack {
                println!("  {:#X},", v);
            }
            println!("]\n");
            println!("Actual logs: [");
            for l in result.logs {
                println!("  {:#?},", l);
            }
            println!("]\n");

            println!("Actual return data:");
            println!("{:#?}", result.ret);

            println!("\nHint: {}\n", test.hint);
            println!("Progress: {}/{}\n\n", index, total);
            panic!("Test failed");
        }
        println!("PASS");
    }
    println!("Congratulations!");
}
