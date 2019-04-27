use std::sync::{Arc, atomic::{AtomicPtr, Ordering}};
use std::ptr;

#[inline]
fn arc2ptr<T>(data: Arc<T>) -> *mut T {
    let ptr = Arc::into_raw(data);
    ptr as *mut _
}
#[inline]
fn ptr2arc<T>(ptr: *mut T) -> Arc<T> {
    unsafe { Arc::from_raw(ptr as *const _) }
}

#[derive(Debug)]
pub struct ArcCell<T> {
    ptr: AtomicPtr<T>,
}

impl<T> ArcCell<T> {
    pub fn new(arc: Arc<T>) -> Self {
        let ptr = AtomicPtr::from(arc2ptr(arc));
        Self { ptr }
    }

    pub fn load(&self, ord: Ordering) -> Arc<T> {
        let arc = ptr2arc(self.ptr.load(ord));
        let rest = Arc::clone(&arc);
        std::mem::forget(arc);
        // Arc::into_raw(data);
        rest
    }

    pub fn get(&self) -> Arc<T> {
        self.load(Ordering::Relaxed)
    }

    pub fn store(&self, data: Arc<T>, ord: Ordering) -> Arc<T> {
        let old = self.get();
        let new = arc2ptr(data);
        self.ptr.store(new, ord);
        old
    }
    
    pub fn set(&self, data: Arc<T>) -> Arc<T> {
        self.store(data, Ordering::SeqCst)
    }
    
    pub fn compare_and_swap(&self, current: Arc<T>, new: Arc<T>, ord: Ordering) -> Arc<T> {
        let cp = arc2ptr(current);
        let np = arc2ptr(new);
        let rp = self.ptr.compare_and_swap(cp, np, ord);

        println!("cas({:p}, {:p}) -> {:p}", cp, np, rp);
        // drop current arc<T>
        ptr2arc(cp);

        if ptr::eq(cp, rp) {
            ptr2arc(rp)
        } else {
            println!("cas({:p}, {:p}) falied: {:p}", cp, np, rp);
            ptr2arc(np)
        }
    }

    pub fn into_inner(self) -> Arc<T> {
        ptr2arc(self.ptr.load(Ordering::SeqCst))
    }
}

impl<T> Drop for ArcCell<T>{
    fn drop(&mut self) {
        let _data = ptr2arc(self.ptr.load(Ordering::SeqCst));
        println!("drop::<ArcCell<T>>()  Arc::strong_count: {}", Arc::strong_count(&_data));
    }
}

impl<T> From<Arc<T>> for ArcCell<T> {
    fn from(data: Arc<T>) -> Self {
        Self::new(data)
    }
}
