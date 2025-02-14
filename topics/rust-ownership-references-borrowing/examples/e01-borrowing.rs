fn main() {
    let s1 = String::from("s1");
    transfer_ownership(s1);
    // println!(
    //     "Trying to use {} in the main scope after  transfering ownership.",
    //     s1
    // );
}

fn transfer_ownership(s: String) {
    println!("Transferred ownership of {}", s);
}
