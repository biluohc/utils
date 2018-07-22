#define _GNU_SOURCE
#define _ATFILE_SOURCE
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <ctype.h>
#include <errno.h>
#include <time.h>
#include <sys/syscall.h>
#include <sys/types.h>
#include <linux/stat.h>
#include <linux/fcntl.h>
#include <sys/stat.h>

// http://man7.org/linux/man-pages/man2/statx.2.html

// static __attribute__((unused))
ssize_t statxf(int dirfd, const char *filename, int flags,
	      unsigned int mask, struct statx *statxbuf)
{
	return syscall(__NR_statx, dirfd, filename, flags, mask, statxbuf);
}

// clang -c src/statx.c
// ar -r libstatx.a statx
