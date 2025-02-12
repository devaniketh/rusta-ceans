fn main () {
    let flag = true ;
    let val = match flag{
        true => 1,
        false => 0,
    };
    println!("The value of {flag} is {val}")
}