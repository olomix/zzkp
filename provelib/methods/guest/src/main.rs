use risc0_zkvm::guest::env;

fn main() {
    // read the input
    let x: u32 = env::read();
    let y: u32 = env::read();

    let z = x.pow(y);

    // write public output to the journal
    env::commit(&z);
    env::commit(&x);
}
