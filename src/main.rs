//Low level Concurrency in Rust
//Notion Sheet: https://scratched-salute-050.notion.site/OS-and-Compilers-in-Rust-8791003a4cd4454183ff3dadb8456b63?pvs=4

// CHAPTER 1
use std::thread;
fn main(){
    //getting value back from a thread.
    self::snip3();
    //using closures instead of Funcs()
    self::snip2();
    //basics of concurrency in Rust.
    self::snip1();
}
fn snip3(){
    let numbers = Vec::from_iter(0..=100);
    let t = thread::spawn(move ||{
        let len = numbers.len();
        let sum = numbers.into_iter().sum::<usize>();
        return sum/len;
    });
    let average = t.join().unwrap();
    println!("{average}");
}
fn snip2(){
    let numbers = vec![1,2,3,4];
    thread::spawn(||{
        //numbers in captured here.
        //move key word moves the owner ship to the closure.
        for n in numbers{
            println!("{n}");
        }
    }).join().unwrap();
}
fn snip1(){
    let t1 = thread::spawn(f);
    let t2 = thread::spawn(f);
    println!("Hello from the main thread!");
    t1.join().unwrap();
    t2.join().unwrap();
}
fn f(){
    println!("Hello from another thread!");
    let id = thread::current().id(); //here the this function is exited before completion
    //because the main thread func has finished.
    println!("This here is my thread id! {id:?}");
    //we add JoinHandle to combat aforementioned issue
}