fn main() {
    let var =  6;
    if var == 6{
        println!("oh no the var is 6");
    }
    else {
    println!("yay the var isn't 6");
    }
}

// We can also do things like assigning variables based on if conditions

fn func() -> i8{
    9
}

let var = 6 + func();

let result = if var == 6 {15} else {200};
let output =
if var == 15 {
    println!("it is 15");
    9;
}else {
    println!("it is not 15");
    10;
;
