// variables
fn main() {
    let mut x =5;

    const THREE_HOURS_MINUTES:i32= 60*60*3;


    println!("the value is :{THREE_HOURS_MINUTES}");


    println!("the value of x is :{}",x);
    x = 5;
    println!("the value of x is :{x}");
}


fn main(){
    let x = 5;

    let x = x +1;
    {
        let x = x*2;
        println!("The value of inner loop: {x}")

    }  
 

 //booleans
fn main() {
    let x = true;
    let f :bool = false;
    println!("vale of x: {x}");
    println!("value of f: {f}");

}
//tuple
fn main () {

    let tup :(i32,f64,u8) =(500,5.6,2);

    let  (_x,_y,_z)= tup;

    println!("The values in the tuples: {_y}",);
}

// functions

fn num (x:i32) ->i32{
    x+1
}

fn main(){
    let x = num(2);
    println!("the value is :{x}");
}


conttrol flow 
fn main (){
    let num = 3;
    if num<5{
        println!("true statement ");

    }
    else {
        println!("False statement")
    }
}