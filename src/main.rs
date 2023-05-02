use std::{
    cell::{Cell, RefCell, UnsafeCell},
    collections::VecDeque,
    marker::PhantomData,
    mem::{ManuallyDrop, MaybeUninit},
    ops::{Deref, DerefMut},
    ptr::NonNull,
    rc::Rc,
    sync::{*, atomic::{*, Ordering::*}},
    thread::{self, Thread},
};

fn main() {
    let t1 = thread::spawn(f);
    let t2 = thread::spawn(f);
    println!("Hello from the main thread.");
    //the main() thread closing before execution of rest of the functions can be resolved with
    // using the join function.
    t1.join().expect("The thread t1 failed execution.");
    t2.join().expect("The thread t2 failed execution.");
    //main() will close only after finishing this execution.
}

fn f() {
    println!("Hello from another thread!");
    let id = thread::current().id();
    println!("This here is my thread {id:?}");
}
