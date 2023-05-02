use std::thread::JoinHandle;
use std::{
    cell::{Cell, RefCell, UnsafeCell},
    collections::VecDeque,
    marker::PhantomData,
    mem::{ManuallyDrop, MaybeUninit},
    ops::{Deref, DerefMut},
    ptr::NonNull,
    rc::Rc,
    sync::{
        atomic::{Ordering::*, *},
        *,
    },
    thread::{self, Thread},
};

fn main() {
    //passing closures to the thread rather than a function itself.
    let numbers: Vec<i32> = vec![1, 2, 4, 5];
    //ERROR: the code here will give an error: Since the value of numbers in moved into another
    // function.
    test(numbers);
    //now using closures. spawn method uses 'static lifetime  on its parameters hence they need
    // to exist for ever. If we dont use move, numbers will be passed as reference and could
    // cease to exists once the main() func goes out of scope.
    thread::spawn(move || {
        for n in numbers {
            println!("{}", n);
        }
    })
    .join()
    .unwrap();
}

fn test(arr: Vec<i32>) {
    println!("{:?}", arr);
}
