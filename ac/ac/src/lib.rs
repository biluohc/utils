use std::sync::{Arc, atomic::{AtomicPtr, Ordering}};

#[derive(Debug)]
pub struct ArcCell<T> {
    ptr: AtomicPtr<T>,
}

impl<T> ArcCell<T> {
    fn data2ptr(data: Arc<T>) -> *mut T {
        let ptr = Arc::into_raw(data);
        ptr as *mut _
    }
    fn ptr2data(ptr: &AtomicPtr<T>, ord: Ordering) -> Arc<T> {
        let ptr =  ptr.load(ord);
        unsafe { Arc::from_raw(ptr as *const _) }
    }

    pub fn new(data: Arc<T>) -> Self {
        let ptr = AtomicPtr::from(Self::data2ptr(data));
        Self { ptr }
    }

    pub fn load(&self, ord: Ordering) -> Arc<T> {
        let data = Self::ptr2data(&self.ptr, ord);
        let res = Arc::clone(&data);
        Arc::into_raw(data);
        res
    }

    pub fn get(&self) -> Arc<T> {
        self.load(Ordering::Relaxed)
    }

    pub fn store(&self, data: Arc<T>, ord: Ordering) -> Arc<T> {
        let old = self.get();
        let new = Self::data2ptr(data);
        self.ptr.store(new, ord);
        old
    }
    
    pub fn set(&self, data: Arc<T>) -> Arc<T> {
        self.store(data, Ordering::SeqCst)
    }
}

impl<T> Drop for ArcCell<T>{
    fn drop(&mut self) {
        let ptr =  self.ptr.load(Ordering::SeqCst);
        let _data = unsafe { Arc::from_raw(ptr as *const _) };
        // println!("drop::<ArcCell<T>>()  Arc::strong_count: {}", Arc::strong_count(&data))
    }
}

impl<T> From<Arc<T>> for ArcCell<T> {
    fn from(data: Arc<T>) -> Self {
        Self::new(data)
    }
}
