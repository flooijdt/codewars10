fn main() {
    println!("Hello, world!");
}

fn move_zeros(arr: &[u8]) -> Vec<u8> {
    let mut vecky = arr.to_vec();
    let mut to_remove: Vec<usize> = Vec::new();
    for i in vecky.iter().enumerate() {
        if i.1 == '0' {
            to_remove.push(i.0);
        }
    }
    let mut counter = 0;
    for i in to_remove {
        vecky.remove(i - counter);
        vecky.push(0);
        counter += 1;
    }
}
