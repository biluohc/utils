#![feature(test)]
extern crate test;
#[macro_use]
extern crate lazy_static;
extern crate crossbeam;
extern crate ocb;
extern crate arc_swap;
extern crate parking_lot;
extern crate ac;

// use crossbeam::atomic::AtomicCell;
use ocb::ArcCell;
use arc_swap::ArcSwap;
use parking_lot::{Mutex, RwLock};
use ac::ArcCell as Ac2;

use std::sync::Arc;

use std::thread;
use std::time::Duration;

fn main() {
    let c2;
    {
        let ac2 = Ac2::new(Arc::new("xmmxm".to_owned()));
        c2 = ac2.get();

        println!("Hello, ac2: {:?}", ac2)
    }

    println!("{:?}", c2)
}

// 特么的，，不 copy 没法 load
// lazy_static! {
//     static ref AC: AtomicCell<Arc<String>> = AtomicCell::new(Arc::new("xmmxm".to_owned()));
// }

lazy_static! {
    static ref MA: Mutex<Arc<String>> = Mutex::new(Arc::new("xmmxm".to_owned()));
}

lazy_static! {
    static ref RWA: RwLock<Arc<String>> = RwLock::new(Arc::new("xmmxm".to_owned()));
}

lazy_static! {
    static ref OAC: ArcCell<String> = ArcCell::new(Arc::new("xmmxm".to_owned()));
}

lazy_static! {
    static ref AC2: Ac2<String> = Ac2::new(Arc::new("xmmxm".to_owned()));
}

lazy_static! {
    static ref AS: ArcSwap<String> = ArcSwap::from(Arc::new("xmmxm".to_owned()));
}

#[bench]
fn bench_aco(b: &mut test::Bencher) {
    thread::spawn(|| {
        let mut count = 0;
        loop {
            thread::sleep(Duration::from_micros(1));
            let old = OAC.get();
            let ns = Arc::new(format!("{}-{}", count, old.len()));
            OAC.set(ns);

            count += 1;
        }
    });

    b.iter(|| {
        OAC.get().len()>2
            // (0..1000)
            // .into_iter()
            // .for_each(|_| assert!(OAC.get().len()>2))
    })
}


#[bench]
fn bench_ac2(b: &mut test::Bencher) {
    thread::spawn(|| {
        let mut count = 0;
        loop {
            thread::sleep(Duration::from_micros(1));
            let old = AC2.get();
            let ns = Arc::new(format!("{}-{}", count, old.len()));
            AC2.set(ns);

            count += 1;
        }
    });

    b.iter(|| {
        AC2.get().len()>2
            // (0..1000)
            // .into_iter()
            // .for_each(|_| assert!(OAC.get().len()>2))
    })
}

#[bench]
fn bench_mutex(b: &mut test::Bencher) {
    thread::spawn(|| {
        let mut count = 0;
        loop {
            thread::sleep(Duration::from_micros(1));
            let mut old = MA.lock();
            let ns = Arc::new(format!("{}-{}", count, old.len()));
            *old = ns;

            count += 1;
        }
    });

    b.iter(|| {
        MA.lock().clone().len()>2
            // (0..1000)
            // .into_iter()
            // .for_each(|_| assert!(OAC.get().len()>2))
    })
}


#[bench]
fn bench_rwlock(b: &mut test::Bencher) {
    thread::spawn(|| {
        let mut count = 0;
        loop {
            thread::sleep(Duration::from_micros(1));
            let mut old = RWA.write();
            let ns = Arc::new(format!("{}-{}", count, old.len()));
            *old = ns;

            count += 1;
        }
    });

    b.iter(|| {
        RWA.read().clone().len()>2
            // (0..1000)
            // .into_iter()
            // .for_each(|_| assert!(OAC.get().len()>2))
    })
}

#[bench]
fn bench_as_(b: &mut test::Bencher) {
    thread::spawn(|| {
        let mut count = 0;
        loop {
            thread::sleep(Duration::from_micros(1));
            let old = AS.lease();
            let ns = Arc::new(format!("{}-{}", count, old.len()));
            AS.store(ns);

            count += 1;
        }
    });

    b.iter(|| {
        AS.lease().len()>2
            // (0..1000)
            // .into_iter()
            // .for_each(|_| assert!(OAC.get().len()>2))
    })
}