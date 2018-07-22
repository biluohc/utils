## Only Linux 4.x and the fs has ctime supporting.

### Toml

```toml
[target.'cfg(target_os = "linux")'.dependencies.statx]
# path = "../statx"
git = "https://github.com/biluohc/utils"
version = "^0.1"
branch = "master"
```

### cfg

```rust
#[cfg(target_os = "linux")]
```

###  Usage

```sh
cargo install -f --git https://github.com/biluohc/utils --bin statx

statx 
```

### nm & ldd
```sh
nm -g --defined-only $CACHE/libstatx.a 

statx.o:
0000000000000000 T statxf
```

```sh
ldd /home/mxo/.cache/mozilla/cargo/release/statx
	linux-vdso.so.1 (0x00007ffd0ac76000)
	libdl.so.2 => /lib64/libdl.so.2 (0x00007fa84c601000)
	librt.so.1 => /lib64/librt.so.1 (0x00007fa84c3f9000)
	libpthread.so.0 => /lib64/libpthread.so.0 (0x00007fa84c1da000)
	libgcc_s.so.1 => /lib64/libgcc_s.so.1 (0x00007fa84bfc2000)
	libc.so.6 => /lib64/libc.so.6 (0x00007fa84bc02000)
	/lib64/ld-linux-x86-64.so.2 (0x00007fa84ca73000)
	libm.so.6 => /lib64/libm.so.6 (0x00007fa84b86f000)
```

### Reference
[http://man7.org/linux/man-pages/man2/statx.2.html](http://man7.org/linux/man-pages/man2/statx.2.html)

[https://raw.githubusercontent.com/torvalds/linux/master/samples/statx/test-statx.c](https://raw.githubusercontent.com/torvalds/linux/master/samples/statx/test-statx.c)

[https://github.com/torvalds/linux/blob/master/include/uapi/linux/stat.h](https://github.com/torvalds/linux/blob/master/include/uapi/linux/stat.h)