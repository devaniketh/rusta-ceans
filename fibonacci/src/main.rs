fn fibonnaci(n: u32) -> u32 {
    if n<2 {
        return n;
    } else {
        return fibonnaci(n-1) + fibonnaci(n-2);
    }
}

fn main () {
    let n = 4;
    println!("Fibonnaci of {n} = {}",fibonnaci(n))
}