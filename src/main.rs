fn main() {
    let t = 33;
    let n = 3;
    let mut x: usize = 1;
    let mut y: usize = 0;
    let mut z: usize = 0;
    let mut sum: usize = 0;

    for i in 0..t-1 {
        z = z + y;
        y = x;
        x = n*z;
        sum = x + y + z;
    }
    println!("vsego krolikov: {}", sum);
}
