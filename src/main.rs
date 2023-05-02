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
    //returning value from a thread.
    let numbers = Vec::from_iter(0..=100);
    let t = thread::spawn(move || {
        let len = numbers.len();
        //here sum() requires explicit type mentioning.
        let sum = numbers.into_iter().sum::<usize>();
        return sum / len;
    })
    .join()
    .expect("The Thread panicked.");
    println!("{}", t);
}
