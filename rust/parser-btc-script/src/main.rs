use bitcoin::ScriptBuf;

fn main() {
    let script = std::env::args().nth(1).unwrap();

    let script = hex::decode(&script).unwrap();
    let script = ScriptBuf::from_bytes(script);

    println!("{}", script);
}
