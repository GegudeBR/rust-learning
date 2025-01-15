use rand::Rng;

fn one() {
    println!("One!");
}

fn two() {
    println!("Two!");
}

fn three() {
    println!("Three!");
}

fn main() {
    let mut rng = rand::thread_rng();
    let num = rng.gen_range(1..=3);

    match num {
        1 => one(),
        2 => two(),
        3 => three(),
        _ => panic!("Wrong Number Generated")
    };


}
