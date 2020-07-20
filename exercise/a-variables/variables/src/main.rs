const STARTING_MISSILES: i32 = 8;
const READY_AMOUNT: i32 = 2;

fn main() {
    let (mut missiles, ready) = (8, 2);

    println!("Firing {} of my {} missiles", ready, missiles);

    missiles = missiles - ready;
    let _bwebwe = "aissata";
    println!("Firing again {}", missiles - ready);

    println!("{} missiles left", missiles);
}
