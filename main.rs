use std::env::consts::FAMILY;
use std::io;
use std::io::BufRead;
use num_bigint::ToBigInt;
use std::ops::{Div, Mul};
use num_traits::cast::FromPrimitive;
use num_traits::ToPrimitive;
fn input(text:&str) -> String {
    //prints text for user
    println!("{}",text);
    let mut buffer:String = "".to_string();
    //match statement to check if the user is supplying a bad unicode string
    match io::stdin().lock().read_line(&mut buffer)
    {
        Ok(..) => return String::from(buffer.trim()),
        Err(..) => return String::from("empty"),
    }
}
fn numcheck(text:&str,low:i64,high:i64) -> i64
{
    //infinite loop until broken by return
    loop
    {
        //match statement to check if the user has entered an i8
        match input(text, ).parse()
        {
            //checks if value is between given limits
            Ok(x) => {
                if (x<=high) & (x>=low)
                {
                    return x;
                }
                println!("please enter a number between {} and {}",low,high);
            }
            //throws an error if user enters non-parseable string
            Err(x) => println!("{}",x),
        }
    }
}
fn factorial(n:i64) ->num_bigint::BigInt
{
    let mut result:num_bigint::BigInt=n.to_bigint().unwrap();
    for x in (2..n)
    {
        //println!("n is {} and x is {}",n,x);
        result *= x;
    }
    return result;
}
fn c(high:i64,low:i64)->num_bigint::BigInt
{
    return divfact(high,low)/factorial(high-low);
}
fn divfact(high:i64, low:i64) -> num_bigint::BigInt
{
    let mut result:num_bigint::BigInt= high.to_bigint().unwrap();
    for x in (low +1..high)
        {
        //println!("n is {} and x is {} and result is {}", high, x, result);
        result *= x;
        }
    return result
}
fn main() {

    let n:i64=numcheck("how many times is it repeated?",1,10000);//number of trials
    let capital_p;
    let x:i64=numcheck("how many times is it successful?",1,10000);//number of successful events
    let many=numcheck("chance (eg for 1 in 6 chance put 1)",1,10000);
    let many2=numcheck("chance2 (eg for 1 in 6 chance put 6)",1,10000);
    let mut lower_p:num_rational::BigRational= num_rational::BigRational::div(num_rational::BigRational::from_i64(many).unwrap(),num_rational::BigRational::from_i64(many2).unwrap()) ;//chance of single successful event
    let mut q:num_rational::BigRational= num_rational::BigRational::div(num_rational::BigRational::from_i64(many2-many).unwrap(),num_rational::BigRational::from_i64(many2).unwrap()) ;//chance of not happening
    lower_p=lower_p.pow(x as i32);
    q=q.pow((n-x) as i32);
    capital_p=(lower_p.mul(num_rational::BigRational::from(c(n,x))).mul(q));
    println!("{}% chance",num_rational::BigRational::to_f64(&capital_p.mul(num_rational::BigRational::from_i64(100).unwrap())).unwrap());
}
