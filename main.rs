use std::env::consts::FAMILY;
use std::io;
use std::io::BufRead;
use num_bigint::{BigInt, ToBigInt};
use std::ops::{Div, Mul};
use num_traits::cast::FromPrimitive;
use num_traits::ToPrimitive;
use rayon::current_num_threads;
use rayon::prelude::*;
use concurrent_queue::ConcurrentQueue;

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
    result
}

fn ParallelFactorial(n:i64) ->num_bigint::BigInt{
    let mut result:num_bigint::BigInt=n.to_bigint().unwrap();
    let len = (n-1)/(current_num_threads() as i64);
    println!("len = {}",len);
    let extra = (n)%(current_num_threads() as i64);
    println!("extra = {}",extra);
    let q:ConcurrentQueue<BigInt> = ConcurrentQueue::bounded(current_num_threads()+1);
    (0..=(current_num_threads() as i64)).into_par_iter().for_each(|x| {
        let mut z:BigInt=BigInt::from(1 as i8);
        println!("hola para aqui");
        for i in ((x*len+i64::from(x==0))..((x+1)*len)) {
            println!("i is {}",i);
            z*=i;
        }
        println!("hola {}",z);
        q.push(z);
    });
    let mut combined=1;
    
    for i in ((n-extra+2)..=n+1){
        println!("combined is {} * {}",combined,i);
        combined *= i;
    }
    println!("combined {}",combined);
    let mut result2:num_bigint::BigInt= BigInt::from(1 as i8);//.to_bigint().unwrap();
    for i in (0..q.len()){
        result2 = result2*q.pop().unwrap();
    }
    
    //result2*=combined;
    //(2..n).par_iter_mut().for_each(|x| *result *= x);
    println!("result {}",result2);
    return result2
}

fn c(high:i64,low:i64)->num_bigint::BigInt
{
    return divfact(high,low)/factorial((high-low)+(((high-low)==0) as i64));
}
fn divfact(high:i64, low:i64) -> num_bigint::BigInt
{
    let mut result:num_bigint::BigInt= high.to_bigint().unwrap();
    for x in (low +1..high)
    {
        //println!("n is {} and x is {} and result is {}", high, x, result);
        result *= x;
    }
    result
}
fn main() {
    ParallelFactorial(19);
    //ParallelFactorial(10);
    let n:i64=numcheck("how many times is it repeated?",1,10000);//number of trials
    let x:i64=numcheck("how many times is it successful?",1,10000);//number of successful events
    let many=numcheck("chance (eg for 1 in 6 chance put 1)",1,10000);
    let many2=numcheck("chance2 (eg for 1 in 6 chance put 6)",1,10000);
    let mut lower_p:num_rational::BigRational= num_rational::BigRational::div(num_rational::BigRational::from_i64(many).unwrap(),num_rational::BigRational::from_i64(many2).unwrap()) ;//chance of single successful event
    let mut q:num_rational::BigRational= num_rational::BigRational::div(num_rational::BigRational::from_i64(many2-many).unwrap(),num_rational::BigRational::from_i64(many2).unwrap()) ;//chance of not happening
    lower_p=lower_p.pow(x as i32);
    q=q.pow((n-x) as i32);
    let capital_p=(lower_p.mul(num_rational::BigRational::from(c(n,x))).mul(q));
    println!("{}% chance",num_rational::BigRational::to_f64(&capital_p.mul(num_rational::BigRational::from_i64(100).unwrap())).unwrap());
}
#[cfg(test)]
mod tests{
    use crate::factorial;

    fn FactorialCheck()
    {
        let x = factorial(20);
        assert_eq!(x,2432902008176640000);
    }
    
    fn ParrallelFactorialCheck()
    {
        let x = factorial(20);
        assert_eq!(x,2432902008176640000);
    }
    
}
