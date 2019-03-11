use libc::syscall;
use std::thread;

// https://docs.rs/libc/0.2.50/libc/fn.syscall.html
// 不需要的参数 填0 或者 不填
// https://zhuanlan.zhihu.com/p/58285124
// 代码编译后使用 ltrace 调试，即可看到输出的 SYS_ ：ltrace -S ./target/release/syscall
fn main() {
    let pid = unsafe { syscall(39) };
    println!("Hello, world, pid: {}", pid);
    thread::park();
}
