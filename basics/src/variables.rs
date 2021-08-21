const STARTING_MISSILES: u32 = 8;
const READY_AMOUNT: u32 = 2;

pub fn main() {
    println!("\nvariables...");
    fire_missiles();
    fire_missiles_more_efficiently();
}

fn fire_missiles() {
    let mut missiles = STARTING_MISSILES;
    let ready = READY_AMOUNT;
    println!("Firing {} out of {} missiles...", ready, missiles);
    missiles -= ready;
    println!("There are {} missiles left.", missiles);
}

fn fire_missiles_more_efficiently() {
    let (missiles, ready) = (STARTING_MISSILES, READY_AMOUNT);
    println!("Firing {} out of {} efficient missiles...", ready, missiles);
    println!("There are {} efficient missiles left.", missiles - ready);
}
