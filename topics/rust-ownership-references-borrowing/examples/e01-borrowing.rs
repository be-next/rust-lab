fn main() {
    let s1 = String::from("s1");
    transfer_ownership(s1);
    // The following line will not compile because the ownership of s1 has been transferred just above.
    // println!(
    //     "Trying to use {} in the main scope after  transfering ownership.",
    //     s1
    // );

    let s2 = String::from("s2");
    immutable_borrow(&s2);
    println!("Using {} in the main scope after borrowing.", s2);

    let mut s3 = String::from("s3");
    mutable_borrow(&mut s3);
    println!("Using {} in the main scope after mutbale borrowing.", s3);
}

fn transfer_ownership(s: String) {
    println!("Transferred ownership of {}", s);
}

fn immutable_borrow(s: &String) {
    println!("Immutable borrow of {}", s);
}

fn mutable_borrow(s: &mut String) {
    s.push_str(" (modified)");
    println!("Mutable borrow of {}", s);
}
