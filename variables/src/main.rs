const STARTING_MISSILES: u32 = 8;
const READY_AMOUNT: u32 = 2;


fn do_stuff(qty: f64, cost: f64) -> f64 {
    qty * cost
}

fn main() {

    let (missiles, ready, unused) = (STARTING_MISSILES, READY_AMOUNT, 0);

    println!("Firing {} of my {} missiles...", ready, missiles);

    println!("{} missiles left", missiles - ready)
}
