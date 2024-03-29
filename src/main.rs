//Low level Concurrency in Rust
//Notion Sheet: https://scratched-salute-050.notion.site/OS-and-Compilers-in-Rust-8791003a4cd4454183ff3dadb8456b63?pvs=4
use std::thread;
use std::boxed::Box;
use std::rc::Rc;
use std::sync::Arc;
use std::cell::Cell;

fn main(){
    // CHAPTER 1
    //Cell (Internal Mutability with Cells)
    let a = Cell::new(8i32);
    self::snip11(&a, &a);
    //borrowing and data-races. (Undefined Behaviour)
    self::snip10();
    //Atomically Ref Count (ARC smart pointer) and threads.
    self::snip9();
    // Reference counting. (example)
    self::snip8();
    // Leaking with Box
    self::snip7();
    // Statics from shared ownership and reference counting.
    self::snip6();
    // scoped Threads;
    self::snip5();
    // call a closure with Thread Builder method.
    self::snip4();
    // getting value back from a thread.
    self::snip3();
    // using closures instead of Funcs()
    self::snip2();
    // basics of concurrency in Rust.
    self::snip1();
}
fn snip11(a: &Cell<i32>, b: &Cell<i32>){
    //here a, b are references to the same variable.
    //here the internal mutability is changed.
    let before = a.get();
    b.set(b.get() + 1);
    let after = a.get();
    println!("{:?} {:?}",a.get(),b.get());
    if before!=after{
        println!("THIS CODE RUNS");
    }
}
fn snip10(){
    let a = [32,2,32,2];
    let b = unsafe{
        //get unchecked return an index value without len bound check;
        //if out of bound it returns random bytes.
        a.get_unchecked(4)
    };
    println!("{}", b);
}
fn snip9(){
    let a = Arc::new([1,2,4,5,5]);
    let b = a.clone(); //create a ref to real value.
    thread::spawn(||{ dbg!(a); });
    thread::spawn(||{ dbg!(b); });
}
fn snip8(){
    let a = Rc::new([1,2,3,4]);
    let b = a.clone(); //cloning is creating ref to the real value.
    //here as_ptr points to the memory address where the first element of
    // the array is stored. NOT THE VALUE
    //e.g a.as_ptr is 0x600001060050 (this varies depending on OS storage)
    assert_eq!(a.as_ptr(), b.as_ptr());
    //for getting the value
    unsafe{
        println!("{:?}", *a.as_ptr());
    }
}
fn snip7(){
    //Box::leak leaks the contents of the Box.
    //this is a static type, meaning it can be
    //borrowed across the program.
    let x : &'static [i32; 3] = Box::leak(Box::new([1,2,3]));
    let f = thread::spawn(move|| dbg!(x));
    let f= thread::spawn(move ||dbg!(x));
    f.join().unwrap();
}
fn snip6(){
    static X : [i32; 3] = [1,2,3];//static vars are never dropped.
    thread::spawn(|| dbg!(&X));
    thread::spawn(|| dbg!(&X));
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