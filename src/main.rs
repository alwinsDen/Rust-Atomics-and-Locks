//Low level Concurrency in Rust
//Notion Sheet: https://scratched-salute-050.notion.site/OS-and-Compilers-in-Rust-8791003a4cd4454183ff3dadb8456b63?pvs=4

use std::thread;
fn main(){
    // CHAPTER 1
    //scoped Threads;
    self::snip5();
    //call a closure with Thread Builder method.
    self::snip4();
    //getting value back from a thread.
    self::snip3();
    //using closures instead of Funcs()
    self::snip2();
    //basics of concurrency in Rust.
    self::snip1();
}
fn snip5(){
    let numbers = vec![1,2,3,5,6,3];
    //PROS: 1. All the the closures exist within the same scope.
    //2. This allows borrowing variables from the environment.
    // (as long as no modification takes place.)
    //3. Closures are executed in order.
    thread::scope(|s|{
        s.spawn(||{
            println!("{}",numbers.len());
        });
        s.spawn(||{
            for n in &numbers{
                println!("{n}");
            }
        });
    });
}
fn snip4(){
    let t = thread::Builder::new().spawn(||{
        return 43i32;
    }).unwrap();
    let tst = t.join().unwrap();
    println!("{tst}");
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