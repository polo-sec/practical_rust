fn main() {
    println!("I don't return anything!");
}

// This is how we define a custom function in Rust, make sure to specify data type 

fn hello() -> u16{
    println!("hello!");
    6; // Rust returns the final expression of the function. Here we want to return 6, so we put it last 
}

// Here we define a function with arguments, again remember data types

fn print_name(name: String){
    println!("{}", name);
}

// Pythonic Comparison: 
// def to_ip_address(ip):
fn to_ip_address(ip: String) -> IpAddr{
}
