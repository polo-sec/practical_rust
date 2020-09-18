fn main(){
    let x = 6;
    let x = x + 1;
    println!("{}", x")
}

/* This program first binds x to a value of 6. Then it shadows x by repeating let x =, taking the original value and 
 * adding 1 so the value of x is then 7.
 *
 * By using let, we can perform transformations on the variable but have the variable still be immutable after all 
 * the transformations have completed.
 */

