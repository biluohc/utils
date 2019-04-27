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

use std::sync::{Arc, atomic::{AtomicUsize, Ordering}};

use std::thread;
use std::time::{Instant, Duration};

// 无论是从bench还是test_ac来看，性能差距应该是近一倍
// 明白了 crossbeam::atomic::ArcCell 那么设计的原因了(自旋锁)， Ac2的实现Drop不安全，导致极端情况下 abort, 不知道有什么方法避免，先cas 0也保证不了
fn main() {
    let a1 = Arc::new("x");
    let a2 = a1.clone();
    let a1p = Arc::into_raw(a1);
    let a2p =  Arc::into_raw(a2);
    let a3 = Arc::new("x");
    let a3p =  Arc::into_raw(a3);
    // clone 出来的指针一样，，cas 能做了
    println!("a1: {:p}", a1p);
    println!("a2: {:p}", a2p);
    assert!(std::ptr::eq(a1p, a2p));
    assert!(!std::ptr::eq(a1p, a3p));
    assert!(!std::ptr::eq(a2p, a3p));

    // 似乎没有判空运算，直接 abort 了，nice
    // fish: 'cargo +nightly rr' terminated by signal SIGSEGV (Address boundary error)
    // unsafe {
    //     let uac: Arc<usize> = Arc::from_raw(std::ptr::null());
    //     println!("{}", uac);
    // }

    let c2;
    {
        let ac2 = Ac2::new(Arc::new("xmmxm".to_owned()));
        c2 = ac2.get();

        println!("Hello, ac2: {:?}", ac2)
    }
    println!("{:?}", c2);


    // let now = Instant::now();
    // test_ac();
    // println!("ac : {:?}\n", now.elapsed());


    let now = Instant::now();
    test_ac2();
    println!("ac2: {:?}", now.elapsed());
}

#[derive(Debug, Clone, PartialEq)]
struct Usize(usize);

fn test_ac() {
    let uptimes = 1000;
    let ac = Arc::new(ArcCell::new(Arc::new(Usize(0))));
    let au = Arc::new(AtomicUsize::new(0));
    let rc = Arc::new(AtomicUsize::new(0));

    for idx in 0..32 {
        let ac = ac.clone();
        let au = au.clone();
        let rc = rc.clone();

        thread::spawn(move|| {
            loop {
                let auv = au.load(Ordering::Relaxed);
                let acv = ac.get().clone().0;
                assert!(acv >= auv);

                rc.fetch_add(1, Ordering::Relaxed);
                if auv >= uptimes {
                    println!("thread_{:02} exit", idx);
                    break;
                }
            }
        });
    }

    for _ in 0..uptimes {
        thread::sleep(Duration::from_millis(1));
        let oldauv = au.load(Ordering::Relaxed);
        ac.set(Arc::new(Usize(oldauv+1)));
        au.store(oldauv+1, Ordering::SeqCst);

        assert_eq!(ac.get().0, oldauv+1);
    }

    while Arc::strong_count(&ac) > 1 {
        thread::sleep(Duration::from_millis(1));        
    }

    println!("ac: {}", ac.get().0);
    println!("au: {}", au.load(Ordering::SeqCst));
    println!("rc: {}", rc.load(Ordering::SeqCst));
}

fn test_ac2() {
    let uptimes = 1000;
    let ac = Arc::new(Ac2::new(Arc::new(Usize(0))));
    let au = Arc::new(AtomicUsize::new(0));
    let rc = Arc::new(AtomicUsize::new(0));

    for idx in 0..32 {
        let ac = ac.clone();
        let au = au.clone();
        let rc = rc.clone();

        thread::spawn(move|| {
            loop {
                let auv = au.load(Ordering::Relaxed);
                let acv = ac.get().clone().0;
                if acv < auv {
                    println!("thread_{}: acv {} < auv {}", idx, acv, auv);
                }
                assert!(acv >= auv);

                rc.fetch_add(1, Ordering::Relaxed);
                if auv >= uptimes {
                    println!("thread_{:02} exit", idx);
                    break;
                }
            }
        });
    }

    for _ in 0..uptimes {
        thread::sleep(Duration::from_millis(1));
        let oldauv = au.load(Ordering::Relaxed);
        // ac.set(Arc::new(Usize(oldauv+1)));
        let oldarc = ac.get();
        assert_eq!(oldarc.0, oldauv);

        let newarc = Arc::new(Usize(oldauv+1));
        let rest = ac.compare_and_swap(oldarc.clone(), newarc.clone(), Ordering::SeqCst);
        println!("cas({:?}, {:?})", oldarc, newarc);
        assert_eq!(oldarc, rest);

        au.store(oldauv+1, Ordering::SeqCst);

        assert_eq!(ac.get().0, oldauv+1);
    }

    while Arc::strong_count(&ac) > 1 {
        thread::sleep(Duration::from_millis(1));        
    }

    println!("ac: {}", ac.get().0);
    println!("au: {}", au.load(Ordering::SeqCst));
    println!("rc: {}", rc.load(Ordering::SeqCst));
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