#[macro_use]
extern crate log;
extern crate flexi_logger;

extern crate fern;
extern crate colored;
extern crate chrono;

pub mod logfern;
pub mod logflex;

fn fern_log() {
    logfern::set(1, false).unwrap();
}

fn flex_log() {
    logflex::set(1, false).unwrap();
}

fn main() {
    fern_log(); 
    // flex_log();  // md，折腾那么久，文件支持不错，但是终端性能差很多，多线程应该不会死锁吧，223, 有空加个异步版(单线程写)

    // _____: fern,  flex, 一万条记录， cargo rr -- 10000
    // mbp__: 366.969851ms, 1.190649124s
    // test1: 511.210901ms, 1.268050752s

    let now = std::time::Instant::now();
    for i in 0..std::env::args().nth(1).and_then(|s|s.parse::<usize>().ok()).unwrap_or(10) {
        trace!("booting up, trace {}", i);
        info!("booting up, info {}", i);
        warn!("booting up, warn {}", i);
        error!("booting up, error {}", i);
    }
    println!("{:?}", now.elapsed());
}