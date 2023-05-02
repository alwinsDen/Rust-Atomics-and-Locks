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
    // scoped threads.
    //using thread::scope.
    // It allows us to spawn threads that cant outlive the scope of the closure we pass the
    // function, making it possible to safely borrow local variables.
    // BUT this will work only as long as the borrowed value is not modified in any of the
    // previous scopes.
    let numbers = Vec::from_iter(1..=10);
    thread::scope(|s| {
        s.spawn(|| {
            println!("Length: {}", numbers.len());
        });
        s.spawn(|| {
            for n in &numbers {
                println!("{}", n);
            }
        });
    })
}
