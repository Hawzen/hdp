execve("/root/.cargo/bin/cargo", ["cargo", "run", "--bin", "client", "127.0.0.1"], 0xffffe4fe1aa0 /* 8 vars */) = 0
brk(NULL)                               = 0xaaaaf6e9a000
mmap(NULL, 8192, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_ANONYMOUS, -1, 0) = 0xffffb16ff000
faccessat(AT_FDCWD, "/etc/ld.so.preload", R_OK) = -1 ENOENT (No such file or directory)
openat(AT_FDCWD, "/etc/ld.so.cache", O_RDONLY|O_CLOEXEC) = 3
newfstatat(3, "", {st_mode=S_IFREG|0644, st_size=16266, ...}, AT_EMPTY_PATH) = 0
mmap(NULL, 16266, PROT_READ, MAP_PRIVATE, 3, 0) = 0xffffb16fb000
close(3)                                = 0
openat(AT_FDCWD, "/lib/aarch64-linux-gnu/libgcc_s.so.1", O_RDONLY|O_CLOEXEC) = 3
read(3, "\177ELF\2\1\1\0\0\0\0\0\0\0\0\0\3\0\267\0\1\0\0\0\0\0\0\0\0\0\0\0"..., 832) = 832
newfstatat(3, "", {st_mode=S_IFREG|0644, st_size=133448, ...}, AT_EMPTY_PATH) = 0
mmap(NULL, 262856, PROT_NONE, MAP_PRIVATE|MAP_ANONYMOUS, -1, 0) = 0xffffb1685000
mmap(0xffffb1690000, 197320, PROT_READ|PROT_EXEC, MAP_PRIVATE|MAP_FIXED|MAP_DENYWRITE, 3, 0) = 0xffffb1690000
munmap(0xffffb1685000, 45056)           = 0
munmap(0xffffb16c1000, 17096)           = 0
mprotect(0xffffb16a4000, 110592, PROT_NONE) = 0
mmap(0xffffb16bf000, 8192, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_FIXED|MAP_DENYWRITE, 3, 0x1f000) = 0xffffb16bf000
close(3)                                = 0
openat(AT_FDCWD, "/lib/aarch64-linux-gnu/libpthread.so.0", O_RDONLY|O_CLOEXEC) = 3
read(3, "\177ELF\2\1\1\0\0\0\0\0\0\0\0\0\3\0\267\0\1\0\0\0\0\0\0\0\0\0\0\0"..., 832) = 832
newfstatat(3, "", {st_mode=S_IFREG|0644, st_size=67528, ...}, AT_EMPTY_PATH) = 0
mmap(NULL, 196640, PROT_NONE, MAP_PRIVATE|MAP_ANONYMOUS, -1, 0) = 0xffffb165f000
mmap(0xffffb1660000, 131104, PROT_READ|PROT_EXEC, MAP_PRIVATE|MAP_FIXED|MAP_DENYWRITE, 3, 0) = 0xffffb1660000
munmap(0xffffb165f000, 4096)            = 0
munmap(0xffffb1681000, 57376)           = 0
mprotect(0xffffb1661000, 122880, PROT_NONE) = 0
mmap(0xffffb167f000, 8192, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_FIXED|MAP_DENYWRITE, 3, 0xf000) = 0xffffb167f000
close(3)                                = 0
openat(AT_FDCWD, "/lib/aarch64-linux-gnu/libm.so.6", O_RDONLY|O_CLOEXEC) = 3
read(3, "\177ELF\2\1\1\0\0\0\0\0\0\0\0\0\3\0\267\0\1\0\0\0\0\0\0\0\0\0\0\0"..., 832) = 832
newfstatat(3, "", {st_mode=S_IFREG|0644, st_size=591960, ...}, AT_EMPTY_PATH) = 0
mmap(NULL, 655472, PROT_NONE, MAP_PRIVATE|MAP_ANONYMOUS, -1, 0) = 0xffffb15bf000
mmap(0xffffb15c0000, 589936, PROT_READ|PROT_EXEC, MAP_PRIVATE|MAP_FIXED|MAP_DENYWRITE, 3, 0) = 0xffffb15c0000
munmap(0xffffb15bf000, 4096)            = 0
munmap(0xffffb1651000, 57456)           = 0
mprotect(0xffffb1640000, 61440, PROT_NONE) = 0
mmap(0xffffb164f000, 8192, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_FIXED|MAP_DENYWRITE, 3, 0x8f000) = 0xffffb164f000
close(3)                                = 0
openat(AT_FDCWD, "/lib/aarch64-linux-gnu/libdl.so.2", O_RDONLY|O_CLOEXEC) = 3
read(3, "\177ELF\2\1\1\0\0\0\0\0\0\0\0\0\3\0\267\0\1\0\0\0\0\0\0\0\0\0\0\0"..., 832) = 832
newfstatat(3, "", {st_mode=S_IFREG|0644, st_size=67528, ...}, AT_EMPTY_PATH) = 0
mmap(NULL, 196640, PROT_NONE, MAP_PRIVATE|MAP_ANONYMOUS, -1, 0) = 0xffffb158f000
mmap(0xffffb1590000, 131104, PROT_READ|PROT_EXEC, MAP_PRIVATE|MAP_FIXED|MAP_DENYWRITE, 3, 0) = 0xffffb1590000
munmap(0xffffb158f000, 4096)            = 0
munmap(0xffffb15b1000, 57376)           = 0
mprotect(0xffffb1591000, 122880, PROT_NONE) = 0
mmap(0xffffb15af000, 8192, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_FIXED|MAP_DENYWRITE, 3, 0xf000) = 0xffffb15af000
close(3)                                = 0
openat(AT_FDCWD, "/lib/aarch64-linux-gnu/libc.so.6", O_RDONLY|O_CLOEXEC) = 3
read(3, "\177ELF\2\1\1\3\0\0\0\0\0\0\0\0\3\0\267\0\1\0\0\0000y\2\0\0\0\0\0"..., 832) = 832
newfstatat(3, "", {st_mode=S_IFREG|0755, st_size=1651408, ...}, AT_EMPTY_PATH) = 0
mmap(NULL, 1826912, PROT_NONE, MAP_PRIVATE|MAP_ANONYMOUS, -1, 0) = 0xffffb13d1000
mmap(0xffffb13e0000, 1761376, PROT_READ|PROT_EXEC, MAP_PRIVATE|MAP_FIXED|MAP_DENYWRITE, 3, 0) = 0xffffb13e0000
munmap(0xffffb13d1000, 61440)           = 0
munmap(0xffffb158f000, 96)              = 0
mprotect(0xffffb1567000, 86016, PROT_NONE) = 0
mmap(0xffffb157c000, 24576, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_FIXED|MAP_DENYWRITE, 3, 0x18c000) = 0xffffb157c000
mmap(0xffffb1582000, 49248, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_FIXED|MAP_ANONYMOUS, -1, 0) = 0xffffb1582000
close(3)                                = 0
mmap(NULL, 8192, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_ANONYMOUS, -1, 0) = 0xffffb16f9000
set_tid_address(0xffffb16f9c50)         = 749
set_robust_list(0xffffb16f9c60, 24)     = 0
rseq(0xffffb16fa2a0, 0x20, 0, 0xd428bc00) = 0
mprotect(0xffffb157c000, 16384, PROT_READ) = 0
mprotect(0xffffb15af000, 4096, PROT_READ) = 0
mprotect(0xffffb164f000, 4096, PROT_READ) = 0
mprotect(0xffffb167f000, 4096, PROT_READ) = 0
mprotect(0xffffb16bf000, 4096, PROT_READ) = 0
mprotect(0xaaaae1d66000, 548864, PROT_READ) = 0
mprotect(0xffffb1704000, 8192, PROT_READ) = 0
prlimit64(0, RLIMIT_STACK, NULL, {rlim_cur=8192*1024, rlim_max=RLIM64_INFINITY}) = 0
munmap(0xffffb16fb000, 16266)           = 0
getrandom("\x87\x96\x7f\xf9\x5a\xad\x3d\xda", 8, GRND_NONBLOCK) = 8
brk(NULL)                               = 0xaaaaf6e9a000
brk(0xaaaaf6ebb000)                     = 0xaaaaf6ebb000
rt_sigprocmask(SIG_SETMASK, ~[ILL TRAP BUS FPE SEGV RTMIN RT_1], [], 8) = 0
rt_sigaction(SIGILL, {sa_handler=0xaaaae1ac76d0, sa_mask=~[ILL TRAP BUS FPE SEGV RTMIN RT_1], sa_flags=0}, {sa_handler=SIG_DFL, sa_mask=[], sa_flags=0}, 8) = 0
rt_sigprocmask(SIG_BLOCK, NULL, ~[ILL TRAP BUS FPE KILL SEGV STOP RTMIN RT_1], 8) = 0
rt_sigaction(SIGILL, {sa_handler=SIG_DFL, sa_mask=[], sa_flags=0}, NULL, 8) = 0
rt_sigprocmask(SIG_SETMASK, [], NULL, 8) = 0
futex(0xaaaae1df3828, FUTEX_WAKE_PRIVATE, 2147483647) = 0
futex(0xaaaae1df3894, FUTEX_WAKE_PRIVATE, 2147483647) = 0
futex(0xaaaae1df3858, FUTEX_WAKE_PRIVATE, 2147483647) = 0
futex(0xaaaae1df391c, FUTEX_WAKE_PRIVATE, 2147483647) = 0
futex(0xaaaae1df3818, FUTEX_WAKE_PRIVATE, 2147483647) = 0
futex(0xaaaae1df381c, FUTEX_WAKE_PRIVATE, 2147483647) = 0
futex(0xaaaae1df384c, FUTEX_WAKE_PRIVATE, 2147483647) = 0
futex(0xaaaae1df0e94, FUTEX_WAKE_PRIVATE, 2147483647) = 0
futex(0xaaaae1df37c8, FUTEX_WAKE_PRIVATE, 2147483647) = 0
futex(0xaaaae1df1710, FUTEX_WAKE_PRIVATE, 2147483647) = 0
futex(0xaaaae1df1714, FUTEX_WAKE_PRIVATE, 2147483647) = 0
futex(0xaaaae1df385c, FUTEX_WAKE_PRIVATE, 2147483647) = 0
futex(0xaaaae1df3840, FUTEX_WAKE_PRIVATE, 2147483647) = 0
futex(0xaaaae1df3848, FUTEX_WAKE_PRIVATE, 2147483647) = 0
openat(AT_FDCWD, "/usr/local/ssl/openssl.cnf", O_RDONLY) = -1 ENOENT (No such file or directory)
futex(0xaaaae1df3838, FUTEX_WAKE_PRIVATE, 2147483647) = 0
sysinfo({uptime=83955, loads=[16160, 13984, 10112], totalram=8217968640, freeram=4365352960, sharedram=3502080, bufferram=159621120, totalswap=1073737728, freeswap=1073737728, procs=324, totalhigh=0, freehigh=0, mem_unit=1}) = 0
futex(0xaaaae1df0e38, FUTEX_WAKE_PRIVATE, 2147483647) = 0
ppoll([{fd=0, events=0}, {fd=1, events=0}, {fd=2, events=0}], 3, {tv_sec=0, tv_nsec=0}, NULL, 0) = 1 ([{fd=0, revents=POLLHUP}], left {tv_sec=0, tv_nsec=0})
rt_sigaction(SIGPIPE, {sa_handler=SIG_IGN, sa_mask=[PIPE], sa_flags=SA_RESTART}, {sa_handler=SIG_DFL, sa_mask=[], sa_flags=0}, 8) = 0
rt_sigaction(SIGSEGV, NULL, {sa_handler=SIG_DFL, sa_mask=[], sa_flags=0}, 8) = 0
rt_sigaction(SIGSEGV, {sa_handler=0xaaaae18c7ed8, sa_mask=[], sa_flags=SA_ONSTACK|SA_SIGINFO}, NULL, 8) = 0
rt_sigaction(SIGBUS, NULL, {sa_handler=SIG_DFL, sa_mask=[], sa_flags=0}, 8) = 0
rt_sigaction(SIGBUS, {sa_handler=0xaaaae18c7ed8, sa_mask=[], sa_flags=SA_ONSTACK|SA_SIGINFO}, NULL, 8) = 0
sigaltstack(NULL, {ss_sp=NULL, ss_flags=SS_DISABLE, ss_size=0}) = 0
mmap(NULL, 20480, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_ANONYMOUS|MAP_STACK, -1, 0) = 0xffffb16f4000
mprotect(0xffffb16f4000, 4096, PROT_NONE) = 0
sigaltstack({ss_sp=0xffffb16f5000, ss_flags=0, ss_size=16384}, NULL) = 0
openat(AT_FDCWD, "/proc/self/maps", O_RDONLY|O_CLOEXEC) = 3
prlimit64(0, RLIMIT_STACK, NULL, {rlim_cur=8192*1024, rlim_max=RLIM64_INFINITY}) = 0
newfstatat(3, "", {st_mode=S_IFREG|0444, st_size=0, ...}, AT_EMPTY_PATH) = 0
read(3, "aaaae14c0000-aaaae1d56000 r-xp 0"..., 1024) = 1024
read(3, "91000-ffffb15af000 ---p 00001000"..., 1024) = 1024
read(3, "r--p 0000f000 00:50 159739      "..., 1024) = 1024
read(3, "0 0                          [vv"..., 1024) = 441
close(3)                                = 0
sched_getaffinity(749, 32, [0 1 2 3 4 5 6 7 8 9]) = 8
ioctl(2, TCGETS, 0xffffcf9b0db0)        = -1 ENOTTY (Inappropriate ioctl for device)
ioctl(1, TCGETS, {c_iflag=ICRNL|IXON, c_oflag=NL0|CR0|TAB0|BS0|VT0|FF0|OPOST|ONLCR, c_cflag=B38400|CS8|CREAD, c_lflag=ISIG|ICANON|ECHO|ECHOE|ECHOK|IEXTEN|ECHOCTL|ECHOKE, ...}) = 0
getcwd("/hdp", 512)                     = 5
readlinkat(AT_FDCWD, "/proc/self/exe", "/root/.cargo/bin/cargo", 256) = 22
getcwd("/hdp", 512)                     = 5
statx(AT_FDCWD, "/root/.cargo/bin/rustup-init", AT_STATX_SYNC_AS_STAT, STATX_ALL, 0xffffcf9affa0) = -1 ENOENT (No such file or directory)
statx(0, NULL, AT_STATX_SYNC_AS_STAT, STATX_ALL, NULL) = -1 EFAULT (Bad address)
getcwd("/hdp", 512)                     = 5
statx(AT_FDCWD, "/root/.rustup", AT_STATX_SYNC_AS_STAT, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFDIR|0755, stx_size=4096, ...}) = 0
openat(AT_FDCWD, "/etc/rustup/settings.toml", O_RDONLY|O_CLOEXEC) = -1 ENOENT (No such file or directory)
statx(AT_FDCWD, "/root/.rustup/settings.toml", AT_STATX_SYNC_AS_STAT, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0644, stx_size=103, ...}) = 0
openat(AT_FDCWD, "/root/.rustup/settings.toml", O_RDONLY|O_CLOEXEC) = 3
statx(3, "", AT_STATX_SYNC_AS_STAT|AT_EMPTY_PATH, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0644, stx_size=103, ...}) = 0
read(3, "default_toolchain = \"stable-aarc"..., 103) = 103
read(3, "", 32)                         = 0
close(3)                                = 0
getrandom("\x9d\xb1\x56\xe8\x78\x2e\x39\xc0\x5b\x1f\x73\x07\xf9\xae\x1b\x73", 16, GRND_INSECURE) = 16
uname({sysname="Linux", nodename="2cc7eee5130c", ...}) = 0
openat(AT_FDCWD, "/bin/sh", O_RDONLY|O_CLOEXEC) = 3
read(3, "\177ELF\2", 5)                 = 5
close(3)                                = 0
brk(0xaaaaf6edc000)                     = 0xaaaaf6edc000
mmap(NULL, 323584, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_ANONYMOUS, -1, 0) = 0xffffb1391000
munmap(0xffffb1391000, 323584)          = 0
brk(0xaaaaf6f09000)                     = 0xaaaaf6f09000
uname({sysname="Linux", nodename="2cc7eee5130c", ...}) = 0
openat(AT_FDCWD, "/bin/sh", O_RDONLY|O_CLOEXEC) = 3
read(3, "\177ELF\2", 5)                 = 5
close(3)                                = 0
statx(AT_FDCWD, "/root/.rustup", AT_STATX_SYNC_AS_STAT, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFDIR|0755, stx_size=4096, ...}) = 0
getcwd("/hdp", 512)                     = 5
statx(AT_FDCWD, "/hdp", AT_STATX_SYNC_AS_STAT, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFDIR|0755, stx_size=4096, ...}) = 0
readlinkat(AT_FDCWD, "/hdp", 0xffffcf9aea10, 1023) = -1 EINVAL (Invalid argument)
openat(AT_FDCWD, "/hdp/rust-toolchain", O_RDONLY|O_CLOEXEC) = -1 ENOENT (No such file or directory)
openat(AT_FDCWD, "/hdp/rust-toolchain.toml", O_RDONLY|O_CLOEXEC) = -1 ENOENT (No such file or directory)
statx(AT_FDCWD, "/", AT_STATX_SYNC_AS_STAT, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=STATX_ATTR_MOUNT_ROOT, stx_mode=S_IFDIR|0755, stx_size=4096, ...}) = 0
openat(AT_FDCWD, "/rust-toolchain", O_RDONLY|O_CLOEXEC) = -1 ENOENT (No such file or directory)
openat(AT_FDCWD, "/rust-toolchain.toml", O_RDONLY|O_CLOEXEC) = -1 ENOENT (No such file or directory)
uname({sysname="Linux", nodename="2cc7eee5130c", ...}) = 0
openat(AT_FDCWD, "/bin/sh", O_RDONLY|O_CLOEXEC) = 3
read(3, "\177ELF\2", 5)                 = 5
close(3)                                = 0
openat(AT_FDCWD, "/root/.rustup/toolchains", O_RDONLY|O_CLOEXEC) = 3
openat(3, "stable-aarch64-unknown-linux-gnu", O_RDONLY|O_NOCTTY|O_CLOEXEC) = 4
close(4)                                = 0
close(3)                                = 0
statx(AT_FDCWD, "/root/.rustup/toolchains/stable-aarch64-unknown-linux-gnu/lib/rustlib/rust-installer-version", AT_STATX_SYNC_AS_STAT, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0644, stx_size=1, ...}) = 0
openat(AT_FDCWD, "/root/.rustup/toolchains/stable-aarch64-unknown-linux-gnu/lib/rustlib/rust-installer-version", O_RDONLY|O_CLOEXEC) = 3
statx(3, "", AT_STATX_SYNC_AS_STAT|AT_EMPTY_PATH, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0644, stx_size=1, ...}) = 0
read(3, "3", 1)                         = 1
read(3, "", 32)                         = 0
close(3)                                = 0
statx(AT_FDCWD, "/root/.rustup/toolchains/stable-aarch64-unknown-linux-gnu/lib/rustlib/multirust-channel-manifest.toml", AT_STATX_SYNC_AS_STAT, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0644, stx_size=837765, ...}) = 0
openat(AT_FDCWD, "/root/.rustup/toolchains/stable-aarch64-unknown-linux-gnu/lib/rustlib/multirust-channel-manifest.toml", O_RDONLY|O_CLOEXEC) = 3
statx(3, "", AT_STATX_SYNC_AS_STAT|AT_EMPTY_PATH, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0644, stx_size=837765, ...}) = 0
mmap(NULL, 839680, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_ANONYMOUS, -1, 0) = 0xffffb1313000
read(3, "date = \"2025-01-30\"\nmanifest-ver"..., 837765) = 837765
read(3, "", 32)                         = 0
close(3)                                = 0
brk(0xaaaaf6f2a000)                     = 0xaaaaf6f2a000
brk(0xaaaaf6f4d000)                     = 0xaaaaf6f4d000
brk(0xaaaaf6f6e000)                     = 0xaaaaf6f6e000
brk(0xaaaaf6f8f000)                     = 0xaaaaf6f8f000
brk(0xaaaaf6fc2000)                     = 0xaaaaf6fc2000
brk(0xaaaaf6fe3000)                     = 0xaaaaf6fe3000
brk(0xaaaaf7004000)                     = 0xaaaaf7004000
brk(0xaaaaf7025000)                     = 0xaaaaf7025000
brk(0xaaaaf7046000)                     = 0xaaaaf7046000
brk(0xaaaaf7067000)                     = 0xaaaaf7067000
brk(0xaaaaf7090000)                     = 0xaaaaf7090000
brk(0xaaaaf70b1000)                     = 0xaaaaf70b1000
brk(0xaaaaf70d2000)                     = 0xaaaaf70d2000
brk(0xaaaaf70f3000)                     = 0xaaaaf70f3000
brk(0xaaaaf7118000)                     = 0xaaaaf7118000
brk(0xaaaaf7139000)                     = 0xaaaaf7139000
brk(0xaaaaf715c000)                     = 0xaaaaf715c000
brk(0xaaaaf7187000)                     = 0xaaaaf7187000
brk(0xaaaaf71a8000)                     = 0xaaaaf71a8000
brk(0xaaaaf71c9000)                     = 0xaaaaf71c9000
brk(0xaaaaf71ea000)                     = 0xaaaaf71ea000
brk(0xaaaaf720e000)                     = 0xaaaaf720e000
brk(0xaaaaf722f000)                     = 0xaaaaf722f000
brk(0xaaaaf7250000)                     = 0xaaaaf7250000
brk(0xaaaaf7271000)                     = 0xaaaaf7271000
brk(0xaaaaf7292000)                     = 0xaaaaf7292000
brk(0xaaaaf72b4000)                     = 0xaaaaf72b4000
brk(0xaaaaf72d5000)                     = 0xaaaaf72d5000
brk(0xaaaaf72f8000)                     = 0xaaaaf72f8000
brk(0xaaaaf7323000)                     = 0xaaaaf7323000
brk(0xaaaaf7344000)                     = 0xaaaaf7344000
brk(0xaaaaf7365000)                     = 0xaaaaf7365000
brk(0xaaaaf7386000)                     = 0xaaaaf7386000
brk(0xaaaaf73aa000)                     = 0xaaaaf73aa000
brk(0xaaaaf73cb000)                     = 0xaaaaf73cb000
brk(0xaaaaf73ec000)                     = 0xaaaaf73ec000
brk(0xaaaaf740d000)                     = 0xaaaaf740d000
brk(0xaaaaf742e000)                     = 0xaaaaf742e000
brk(0xaaaaf7452000)                     = 0xaaaaf7452000
brk(0xaaaaf7473000)                     = 0xaaaaf7473000
brk(0xaaaaf7496000)                     = 0xaaaaf7496000
brk(0xaaaaf74b7000)                     = 0xaaaaf74b7000
brk(0xaaaaf74d8000)                     = 0xaaaaf74d8000
brk(0xaaaaf74f9000)                     = 0xaaaaf74f9000
brk(0xaaaaf751a000)                     = 0xaaaaf751a000
brk(0xaaaaf753b000)                     = 0xaaaaf753b000
brk(0xaaaaf7566000)                     = 0xaaaaf7566000
brk(0xaaaaf7587000)                     = 0xaaaaf7587000
brk(0xaaaaf75a8000)                     = 0xaaaaf75a8000
brk(0xaaaaf75c9000)                     = 0xaaaaf75c9000
brk(0xaaaaf75ee000)                     = 0xaaaaf75ee000
brk(0xaaaaf760f000)                     = 0xaaaaf760f000
brk(0xaaaaf7630000)                     = 0xaaaaf7630000
brk(0xaaaaf7651000)                     = 0xaaaaf7651000
brk(0xaaaaf7672000)                     = 0xaaaaf7672000
brk(0xaaaaf7694000)                     = 0xaaaaf7694000
brk(0xaaaaf76b5000)                     = 0xaaaaf76b5000
brk(0xaaaaf76d8000)                     = 0xaaaaf76d8000
brk(0xaaaaf7703000)                     = 0xaaaaf7703000
brk(0xaaaaf7724000)                     = 0xaaaaf7724000
brk(0xaaaaf7745000)                     = 0xaaaaf7745000
brk(0xaaaaf7766000)                     = 0xaaaaf7766000
brk(0xaaaaf778b000)                     = 0xaaaaf778b000
brk(0xaaaaf77ac000)                     = 0xaaaaf77ac000
brk(0xaaaaf77ce000)                     = 0xaaaaf77ce000
brk(0xaaaaf77ef000)                     = 0xaaaaf77ef000
brk(0xaaaaf7810000)                     = 0xaaaaf7810000
brk(0xaaaaf7831000)                     = 0xaaaaf7831000
brk(0xaaaaf7852000)                     = 0xaaaaf7852000
brk(0xaaaaf7879000)                     = 0xaaaaf7879000
brk(0xaaaaf789a000)                     = 0xaaaaf789a000
brk(0xaaaaf78bb000)                     = 0xaaaaf78bb000
brk(0xaaaaf78dc000)                     = 0xaaaaf78dc000
brk(0xaaaaf78fd000)                     = 0xaaaaf78fd000
brk(0xaaaaf791e000)                     = 0xaaaaf791e000
brk(0xaaaaf793f000)                     = 0xaaaaf793f000
brk(0xaaaaf7960000)                     = 0xaaaaf7960000
brk(0xaaaaf7981000)                     = 0xaaaaf7981000
brk(0xaaaaf79a3000)                     = 0xaaaaf79a3000
munmap(0xffffb1313000, 839680)          = 0
statx(AT_FDCWD, "/root/.rustup/toolchains/stable-aarch64-unknown-linux-gnu/lib/rustlib/multirust-config.toml", AT_STATX_SYNC_AS_STAT, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0644, stx_size=449, ...}) = 0
openat(AT_FDCWD, "/root/.rustup/toolchains/stable-aarch64-unknown-linux-gnu/lib/rustlib/multirust-config.toml", O_RDONLY|O_CLOEXEC) = 3
statx(3, "", AT_STATX_SYNC_AS_STAT|AT_EMPTY_PATH, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0644, stx_size=449, ...}) = 0
read(3, "config_version = \"1\"\n\n[[componen"..., 449) = 449
read(3, "", 32)                         = 0
close(3)                                = 0
statx(AT_FDCWD, "/root/.rustup/toolchains/stable-aarch64-unknown-linux-gnu/bin/cargo", AT_STATX_SYNC_AS_STAT, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0755, stx_size=40937544, ...}) = 0
getcwd("/hdp", 512)                     = 5
getcwd("/hdp", 512)                     = 5
rt_sigaction(SIGPIPE, {sa_handler=SIG_DFL, sa_mask=[PIPE], sa_flags=SA_RESTART}, {sa_handler=SIG_IGN, sa_mask=[PIPE], sa_flags=SA_RESTART}, 8) = 0
execve("/root/.rustup/toolchains/stable-aarch64-unknown-linux-gnu/bin/cargo", ["/root/.rustup/toolchains/stable-"..., "run", "--bin", "client", "127.0.0.1"], 0xaaaaf6eb7110 /* 13 vars */) = 0
brk(NULL)                               = 0xaaaaeab0f000
mmap(NULL, 8192, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_ANONYMOUS, -1, 0) = 0xffffa90fe000
readlinkat(AT_FDCWD, "/proc/self/exe", "/root/.rustup/toolchains/stable-"..., 4096) = 67
faccessat(AT_FDCWD, "/etc/ld.so.preload", R_OK) = -1 ENOENT (No such file or directory)
openat(AT_FDCWD, "/root/.rustup/toolchains/stable-aarch64-unknown-linux-gnu/bin/../lib/tls/aarch64/atomics/libdl.so.2", O_RDONLY|O_CLOEXEC) = -1 ENOENT (No such file or directory)
newfstatat(AT_FDCWD, "/root/.rustup/toolchains/stable-aarch64-unknown-linux-gnu/bin/../lib/tls/aarch64/atomics", 0xffffc61c1c90, 0) = -1 ENOENT (No such file or directory)
openat(AT_FDCWD, "/root/.rustup/toolchains/stable-aarch64-unknown-linux-gnu/bin/../lib/tls/aarch64/libdl.so.2", O_RDONLY|O_CLOEXEC) = -1 ENOENT (No such file or directory)
newfstatat(AT_FDCWD, "/root/.rustup/toolchains/stable-aarch64-unknown-linux-gnu/bin/../lib/tls/aarch64", 0xffffc61c1c90, 0) = -1 ENOENT (No such file or directory)
openat(AT_FDCWD, "/root/.rustup/toolchains/stable-aarch64-unknown-linux-gnu/bin/../lib/tls/atomics/libdl.so.2", O_RDONLY|O_CLOEXEC) = -1 ENOENT (No such file or directory)
newfstatat(AT_FDCWD, "/root/.rustup/toolchains/stable-aarch64-unknown-linux-gnu/bin/../lib/tls/atomics", 0xffffc61c1c90, 0) = -1 ENOENT (No such file or directory)
openat(AT_FDCWD, "/root/.rustup/toolchains/stable-aarch64-unknown-linux-gnu/bin/../lib/tls/libdl.so.2", O_RDONLY|O_CLOEXEC) = -1 ENOENT (No such file or directory)
newfstatat(AT_FDCWD, "/root/.rustup/toolchains/stable-aarch64-unknown-linux-gnu/bin/../lib/tls", 0xffffc61c1c90, 0) = -1 ENOENT (No such file or directory)
openat(AT_FDCWD, "/root/.rustup/toolchains/stable-aarch64-unknown-linux-gnu/bin/../lib/aarch64/atomics/libdl.so.2", O_RDONLY|O_CLOEXEC) = -1 ENOENT (No such file or directory)
newfstatat(AT_FDCWD, "/root/.rustup/toolchains/stable-aarch64-unknown-linux-gnu/bin/../lib/aarch64/atomics", 0xffffc61c1c90, 0) = -1 ENOENT (No such file or directory)
openat(AT_FDCWD, "/root/.rustup/toolchains/stable-aarch64-unknown-linux-gnu/bin/../lib/aarch64/libdl.so.2", O_RDONLY|O_CLOEXEC) = -1 ENOENT (No such file or directory)
newfstatat(AT_FDCWD, "/root/.rustup/toolchains/stable-aarch64-unknown-linux-gnu/bin/../lib/aarch64", 0xffffc61c1c90, 0) = -1 ENOENT (No such file or directory)
openat(AT_FDCWD, "/root/.rustup/toolchains/stable-aarch64-unknown-linux-gnu/bin/../lib/atomics/libdl.so.2", O_RDONLY|O_CLOEXEC) = -1 ENOENT (No such file or directory)
newfstatat(AT_FDCWD, "/root/.rustup/toolchains/stable-aarch64-unknown-linux-gnu/bin/../lib/atomics", 0xffffc61c1c90, 0) = -1 ENOENT (No such file or directory)
openat(AT_FDCWD, "/root/.rustup/toolchains/stable-aarch64-unknown-linux-gnu/bin/../lib/libdl.so.2", O_RDONLY|O_CLOEXEC) = -1 ENOENT (No such file or directory)
newfstatat(AT_FDCWD, "/root/.rustup/toolchains/stable-aarch64-unknown-linux-gnu/bin/../lib", {st_mode=S_IFDIR|0755, st_size=4096, ...}, 0) = 0
openat(AT_FDCWD, "/root/.rustup/toolchains/stable-aarch64-unknown-linux-gnu/lib/tls/aarch64/atomics/libdl.so.2", O_RDONLY|O_CLOEXEC) = -1 ENOENT (No such file or directory)
newfstatat(AT_FDCWD, "/root/.rustup/toolchains/stable-aarch64-unknown-linux-gnu/lib/tls/aarch64/atomics", 0xffffc61c1c90, 0) = -1 ENOENT (No such file or directory)
openat(AT_FDCWD, "/root/.rustup/toolchains/stable-aarch64-unknown-linux-gnu/lib/tls/aarch64/libdl.so.2", O_RDONLY|O_CLOEXEC) = -1 ENOENT (No such file or directory)
newfstatat(AT_FDCWD, "/root/.rustup/toolchains/stable-aarch64-unknown-linux-gnu/lib/tls/aarch64", 0xffffc61c1c90, 0) = -1 ENOENT (No such file or directory)
openat(AT_FDCWD, "/root/.rustup/toolchains/stable-aarch64-unknown-linux-gnu/lib/tls/atomics/libdl.so.2", O_RDONLY|O_CLOEXEC) = -1 ENOENT (No such file or directory)
newfstatat(AT_FDCWD, "/root/.rustup/toolchains/stable-aarch64-unknown-linux-gnu/lib/tls/atomics", 0xffffc61c1c90, 0) = -1 ENOENT (No such file or directory)
openat(AT_FDCWD, "/root/.rustup/toolchains/stable-aarch64-unknown-linux-gnu/lib/tls/libdl.so.2", O_RDONLY|O_CLOEXEC) = -1 ENOENT (No such file or directory)
newfstatat(AT_FDCWD, "/root/.rustup/toolchains/stable-aarch64-unknown-linux-gnu/lib/tls", 0xffffc61c1c90, 0) = -1 ENOENT (No such file or directory)
openat(AT_FDCWD, "/root/.rustup/toolchains/stable-aarch64-unknown-linux-gnu/lib/aarch64/atomics/libdl.so.2", O_RDONLY|O_CLOEXEC) = -1 ENOENT (No such file or directory)
newfstatat(AT_FDCWD, "/root/.rustup/toolchains/stable-aarch64-unknown-linux-gnu/lib/aarch64/atomics", 0xffffc61c1c90, 0) = -1 ENOENT (No such file or directory)
openat(AT_FDCWD, "/root/.rustup/toolchains/stable-aarch64-unknown-linux-gnu/lib/aarch64/libdl.so.2", O_RDONLY|O_CLOEXEC) = -1 ENOENT (No such file or directory)
newfstatat(AT_FDCWD, "/root/.rustup/toolchains/stable-aarch64-unknown-linux-gnu/lib/aarch64", 0xffffc61c1c90, 0) = -1 ENOENT (No such file or directory)
openat(AT_FDCWD, "/root/.rustup/toolchains/stable-aarch64-unknown-linux-gnu/lib/atomics/libdl.so.2", O_RDONLY|O_CLOEXEC) = -1 ENOENT (No such file or directory)
newfstatat(AT_FDCWD, "/root/.rustup/toolchains/stable-aarch64-unknown-linux-gnu/lib/atomics", 0xffffc61c1c90, 0) = -1 ENOENT (No such file or directory)
openat(AT_FDCWD, "/root/.rustup/toolchains/stable-aarch64-unknown-linux-gnu/lib/libdl.so.2", O_RDONLY|O_CLOEXEC) = -1 ENOENT (No such file or directory)
newfstatat(AT_FDCWD, "/root/.rustup/toolchains/stable-aarch64-unknown-linux-gnu/lib", {st_mode=S_IFDIR|0755, st_size=4096, ...}, 0) = 0
openat(AT_FDCWD, "/etc/ld.so.cache", O_RDONLY|O_CLOEXEC) = 3
newfstatat(3, "", {st_mode=S_IFREG|0644, st_size=16266, ...}, AT_EMPTY_PATH) = 0
mmap(NULL, 16266, PROT_READ, MAP_PRIVATE, 3, 0) = 0xffffa90fa000
close(3)                                = 0
openat(AT_FDCWD, "/lib/aarch64-linux-gnu/libdl.so.2", O_RDONLY|O_CLOEXEC) = 3
read(3, "\177ELF\2\1\1\0\0\0\0\0\0\0\0\0\3\0\267\0\1\0\0\0\0\0\0\0\0\0\0\0"..., 832) = 832
newfstatat(3, "", {st_mode=S_IFREG|0644, st_size=67528, ...}, AT_EMPTY_PATH) = 0
mmap(NULL, 196640, PROT_NONE, MAP_PRIVATE|MAP_ANONYMOUS, -1, 0) = 0xffffa9094000
mmap(0xffffa90a0000, 131104, PROT_READ|PROT_EXEC, MAP_PRIVATE|MAP_FIXED|MAP_DENYWRITE, 3, 0) = 0xffffa90a0000
munmap(0xffffa9094000, 49152)           = 0
munmap(0xffffa90c1000, 12320)           = 0
mprotect(0xffffa90a1000, 122880, PROT_NONE) = 0
mmap(0xffffa90bf000, 8192, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_FIXED|MAP_DENYWRITE, 3, 0xf000) = 0xffffa90bf000
close(3)                                = 0
openat(AT_FDCWD, "/root/.rustup/toolchains/stable-aarch64-unknown-linux-gnu/bin/../lib/libgcc_s.so.1", O_RDONLY|O_CLOEXEC) = -1 ENOENT (No such file or directory)
openat(AT_FDCWD, "/root/.rustup/toolchains/stable-aarch64-unknown-linux-gnu/lib/libgcc_s.so.1", O_RDONLY|O_CLOEXEC) = -1 ENOENT (No such file or directory)
openat(AT_FDCWD, "/lib/aarch64-linux-gnu/libgcc_s.so.1", O_RDONLY|O_CLOEXEC) = 3
read(3, "\177ELF\2\1\1\0\0\0\0\0\0\0\0\0\3\0\267\0\1\0\0\0\0\0\0\0\0\0\0\0"..., 832) = 832
newfstatat(3, "", {st_mode=S_IFREG|0644, st_size=133448, ...}, AT_EMPTY_PATH) = 0
mmap(NULL, 262856, PROT_NONE, MAP_PRIVATE|MAP_ANONYMOUS, -1, 0) = 0xffffa905f000
mmap(0xffffa9060000, 197320, PROT_READ|PROT_EXEC, MAP_PRIVATE|MAP_FIXED|MAP_DENYWRITE, 3, 0) = 0xffffa9060000
munmap(0xffffa905f000, 4096)            = 0
munmap(0xffffa9091000, 58056)           = 0
mprotect(0xffffa9074000, 110592, PROT_NONE) = 0
mmap(0xffffa908f000, 8192, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_FIXED|MAP_DENYWRITE, 3, 0x1f000) = 0xffffa908f000
close(3)                                = 0
openat(AT_FDCWD, "/root/.rustup/toolchains/stable-aarch64-unknown-linux-gnu/bin/../lib/libpthread.so.0", O_RDONLY|O_CLOEXEC) = -1 ENOENT (No such file or directory)
openat(AT_FDCWD, "/root/.rustup/toolchains/stable-aarch64-unknown-linux-gnu/lib/libpthread.so.0", O_RDONLY|O_CLOEXEC) = -1 ENOENT (No such file or directory)
openat(AT_FDCWD, "/lib/aarch64-linux-gnu/libpthread.so.0", O_RDONLY|O_CLOEXEC) = 3
read(3, "\177ELF\2\1\1\0\0\0\0\0\0\0\0\0\3\0\267\0\1\0\0\0\0\0\0\0\0\0\0\0"..., 832) = 832
newfstatat(3, "", {st_mode=S_IFREG|0644, st_size=67528, ...}, AT_EMPTY_PATH) = 0
mmap(NULL, 196640, PROT_NONE, MAP_PRIVATE|MAP_ANONYMOUS, -1, 0) = 0xffffa902f000
mmap(0xffffa9030000, 131104, PROT_READ|PROT_EXEC, MAP_PRIVATE|MAP_FIXED|MAP_DENYWRITE, 3, 0) = 0xffffa9030000
munmap(0xffffa902f000, 4096)            = 0
munmap(0xffffa9051000, 57376)           = 0
mprotect(0xffffa9031000, 122880, PROT_NONE) = 0
mmap(0xffffa904f000, 8192, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_FIXED|MAP_DENYWRITE, 3, 0xf000) = 0xffffa904f000
close(3)                                = 0
openat(AT_FDCWD, "/root/.rustup/toolchains/stable-aarch64-unknown-linux-gnu/bin/../lib/libm.so.6", O_RDONLY|O_CLOEXEC) = -1 ENOENT (No such file or directory)
openat(AT_FDCWD, "/root/.rustup/toolchains/stable-aarch64-unknown-linux-gnu/lib/libm.so.6", O_RDONLY|O_CLOEXEC) = -1 ENOENT (No such file or directory)
openat(AT_FDCWD, "/lib/aarch64-linux-gnu/libm.so.6", O_RDONLY|O_CLOEXEC) = 3
read(3, "\177ELF\2\1\1\0\0\0\0\0\0\0\0\0\3\0\267\0\1\0\0\0\0\0\0\0\0\0\0\0"..., 832) = 832
newfstatat(3, "", {st_mode=S_IFREG|0644, st_size=591960, ...}, AT_EMPTY_PATH) = 0
mmap(NULL, 655472, PROT_NONE, MAP_PRIVATE|MAP_ANONYMOUS, -1, 0) = 0xffffa8f8f000
mmap(0xffffa8f90000, 589936, PROT_READ|PROT_EXEC, MAP_PRIVATE|MAP_FIXED|MAP_DENYWRITE, 3, 0) = 0xffffa8f90000
munmap(0xffffa8f8f000, 4096)            = 0
munmap(0xffffa9021000, 57456)           = 0
mprotect(0xffffa9010000, 61440, PROT_NONE) = 0
mmap(0xffffa901f000, 8192, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_FIXED|MAP_DENYWRITE, 3, 0x8f000) = 0xffffa901f000
close(3)                                = 0
openat(AT_FDCWD, "/root/.rustup/toolchains/stable-aarch64-unknown-linux-gnu/bin/../lib/libc.so.6", O_RDONLY|O_CLOEXEC) = -1 ENOENT (No such file or directory)
openat(AT_FDCWD, "/root/.rustup/toolchains/stable-aarch64-unknown-linux-gnu/lib/libc.so.6", O_RDONLY|O_CLOEXEC) = -1 ENOENT (No such file or directory)
openat(AT_FDCWD, "/lib/aarch64-linux-gnu/libc.so.6", O_RDONLY|O_CLOEXEC) = 3
read(3, "\177ELF\2\1\1\3\0\0\0\0\0\0\0\0\3\0\267\0\1\0\0\0000y\2\0\0\0\0\0"..., 832) = 832
newfstatat(3, "", {st_mode=S_IFREG|0755, st_size=1651408, ...}, AT_EMPTY_PATH) = 0
mmap(NULL, 1826912, PROT_NONE, MAP_PRIVATE|MAP_ANONYMOUS, -1, 0) = 0xffffa8dd1000
mmap(0xffffa8de0000, 1761376, PROT_READ|PROT_EXEC, MAP_PRIVATE|MAP_FIXED|MAP_DENYWRITE, 3, 0) = 0xffffa8de0000
munmap(0xffffa8dd1000, 61440)           = 0
munmap(0xffffa8f8f000, 96)              = 0
mprotect(0xffffa8f67000, 86016, PROT_NONE) = 0
mmap(0xffffa8f7c000, 24576, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_FIXED|MAP_DENYWRITE, 3, 0x18c000) = 0xffffa8f7c000
mmap(0xffffa8f82000, 49248, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_FIXED|MAP_ANONYMOUS, -1, 0) = 0xffffa8f82000
close(3)                                = 0
mmap(NULL, 8192, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_ANONYMOUS, -1, 0) = 0xffffa90f8000
set_tid_address(0xffffa90f8e30)         = 749
set_robust_list(0xffffa90f8e40, 24)     = 0
rseq(0xffffa90f9480, 0x20, 0, 0xd428bc00) = 0
mprotect(0xffffa8f7c000, 16384, PROT_READ) = 0
mprotect(0xffffa901f000, 4096, PROT_READ) = 0
mprotect(0xffffa904f000, 4096, PROT_READ) = 0
mprotect(0xffffa908f000, 4096, PROT_READ) = 0
mprotect(0xffffa90bf000, 4096, PROT_READ) = 0
mprotect(0xaaaab598b000, 1224704, PROT_READ) = 0
mmap(NULL, 8192, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_ANONYMOUS, -1, 0) = 0xffffa90f6000
mprotect(0xffffa9103000, 8192, PROT_READ) = 0
prlimit64(0, RLIMIT_STACK, NULL, {rlim_cur=8192*1024, rlim_max=RLIM64_INFINITY}) = 0
munmap(0xffffa90fa000, 16266)           = 0
getrandom("\x39\xe4\x21\xa7\x2c\xff\x51\xc8", 8, GRND_NONBLOCK) = 8
brk(NULL)                               = 0xaaaaeab0f000
brk(0xaaaaeab30000)                     = 0xaaaaeab30000
rt_sigprocmask(SIG_SETMASK, ~[ILL TRAP BUS FPE SEGV RTMIN RT_1], [], 8) = 0
rt_sigaction(SIGILL, {sa_handler=0xaaaab5097b40, sa_mask=~[ILL TRAP BUS FPE SEGV RTMIN RT_1], sa_flags=0}, {sa_handler=SIG_DFL, sa_mask=[], sa_flags=0}, 8) = 0
rt_sigprocmask(SIG_BLOCK, NULL, ~[ILL TRAP BUS FPE KILL SEGV STOP RTMIN RT_1], 8) = 0
rt_sigaction(SIGILL, {sa_handler=SIG_DFL, sa_mask=[], sa_flags=0}, NULL, 8) = 0
rt_sigprocmask(SIG_SETMASK, [], NULL, 8) = 0
futex(0xaaaab5ac8724, FUTEX_WAKE_PRIVATE, 2147483647) = 0
futex(0xaaaab5ac8718, FUTEX_WAKE_PRIVATE, 2147483647) = 0
futex(0xaaaab5ac8710, FUTEX_WAKE_PRIVATE, 2147483647) = 0
futex(0xaaaab5ac87b8, FUTEX_WAKE_PRIVATE, 2147483647) = 0
futex(0xaaaab5ac8700, FUTEX_WAKE_PRIVATE, 2147483647) = 0
futex(0xaaaab5ac86f8, FUTEX_WAKE_PRIVATE, 2147483647) = 0
futex(0xaaaab5ac86cc, FUTEX_WAKE_PRIVATE, 2147483647) = 0
futex(0xaaaab5ac5bdc, FUTEX_WAKE_PRIVATE, 2147483647) = 0
futex(0xaaaab5ac85dc, FUTEX_WAKE_PRIVATE, 2147483647) = 0
futex(0xaaaab5ac8574, FUTEX_WAKE_PRIVATE, 2147483647) = 0
futex(0xaaaab5ac8568, FUTEX_WAKE_PRIVATE, 2147483647) = 0
futex(0xaaaab5ac8708, FUTEX_WAKE_PRIVATE, 2147483647) = 0
futex(0xaaaab5ac86c4, FUTEX_WAKE_PRIVATE, 2147483647) = 0
futex(0xaaaab5ac86bc, FUTEX_WAKE_PRIVATE, 2147483647) = 0
openat(AT_FDCWD, "/usr/local/ssl/openssl.cnf", O_RDONLY) = -1 ENOENT (No such file or directory)
futex(0xaaaab5ac86f0, FUTEX_WAKE_PRIVATE, 2147483647) = 0
sysinfo({uptime=83955, loads=[16160, 13984, 10112], totalram=8217968640, freeram=4349095936, sharedram=3502080, bufferram=159621120, totalswap=1073737728, freeswap=1073737728, procs=324, totalhigh=0, freehigh=0, mem_unit=1}) = 0
futex(0xaaaab5ac5b70, FUTEX_WAKE_PRIVATE, 2147483647) = 0
ppoll([{fd=0, events=0}, {fd=1, events=0}, {fd=2, events=0}], 3, {tv_sec=0, tv_nsec=0}, NULL, 0) = 1 ([{fd=0, revents=POLLHUP}], left {tv_sec=0, tv_nsec=0})
rt_sigaction(SIGPIPE, {sa_handler=SIG_IGN, sa_mask=[PIPE], sa_flags=SA_RESTART}, {sa_handler=SIG_DFL, sa_mask=[], sa_flags=0}, 8) = 0
openat(AT_FDCWD, "/proc/self/maps", O_RDONLY|O_CLOEXEC) = 3
prlimit64(0, RLIMIT_STACK, NULL, {rlim_cur=8192*1024, rlim_max=RLIM64_INFINITY}) = 0
newfstatat(3, "", {st_mode=S_IFREG|0444, st_size=0, ...}, AT_EMPTY_PATH) = 0
read(3, "aaaab41e0000-aaaab597b000 r-xp 0"..., 1024) = 1024
read(3, "0000000 00:00 0 \nffffa8f90000-ff"..., 1024) = 1024
read(3, "gcc_s.so.1\nffffa9074000-ffffa908"..., 1024) = 1024
read(3, "00000-ffffa9102000 r--p 00000000"..., 1024) = 478
close(3)                                = 0
sched_getaffinity(749, 32, [0 1 2 3 4 5 6 7 8 9]) = 8
rt_sigaction(SIGSEGV, NULL, {sa_handler=SIG_DFL, sa_mask=[], sa_flags=0}, 8) = 0
sigaltstack(NULL, {ss_sp=NULL, ss_flags=SS_DISABLE, ss_size=0}) = 0
mmap(NULL, 20480, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_ANONYMOUS|MAP_STACK, -1, 0) = 0xffffa90f1000
mprotect(0xffffa90f1000, 4096, PROT_NONE) = 0
sigaltstack({ss_sp=0xffffa90f2000, ss_flags=0, ss_size=16384}, NULL) = 0
rt_sigaction(SIGSEGV, {sa_handler=0xaaaab5439f0c, sa_mask=[], sa_flags=SA_ONSTACK|SA_SIGINFO}, NULL, 8) = 0
rt_sigaction(SIGBUS, NULL, {sa_handler=SIG_DFL, sa_mask=[], sa_flags=0}, 8) = 0
rt_sigaction(SIGBUS, {sa_handler=0xaaaab5439f0c, sa_mask=[], sa_flags=SA_ONSTACK|SA_SIGINFO}, NULL, 8) = 0
getrandom("\x22\xc2\xed\x83\xd3\x1c\xbe\xad\xa2\x3f\x7c\x0e\xd8\xeb\x8c\x6d", 16, GRND_INSECURE) = 16
ioctl(2, TCGETS, 0xffffc61c0300)        = -1 ENOTTY (Inappropriate ioctl for device)
brk(0xaaaaeab53000)                     = 0xaaaaeab53000
ioctl(1, TCGETS, {c_iflag=ICRNL|IXON, c_oflag=NL0|CR0|TAB0|BS0|VT0|FF0|OPOST|ONLCR, c_cflag=B38400|CS8|CREAD, c_lflag=ISIG|ICANON|ECHO|ECHOE|ECHOK|IEXTEN|ECHOCTL|ECHOKE, ...}) = 0
ioctl(1, TCGETS, {c_iflag=ICRNL|IXON, c_oflag=NL0|CR0|TAB0|BS0|VT0|FF0|OPOST|ONLCR, c_cflag=B38400|CS8|CREAD, c_lflag=ISIG|ICANON|ECHO|ECHOE|ECHOK|IEXTEN|ECHOCTL|ECHOKE, ...}) = 0
ioctl(2, TCGETS, 0xffffc61c00e0)        = -1 ENOTTY (Inappropriate ioctl for device)
ioctl(2, TCGETS, 0xffffc61c0160)        = -1 ENOTTY (Inappropriate ioctl for device)
ioctl(1, TCGETS, {c_iflag=ICRNL|IXON, c_oflag=NL0|CR0|TAB0|BS0|VT0|FF0|OPOST|ONLCR, c_cflag=B38400|CS8|CREAD, c_lflag=ISIG|ICANON|ECHO|ECHOE|ECHOK|IEXTEN|ECHOCTL|ECHOKE, ...}) = 0
ioctl(2, TCGETS, 0xffffc61c0160)        = -1 ENOTTY (Inappropriate ioctl for device)
getcwd("/hdp", 512)                     = 5
openat(AT_FDCWD, "/hdp/.cargo/config", O_RDONLY|O_CLOEXEC) = -1 ENOENT (No such file or directory)
statx(AT_FDCWD, "/hdp/.cargo/config.toml", AT_STATX_SYNC_AS_STAT, STATX_ALL, 0xffffc61bde10) = -1 ENOENT (No such file or directory)
statx(0, NULL, AT_STATX_SYNC_AS_STAT, STATX_ALL, NULL) = -1 EFAULT (Bad address)
openat(AT_FDCWD, "/.cargo/config", O_RDONLY|O_CLOEXEC) = -1 ENOENT (No such file or directory)
statx(AT_FDCWD, "/.cargo/config.toml", AT_STATX_SYNC_AS_STAT, STATX_ALL, 0xffffc61bde10) = -1 ENOENT (No such file or directory)
openat(AT_FDCWD, "/root/.cargo/config", O_RDONLY|O_CLOEXEC) = -1 ENOENT (No such file or directory)
statx(AT_FDCWD, "/root/.cargo/config.toml", AT_STATX_SYNC_AS_STAT, STATX_ALL, 0xffffc61bde10) = -1 ENOENT (No such file or directory)
brk(0xaaaaeab75000)                     = 0xaaaaeab75000
brk(0xaaaaeab74000)                     = 0xaaaaeab74000
brk(0xaaaaeab96000)                     = 0xaaaaeab96000
brk(0xaaaaeabb7000)                     = 0xaaaaeabb7000
brk(0xaaaaeabdb000)                     = 0xaaaaeabdb000
brk(0xaaaaeac00000)                     = 0xaaaaeac00000
brk(0xaaaaeabfd000)                     = 0xaaaaeabfd000
brk(0xaaaaeac1f000)                     = 0xaaaaeac1f000
brk(0xaaaaeac42000)                     = 0xaaaaeac42000
brk(0xaaaaeac63000)                     = 0xaaaaeac63000
brk(0xaaaaeac5a000)                     = 0xaaaaeac5a000
brk(0xaaaaeac03000)                     = 0xaaaaeac03000
brk(0xaaaaeabf7000)                     = 0xaaaaeabf7000
ioctl(1, TCGETS, {c_iflag=ICRNL|IXON, c_oflag=NL0|CR0|TAB0|BS0|VT0|FF0|OPOST|ONLCR, c_cflag=B38400|CS8|CREAD, c_lflag=ISIG|ICANON|ECHO|ECHOE|ECHOK|IEXTEN|ECHOCTL|ECHOKE, ...}) = 0
ioctl(1, TCGETS, {c_iflag=ICRNL|IXON, c_oflag=NL0|CR0|TAB0|BS0|VT0|FF0|OPOST|ONLCR, c_cflag=B38400|CS8|CREAD, c_lflag=ISIG|ICANON|ECHO|ECHOE|ECHOK|IEXTEN|ECHOCTL|ECHOKE, ...}) = 0
ioctl(2, TCGETS, 0xffffc61bf5a0)        = -1 ENOTTY (Inappropriate ioctl for device)
statx(AT_FDCWD, "/var/ssl", AT_STATX_SYNC_AS_STAT, STATX_ALL, 0xffffc61bf390) = -1 ENOENT (No such file or directory)
statx(AT_FDCWD, "/usr/share/ssl", AT_STATX_SYNC_AS_STAT, STATX_ALL, 0xffffc61bf390) = -1 ENOENT (No such file or directory)
statx(AT_FDCWD, "/usr/local/ssl", AT_STATX_SYNC_AS_STAT, STATX_ALL, 0xffffc61bf390) = -1 ENOENT (No such file or directory)
statx(AT_FDCWD, "/usr/local/openssl", AT_STATX_SYNC_AS_STAT, STATX_ALL, 0xffffc61bf390) = -1 ENOENT (No such file or directory)
statx(AT_FDCWD, "/usr/local/etc/openssl", AT_STATX_SYNC_AS_STAT, STATX_ALL, 0xffffc61bf390) = -1 ENOENT (No such file or directory)
statx(AT_FDCWD, "/usr/local/share", AT_STATX_SYNC_AS_STAT, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFDIR|0755, stx_size=4096, ...}) = 0
statx(AT_FDCWD, "/usr/local/share/cert.pem", AT_STATX_SYNC_AS_STAT, STATX_ALL, 0xffffc61bf390) = -1 ENOENT (No such file or directory)
statx(AT_FDCWD, "/usr/local/share/certs.pem", AT_STATX_SYNC_AS_STAT, STATX_ALL, 0xffffc61bf390) = -1 ENOENT (No such file or directory)
statx(AT_FDCWD, "/usr/local/share/ca-bundle.pem", AT_STATX_SYNC_AS_STAT, STATX_ALL, 0xffffc61bf390) = -1 ENOENT (No such file or directory)
statx(AT_FDCWD, "/usr/local/share/cacert.pem", AT_STATX_SYNC_AS_STAT, STATX_ALL, 0xffffc61bf390) = -1 ENOENT (No such file or directory)
statx(AT_FDCWD, "/usr/local/share/ca-certificates.crt", AT_STATX_SYNC_AS_STAT, STATX_ALL, 0xffffc61bf390) = -1 ENOENT (No such file or directory)
statx(AT_FDCWD, "/usr/local/share/certs/ca-certificates.crt", AT_STATX_SYNC_AS_STAT, STATX_ALL, 0xffffc61bf390) = -1 ENOENT (No such file or directory)
statx(AT_FDCWD, "/usr/local/share/certs/ca-root-nss.crt", AT_STATX_SYNC_AS_STAT, STATX_ALL, 0xffffc61bf390) = -1 ENOENT (No such file or directory)
statx(AT_FDCWD, "/usr/local/share/certs/ca-bundle.crt", AT_STATX_SYNC_AS_STAT, STATX_ALL, 0xffffc61bf390) = -1 ENOENT (No such file or directory)
statx(AT_FDCWD, "/usr/local/share/CARootCertificates.pem", AT_STATX_SYNC_AS_STAT, STATX_ALL, 0xffffc61bf390) = -1 ENOENT (No such file or directory)
statx(AT_FDCWD, "/usr/local/share/tls-ca-bundle.pem", AT_STATX_SYNC_AS_STAT, STATX_ALL, 0xffffc61bf390) = -1 ENOENT (No such file or directory)
statx(AT_FDCWD, "/usr/local/share/certs", AT_STATX_SYNC_AS_STAT, STATX_ALL, 0xffffc61bf390) = -1 ENOENT (No such file or directory)
statx(AT_FDCWD, "/usr/lib/ssl", AT_STATX_SYNC_AS_STAT, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFDIR|0755, stx_size=4096, ...}) = 0
statx(AT_FDCWD, "/usr/lib/ssl/cert.pem", AT_STATX_SYNC_AS_STAT, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0644, stx_size=213777, ...}) = 0
statx(AT_FDCWD, "/usr/lib/ssl/certs", AT_STATX_SYNC_AS_STAT, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFDIR|0755, stx_size=16384, ...}) = 0
futex(0xaaaab5ac5b68, FUTEX_WAKE_PRIVATE, 2147483647) = 0
openat(AT_FDCWD, "/dev/urandom", O_RDONLY) = 3
read(3, "\2013\1\303\2719\256\371", 8)  = 8
close(3)                                = 0
getuid()                                = 0
geteuid()                               = 0
getuid()                                = 0
geteuid()                               = 0
getuid()                                = 0
geteuid()                               = 0
futex(0xaaaab5ac59d8, FUTEX_WAKE_PRIVATE, 2147483647) = 0
futex(0xaaaab5ac8828, FUTEX_WAKE_PRIVATE, 2147483647) = 0
getpid()                                = 749
getrandom("\xdb\xc7\xa4\xde\x3d\x9e\x9c\xdf\xce\x1c\x34\xc7\x7f\x8d\x8b\x19\x5d\x1f\x71\x8b\x88\x46\x8e\x85\x70\x1f\xc5\xc3\xb3\x61\x14\x18", 32, 0) = 32
getpid()                                = 749
futex(0xaaaab5ac87f8, FUTEX_WAKE_PRIVATE, 2147483647) = 0
getpid()                                = 749
getpid()                                = 749
getpid()                                = 749
getpid()                                = 749
getpid()                                = 749
getpid()                                = 749
getpid()                                = 749
getpid()                                = 749
getpid()                                = 749
getpid()                                = 749
getpid()                                = 749
getpid()                                = 749
openat(AT_FDCWD, "/usr/lib/ssl/cert.pem", O_RDONLY) = 3
newfstatat(3, "", {st_mode=S_IFREG|0644, st_size=213777, ...}, AT_EMPTY_PATH) = 0
read(3, "-----BEGIN CERTIFICATE-----\nMIIH"..., 4096) = 4096
read(3, "8B1\nRXxlDPiyN8+sD8+Nb/kZ94/sHvJw"..., 4096) = 4096
read(3, "BgkqhkiG9w0BAQEFAAOCAg8AMIICCgKC"..., 4096) = 4096
read(3, "oZIhvcNAQEMBQAwQTELMAkGA1UE\nBhMC"..., 4096) = 4096
read(3, "DCCAgoCggIBAK2Wny2cSkxK\ngXlRmeyK"..., 4096) = 4096
read(3, "aAjMaZ7snkGeRDImeuKHCnE96+RapNLb"..., 4096) = 4096
read(3, "FFQ4ueCyE8S1wF3BqfmI7avSKecs2t\nC"..., 4096) = 4096
read(3, "mdvhFHJlsTmKtdFoqwNxxXnUX/iJY2v7"..., 4096) = 4096
read(3, "t/SyZi4QKPaXWnuWFo8BGS1sbn85WAZk"..., 4096) = 4096
read(3, "dHkwggIiMA0GCSqGSIb3DQEBAQUAA4IC"..., 4096) = 4096
read(3, "2IpHLlOR+Vnb5n\nwXARPbv0+Em34yaXO"..., 4096) = 4096
read(3, "0gQ2VydGlmaWNhdGlvbiBBdXRob3JpdH"..., 4096) = 4096
read(3, "TLkEu\nMScwJQYDVQQLEx5DZXJ0dW0gQ2"..., 4096) = 4096
read(3, "Y2FfMV8yMDIwLmNybDB5oHegdYZzbGRh"..., 4096) = 4096
read(3, "A4IBAQA07XtaPKSUiO8aEXUHL7P+PPoe"..., 4096) = 4096
read(3, "iYWwgUm9vdCBD\nQTAeFw0wNjExMTAwMD"..., 4096) = 4096
read(3, "EwDgYDVR0PAQH/BAQDAgGGMA8GA1UdEw"..., 4096) = 4096
read(3, "9k98FpiHaYdj1ZXUJ2h4mXaXpI8OCi\nE"..., 4096) = 4096
read(3, "xGZsrTie0bBRiKWQzPUwHQYDVR0OBBYE"..., 4096) = 4096
read(3, "ZmVyZW5jZTEfMB0GA1UECxMW\nKGMpIDI"..., 4096) = 4096
read(3, "MIIGSzCCBDOgAwIBAgIRANm1Q3+vqTkP"..., 4096) = 4096
read(3, "aaApJUqlyyvdimYHFngVV3Eb7PVHhPOe"..., 4096) = 4096
read(3, "A1UEChMZR29vZ2xlIFRydXN0IFNlcnZp"..., 4096) = 4096
read(3, "cflK2GwwCgYIKoZIzj0EAwMwUDEk\nMCI"..., 4096) = 4096
read(3, "yCLNhZglqsQY6ZZZZwPA1/cnaKI0aEYd"..., 4096) = 4096
read(3, "LjExMC8GA1UECxMoR28gRGFkZHkgQ2xh"..., 4096) = 4096
read(3, "/JiZ+yykgmvw\nKh+OC19xXFyuQnspiYH"..., 4096) = 4096
read(3, "aSrT2y7HxjbdavYy5LNlDhhDgcGH0tGE"..., 4096) = 4096
read(3, "4i707mV78vH9toxdCim5lSJ9UExyuUmG"..., 4096) = 4096
read(3, "gCyzuFJ0nN6T5U6VR5CmD1/iQMVtCnwr"..., 4096) = 4096
read(3, "wZS5jb20wHhcNMDcxMjEzMTMwODI4Whc"..., 4096) = 4096
read(3, "I3\nFQEEAwIBADAKBggqhkjOPQQDAwNoA"..., 4096) = 4096
read(3, "tP+oGI/hGoiLtk/bdmuYqh7GYVPEi92t"..., 4096) = 4096
read(3, "gxCzAJBgNVBAYTAkJNMRkwFwYDVQQKEx"..., 4096) = 4096
read(3, "ggIKAoICAQChriWyARjcV4g/Ruv5r+Lr"..., 4096) = 4096
read(3, "BHMzAeFw0xMjAxMTIyMDI2MzJaFw00\nM"..., 4096) = 4096
read(3, "ZhcFUZh1++VQLHqe8RT6q9OKPv+RKY9j"..., 4096) = 4096
read(3, "Uk9PVCBDQTIwggEiMA0GCSqGSIb3DQEB"..., 4096) = 4096
read(3, "BCWeZ4WNOaptvolRTnI\nHmX5k/Wq8VLc"..., 4096) = 4096
read(3, "DcAiMI4u8hOscNtybS\nYpOnpSNyByCCY"..., 4096) = 4096
read(3, "Z/5FSuS/hVclcCGfgXcVnrHigHdMWdSL"..., 4096) = 4096
read(3, "6LqjviOvrv1vA+ACOzB2+htt\nQc8Bsem"..., 4096) = 4096
read(3, "SivwKixVA9ZIw+A5OO3yXDw/\nRLyTPWG"..., 4096) = 4096
read(3, "PPyBJUgriOCxLM6AGK/5jYk4Ve6xx6Qd"..., 4096) = 4096
read(3, "EPlcDaMtjNXepUugqD0XBCzYYP2AgWGL"..., 4096) = 4096
read(3, "/HHk484IkzlQsPpTLWPFp5LBk=\n-----"..., 4096) = 4096
read(3, "ekDFQdxh\nVicGaeVyQYHTtgGJoC86cnn"..., 4096) = 4096
read(3, "TkD5OGwDxFa2DK5o=\n-----END CERTI"..., 4096) = 4096
read(3, "ydGlmaWNhdGlvbiBBdXRob3JpdHkwHhc"..., 4096) = 4096
brk(0xaaaaeac18000)                     = 0xaaaaeac18000
read(3, "c5vMZnT5r7SHpDwCRR5XCOrTdLa\nIR9N"..., 4096) = 4096
read(3, "\n-----END CERTIFICATE-----\n-----"..., 4096) = 4096
read(3, "MxMIIBIjANBgkqhkiG9w0BAQEFAAOCAQ"..., 4096) = 4096
read(3, "MBAf8E\nBTADAQH/MA4GA1UdDwEB/wQEA"..., 4096) = 785
read(3, "", 4096)                       = 0
close(3)                                = 0
futex(0xaaaab5ac5b8c, FUTEX_WAKE_PRIVATE, 2147483647) = 0
faccessat(AT_FDCWD, "/root/.gitconfig", F_OK) = -1 ENOENT (No such file or directory)
newfstatat(AT_FDCWD, "/root/.gitconfig", 0xffffc61bf4a0, 0) = -1 ENOENT (No such file or directory)
faccessat(AT_FDCWD, "/root/.gitconfig", F_OK) = -1 ENOENT (No such file or directory)
faccessat(AT_FDCWD, "/root/.config/git/config", F_OK) = -1 ENOENT (No such file or directory)
faccessat(AT_FDCWD, "/etc/gitconfig", F_OK) = -1 ENOENT (No such file or directory)
newfstatat(AT_FDCWD, "/root/.gitconfig", 0xffffc61bf2d0, 0) = -1 ENOENT (No such file or directory)
newfstatat(AT_FDCWD, "/root/.gitconfig", 0xffffc61bf2d0, 0) = -1 ENOENT (No such file or directory)
faccessat(AT_FDCWD, "/root/.gitconfig", F_OK) = -1 ENOENT (No such file or directory)
newfstatat(AT_FDCWD, "/root/.gitconfig", 0xffffc61bf7c0, 0) = -1 ENOENT (No such file or directory)
faccessat(AT_FDCWD, "/root/.gitconfig", F_OK) = -1 ENOENT (No such file or directory)
faccessat(AT_FDCWD, "/root/.config/git/config", F_OK) = -1 ENOENT (No such file or directory)
faccessat(AT_FDCWD, "/etc/gitconfig", F_OK) = -1 ENOENT (No such file or directory)
newfstatat(AT_FDCWD, "/root/.gitconfig", 0xffffc61bf5f0, 0) = -1 ENOENT (No such file or directory)
newfstatat(AT_FDCWD, "/root/.gitconfig", 0xffffc61bf5f0, 0) = -1 ENOENT (No such file or directory)
statx(AT_FDCWD, "/hdp/Cargo.toml", AT_STATX_SYNC_AS_STAT, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0644, stx_size=391, ...}) = 0
openat(AT_FDCWD, "/hdp/Cargo.toml", O_RDONLY|O_CLOEXEC) = 3
statx(3, "", AT_STATX_SYNC_AS_STAT|AT_EMPTY_PATH, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0644, stx_size=391, ...}) = 0
read(3, "[package]\nname = \"hdp\"\nversion ="..., 391) = 391
read(3, "", 32)                         = 0
close(3)                                = 0
statx(AT_FDCWD, "/hdp/build.rs", AT_STATX_SYNC_AS_STAT, STATX_ALL, 0xffffc61b7aa0) = -1 ENOENT (No such file or directory)
statx(AT_FDCWD, "/hdp/README.md", AT_STATX_SYNC_AS_STAT, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0644, stx_size=189, ...}) = 0
statx(AT_FDCWD, "/hdp/src/lib.rs", AT_STATX_SYNC_AS_STAT, STATX_ALL, 0xffffc61b8450) = -1 ENOENT (No such file or directory)
statx(AT_FDCWD, "/hdp/src/main.rs", AT_STATX_SYNC_AS_STAT, STATX_ALL, 0xffffc61b8450) = -1 ENOENT (No such file or directory)
openat(AT_FDCWD, "/hdp/src/bin", O_RDONLY|O_NONBLOCK|O_CLOEXEC|O_DIRECTORY) = -1 ENOENT (No such file or directory)
openat(AT_FDCWD, "/hdp/examples", O_RDONLY|O_NONBLOCK|O_CLOEXEC|O_DIRECTORY) = -1 ENOENT (No such file or directory)
openat(AT_FDCWD, "/hdp/tests", O_RDONLY|O_NONBLOCK|O_CLOEXEC|O_DIRECTORY) = -1 ENOENT (No such file or directory)
openat(AT_FDCWD, "/hdp/benches", O_RDONLY|O_NONBLOCK|O_CLOEXEC|O_DIRECTORY) = -1 ENOENT (No such file or directory)
statx(AT_FDCWD, "/hdp", AT_STATX_SYNC_AS_STAT, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFDIR|0755, stx_size=4096, ...}) = 0
statx(AT_FDCWD, "/Cargo.toml", AT_STATX_SYNC_AS_STAT, STATX_ALL, 0xffffc61bd9e0) = -1 ENOENT (No such file or directory)
openat(AT_FDCWD, "/proc/self/cgroup", O_RDONLY|O_CLOEXEC) = 3
statx(3, "", AT_STATX_SYNC_AS_STAT|AT_EMPTY_PATH, STATX_ALL, {stx_mask=STATX_BASIC_STATS|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0444, stx_size=0, ...}) = 0
lseek(3, 0, SEEK_CUR)                   = 0
read(3, "0::/\n", 128)                  = 5
read(3, "", 123)                        = 0
close(3)                                = 0
statx(AT_FDCWD, "/sys/fs/cgroup/cgroup.controllers", AT_STATX_SYNC_AS_STAT, STATX_ALL, {stx_mask=STATX_BASIC_STATS|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0444, stx_size=0, ...}) = 0
openat(AT_FDCWD, "/sys/fs/cgroup/cpu.max", O_RDONLY|O_CLOEXEC) = 3
statx(3, "", AT_STATX_SYNC_AS_STAT|AT_EMPTY_PATH, STATX_ALL, {stx_mask=STATX_BASIC_STATS|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0644, stx_size=0, ...}) = 0
lseek(3, 0, SEEK_CUR)                   = 0
read(3, "max 100000\n", 32)             = 11
read(3, "", 9)                          = 0
close(3)                                = 0
sched_getaffinity(0, 128, [0 1 2 3 4 5 6 7 8 9]) = 8
statx(AT_FDCWD, "/root/.cargo/bin/rustc", AT_STATX_SYNC_AS_STAT, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0755, stx_size=15187976, ...}) = 0
statx(AT_FDCWD, "/root/.cargo/bin/rustup", AT_STATX_SYNC_AS_STAT, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0755, stx_size=15187976, ...}) = 0
statx(AT_FDCWD, "/root/.cargo/bin/rustc", AT_STATX_SYNC_AS_STAT, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0755, stx_size=15187976, ...}) = 0
statx(AT_FDCWD, "/root/.cargo/bin/rustup", AT_STATX_SYNC_AS_STAT, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0755, stx_size=15187976, ...}) = 0
getcwd("/hdp", 512)                     = 5
statx(AT_FDCWD, "/root/.rustup/toolchains/stable-aarch64-unknown-linux-gnu/bin/rustc", AT_STATX_SYNC_AS_STAT, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0755, stx_size=13632, ...}) = 0
statx(AT_FDCWD, "/root/.rustup/toolchains/stable-aarch64-unknown-linux-gnu/bin/rustc", AT_STATX_SYNC_AS_STAT, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0755, stx_size=13632, ...}) = 0
statx(AT_FDCWD, "/root/.rustup/toolchains/stable-aarch64-unknown-linux-gnu/bin/rustc", AT_STATX_SYNC_AS_STAT, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0755, stx_size=13632, ...}) = 0
openat(AT_FDCWD, "/hdp/target/.rustc_info.json", O_RDONLY|O_CLOEXEC) = 3
statx(3, "", AT_STATX_SYNC_AS_STAT|AT_EMPTY_PATH, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0644, stx_size=1015, ...}) = 0
read(3, "{\"rustc_fingerprint\":15931981652"..., 1015) = 1015
read(3, "", 32)                         = 0
close(3)                                = 0
statx(AT_FDCWD, "/hdp/Cargo.lock", AT_STATX_SYNC_AS_STAT, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0644, stx_size=9113, ...}) = 0
openat(AT_FDCWD, "/hdp/Cargo.lock", O_RDONLY|O_CLOEXEC) = 3
statfs("/hdp/Cargo.lock", {f_type=OVERLAYFS_SUPER_MAGIC, f_bsize=4096, f_blocks=263940461, f_bfree=261336868, f_bavail=247911012, f_files=67108864, f_ffree=66876332, f_fsid={val=[0x25291b5, 0x7d4d1bca]}, f_namelen=255, f_frsize=4096, f_flags=ST_VALID|ST_RELATIME}) = 0
flock(3, LOCK_SH|LOCK_NB)               = 0
read(3, "# This file is automatically @ge", 32) = 32
read(3, "nerated by Cargo.\n# It is not in", 32) = 32
read(3, "tended for manual editing.\nversi"..., 64) = 64
read(3, "\"\nversion = \"0.2.14\"\nsource = \"r"..., 128) = 128
read(3, "dd3f37e93092cbf377614828a319d5fe"..., 256) = 256
read(3, "1fb82e453128456482613d36\"\n\n[[pac"..., 512) = 512
read(3, "ub.com/rust-lang/crates.io-index"..., 1024) = 1024
read(3, "dba1cbe1c6ab0b15e29fff3f6c077fd9"..., 2048) = 2048
read(3, "version = \"0.1.0\"\nsource = \"regi"..., 4096) = 4096
brk(0xaaaaeac3c000)                     = 0xaaaaeac3c000
brk(0xaaaaeac3a000)                     = 0xaaaaeac3a000
read(3, "rust-lang/crates.io-index\"\ncheck"..., 8192) = 921
read(3, "", 7271)                       = 0
brk(0xaaaaeac5c000)                     = 0xaaaaeac5c000
brk(0xaaaaeac7d000)                     = 0xaaaaeac7d000
flock(3, LOCK_UN)                       = 0
close(3)                                = 0
openat(AT_FDCWD, "/root/.cargo/.package-cache", O_RDWR|O_CREAT|O_CLOEXEC, 0666) = 3
statfs("/root/.cargo/.package-cache", {f_type=OVERLAYFS_SUPER_MAGIC, f_bsize=4096, f_blocks=263940461, f_bfree=261336868, f_bavail=247911012, f_files=67108864, f_ffree=66876332, f_fsid={val=[0x25291b5, 0x7d4d1bca]}, f_namelen=255, f_frsize=4096, f_flags=ST_VALID|ST_RELATIME}) = 0
flock(3, LOCK_EX|LOCK_NB)               = 0
socketpair(AF_UNIX, SOCK_STREAM|SOCK_CLOEXEC|SOCK_NONBLOCK, 0, [4, 5]) = 0
mkdirat(AT_FDCWD, "/root/.cargo/registry", 0777) = -1 EEXIST (File exists)
statx(AT_FDCWD, "/root/.cargo/registry", AT_STATX_SYNC_AS_STAT, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFDIR|0755, stx_size=4096, ...}) = 0
statx(AT_FDCWD, "/root/.cargo/registry/CACHEDIR.TAG", AT_STATX_SYNC_AS_STAT, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0644, stx_size=177, ...}) = 0
openat(AT_FDCWD, "/root/.cargo/registry/index/index.crates.io-6f17d22bba15001f/.cache/at/ty/atty", O_RDONLY|O_CLOEXEC) = 6
statx(6, "", AT_STATX_SYNC_AS_STAT|AT_EMPTY_PATH, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0644, stx_size=9086, ...}) = 0
read(6, "\3\2\0\0\0etag: W/\"e213af317961e3049d"..., 9086) = 9086
read(6, "", 32)                         = 0
close(6)                                = 0
openat(AT_FDCWD, "/root/.cargo/registry/index/index.crates.io-6f17d22bba15001f/.cache/so/ck/socket2", O_RDONLY|O_CLOEXEC) = 6
statx(6, "", AT_STATX_SYNC_AS_STAT|AT_EMPTY_PATH, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0644, stx_size=34574, ...}) = 0
read(6, "\3\2\0\0\0etag: W/\"95542c3778e02f72c0"..., 34574) = 34574
read(6, "", 32)                         = 0
close(6)                                = 0
openat(AT_FDCWD, "/root/.cargo/registry/index/index.crates.io-6f17d22bba15001f/.cache/sy/s-/sys-info", O_RDONLY|O_CLOEXEC) = 6
statx(6, "", AT_STATX_SYNC_AS_STAT|AT_EMPTY_PATH, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0644, stx_size=10842, ...}) = 0
read(6, "\3\2\0\0\0etag: W/\"3cb7b3907652b9d6bc"..., 10842) = 10842
read(6, "", 32)                         = 0
close(6)                                = 0
openat(AT_FDCWD, "/root/.cargo/registry/index/index.crates.io-6f17d22bba15001f/.cache/wh/oa/whoami", O_RDONLY|O_CLOEXEC) = 6
statx(6, "", AT_STATX_SYNC_AS_STAT|AT_EMPTY_PATH, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0644, stx_size=20037, ...}) = 0
read(6, "\3\2\0\0\0etag: W/\"71b2b639af46bda8ac"..., 20037) = 20037
read(6, "", 32)                         = 0
close(6)                                = 0
openat(AT_FDCWD, "/root/.cargo/registry/index/index.crates.io-6f17d22bba15001f/.cache/he/rm/hermit-abi", O_RDONLY|O_CLOEXEC) = 6
statx(6, "", AT_STATX_SYNC_AS_STAT|AT_EMPTY_PATH, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0644, stx_size=26979, ...}) = 0
read(6, "\3\2\0\0\0etag: W/\"462a779b5bce2e1d24"..., 26979) = 26979
read(6, "", 32)                         = 0
close(6)                                = 0
openat(AT_FDCWD, "/root/.cargo/registry/index/index.crates.io-6f17d22bba15001f/.cache/li/bc/libc", O_RDONLY|O_CLOEXEC) = 6
statx(6, "", AT_STATX_SYNC_AS_STAT|AT_EMPTY_PATH, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0644, stx_size=64893, ...}) = 0
brk(0xaaaaeaca4000)                     = 0xaaaaeaca4000
read(6, "\3\2\0\0\0etag: W/\"aa266c71fbb847d50c"..., 64893) = 64893
read(6, "", 32)                         = 0
close(6)                                = 0
openat(AT_FDCWD, "/root/.cargo/registry/index/index.crates.io-6f17d22bba15001f/.cache/wi/na/winapi", O_RDONLY|O_CLOEXEC) = 6
statx(6, "", AT_STATX_SYNC_AS_STAT|AT_EMPTY_PATH, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0644, stx_size=111048, ...}) = 0
brk(0xaaaaeacca000)                     = 0xaaaaeacca000
read(6, "\3\2\0\0\0etag: W/\"d4e6b8aa2c5b372883"..., 111048) = 111048
read(6, "", 32)                         = 0
close(6)                                = 0
openat(AT_FDCWD, "/root/.cargo/registry/index/index.crates.io-6f17d22bba15001f/.cache/wi/nd/windows-sys", O_RDONLY|O_CLOEXEC) = 6
statx(6, "", AT_STATX_SYNC_AS_STAT|AT_EMPTY_PATH, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0644, stx_size=419792, ...}) = 0
mmap(NULL, 421888, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_ANONYMOUS, -1, 0) = 0xffffa8d79000
read(6, "\3\2\0\0\0etag: W/\"c08c449110f7fc47e3"..., 419792) = 419792
read(6, "", 32)                         = 0
close(6)                                = 0
openat(AT_FDCWD, "/root/.cargo/registry/index/index.crates.io-6f17d22bba15001f/.cache/2/cc", O_RDONLY|O_CLOEXEC) = 6
statx(6, "", AT_STATX_SYNC_AS_STAT|AT_EMPTY_PATH, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0644, stx_size=87216, ...}) = 0
brk(0xaaaaeacfa000)                     = 0xaaaaeacfa000
read(6, "\3\2\0\0\0etag: W/\"3ad6a11c4ee5e68cc6"..., 87216) = 87216
read(6, "", 32)                         = 0
close(6)                                = 0
openat(AT_FDCWD, "/root/.cargo/registry/index/index.crates.io-6f17d22bba15001f/.cache/re/do/redox_syscall", O_RDONLY|O_CLOEXEC) = 6
statx(6, "", AT_STATX_SYNC_AS_STAT|AT_EMPTY_PATH, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0644, stx_size=22154, ...}) = 0
read(6, "\3\2\0\0\0etag: W/\"a8756b0702502e5600"..., 22154) = 22154
read(6, "", 32)                         = 0
close(6)                                = 0
openat(AT_FDCWD, "/root/.cargo/registry/index/index.crates.io-6f17d22bba15001f/.cache/wa/si/wasite", O_RDONLY|O_CLOEXEC) = 6
statx(6, "", AT_STATX_SYNC_AS_STAT|AT_EMPTY_PATH, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0644, stx_size=199, ...}) = 0
read(6, "\3\2\0\0\0etag: \"b74c73cb0e3430726142"..., 199) = 199
read(6, "", 32)                         = 0
close(6)                                = 0
openat(AT_FDCWD, "/root/.cargo/registry/index/index.crates.io-6f17d22bba15001f/.cache/we/b-/web-sys", O_RDONLY|O_CLOEXEC) = 6
statx(6, "", AT_STATX_SYNC_AS_STAT|AT_EMPTY_PATH, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0644, stx_size=3038477, ...}) = 0
mmap(NULL, 3039232, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_ANONYMOUS, -1, 0) = 0xffffa8a00000
read(6, "\3\2\0\0\0etag: W/\"254ab95bbaeb9b1b0d"..., 3038477) = 3038477
read(6, "", 32)                         = 0
close(6)                                = 0
brk(0xaaaaead1b000)                     = 0xaaaaead1b000
brk(0xaaaaead3c000)                     = 0xaaaaead3c000
brk(0xaaaaead5d000)                     = 0xaaaaead5d000
openat(AT_FDCWD, "/root/.cargo/registry/index/index.crates.io-6f17d22bba15001f/.cache/wi/na/winapi-i686-pc-windows-gnu", O_RDONLY|O_CLOEXEC) = 6
statx(6, "", AT_STATX_SYNC_AS_STAT|AT_EMPTY_PATH, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0644, stx_size=738, ...}) = 0
read(6, "\3\2\0\0\0etag: \"39255a14221e48d25435"..., 738) = 738
read(6, "", 32)                         = 0
close(6)                                = 0
openat(AT_FDCWD, "/root/.cargo/registry/index/index.crates.io-6f17d22bba15001f/.cache/wi/na/winapi-x86_64-pc-windows-gnu", O_RDONLY|O_CLOEXEC) = 6
statx(6, "", AT_STATX_SYNC_AS_STAT|AT_EMPTY_PATH, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0644, stx_size=746, ...}) = 0
read(6, "\3\2\0\0\0etag: \"c3c1e999f6876c53e467"..., 746) = 746
read(6, "", 32)                         = 0
close(6)                                = 0
openat(AT_FDCWD, "/root/.cargo/registry/index/index.crates.io-6f17d22bba15001f/.cache/wi/nd/windows-targets", O_RDONLY|O_CLOEXEC) = 6
statx(6, "", AT_STATX_SYNC_AS_STAT|AT_EMPTY_PATH, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0644, stx_size=28867, ...}) = 0
read(6, "\3\2\0\0\0etag: W/\"adfb5b9c19cd197fab"..., 28867) = 28867
read(6, "", 32)                         = 0
close(6)                                = 0
openat(AT_FDCWD, "/root/.cargo/registry/index/index.crates.io-6f17d22bba15001f/.cache/sh/le/shlex", O_RDONLY|O_CLOEXEC) = 6
statx(6, "", AT_STATX_SYNC_AS_STAT|AT_EMPTY_PATH, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0644, stx_size=1240, ...}) = 0
read(6, "\3\2\0\0\0etag: W/\"4ca4af30470fcc2bf2"..., 1240) = 1240
read(6, "", 32)                         = 0
close(6)                                = 0
openat(AT_FDCWD, "/root/.cargo/registry/index/index.crates.io-6f17d22bba15001f/.cache/bi/tf/bitflags", O_RDONLY|O_CLOEXEC) = 6
statx(6, "", AT_STATX_SYNC_AS_STAT|AT_EMPTY_PATH, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0644, stx_size=41048, ...}) = 0
brk(0xaaaaead7e000)                     = 0xaaaaead7e000
read(6, "\3\2\0\0\0etag: W/\"4be1aaf1a48d0d2511"..., 41048) = 41048
read(6, "", 32)                         = 0
close(6)                                = 0
openat(AT_FDCWD, "/root/.cargo/registry/index/index.crates.io-6f17d22bba15001f/.cache/js/-s/js-sys", O_RDONLY|O_CLOEXEC) = 6
statx(6, "", AT_STATX_SYNC_AS_STAT|AT_EMPTY_PATH, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0644, stx_size=56093, ...}) = 0
read(6, "\3\2\0\0\0etag: W/\"a8a442940067e01e0d"..., 56093) = 56093
read(6, "", 32)                         = 0
close(6)                                = 0
openat(AT_FDCWD, "/root/.cargo/registry/index/index.crates.io-6f17d22bba15001f/.cache/wa/sm/wasm-bindgen", O_RDONLY|O_CLOEXEC) = 6
statx(6, "", AT_STATX_SYNC_AS_STAT|AT_EMPTY_PATH, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0644, stx_size=129012, ...}) = 0
brk(0xaaaaeadaf000)                     = 0xaaaaeadaf000
read(6, "\3\2\0\0\0etag: W/\"73ef7d4f23777152c8"..., 129012) = 129012
read(6, "", 32)                         = 0
close(6)                                = 0
openat(AT_FDCWD, "/root/.cargo/registry/index/index.crates.io-6f17d22bba15001f/.cache/wi/nd/windows_aarch64_gnullvm", O_RDONLY|O_CLOEXEC) = 6
statx(6, "", AT_STATX_SYNC_AS_STAT|AT_EMPTY_PATH, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0644, stx_size=3467, ...}) = 0
read(6, "\3\2\0\0\0etag: W/\"52f7d74c182af96523"..., 3467) = 3467
read(6, "", 32)                         = 0
close(6)                                = 0
openat(AT_FDCWD, "/root/.cargo/registry/index/index.crates.io-6f17d22bba15001f/.cache/wi/nd/windows_aarch64_msvc", O_RDONLY|O_CLOEXEC) = 6
statx(6, "", AT_STATX_SYNC_AS_STAT|AT_EMPTY_PATH, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0644, stx_size=5947, ...}) = 0
read(6, "\3\2\0\0\0etag: W/\"da6183700e4f0897c5"..., 5947) = 5947
read(6, "", 32)                         = 0
close(6)                                = 0
openat(AT_FDCWD, "/root/.cargo/registry/index/index.crates.io-6f17d22bba15001f/.cache/wi/nd/windows_i686_gnu", O_RDONLY|O_CLOEXEC) = 6
statx(6, "", AT_STATX_SYNC_AS_STAT|AT_EMPTY_PATH, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0644, stx_size=6141, ...}) = 0
read(6, "\3\2\0\0\0etag: W/\"39b3af6f1074a94da5"..., 6141) = 6141
read(6, "", 32)                         = 0
close(6)                                = 0
openat(AT_FDCWD, "/root/.cargo/registry/index/index.crates.io-6f17d22bba15001f/.cache/wi/nd/windows_i686_gnullvm", O_RDONLY|O_CLOEXEC) = 6
statx(6, "", AT_STATX_SYNC_AS_STAT|AT_EMPTY_PATH, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0644, stx_size=808, ...}) = 0
read(6, "\3\2\0\0\0etag: \"7cd017200112ad348915"..., 808) = 808
read(6, "", 32)                         = 0
close(6)                                = 0
openat(AT_FDCWD, "/root/.cargo/registry/index/index.crates.io-6f17d22bba15001f/.cache/wi/nd/windows_i686_msvc", O_RDONLY|O_CLOEXEC) = 6
statx(6, "", AT_STATX_SYNC_AS_STAT|AT_EMPTY_PATH, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0644, stx_size=6343, ...}) = 0
read(6, "\3\2\0\0\0etag: W/\"4ca4d2b0e6332131db"..., 6343) = 6343
read(6, "", 32)                         = 0
close(6)                                = 0
openat(AT_FDCWD, "/root/.cargo/registry/index/index.crates.io-6f17d22bba15001f/.cache/wi/nd/windows_x86_64_gnu", O_RDONLY|O_CLOEXEC) = 6
statx(6, "", AT_STATX_SYNC_AS_STAT|AT_EMPTY_PATH, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0644, stx_size=6213, ...}) = 0
read(6, "\3\2\0\0\0etag: W/\"0c158d704a01a2b0cd"..., 6213) = 6213
read(6, "", 32)                         = 0
close(6)                                = 0
openat(AT_FDCWD, "/root/.cargo/registry/index/index.crates.io-6f17d22bba15001f/.cache/wi/nd/windows_x86_64_gnullvm", O_RDONLY|O_CLOEXEC) = 6
statx(6, "", AT_STATX_SYNC_AS_STAT|AT_EMPTY_PATH, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0644, stx_size=3448, ...}) = 0
read(6, "\3\2\0\0\0etag: W/\"1d50abb988a5b28254"..., 3448) = 3448
read(6, "", 32)                         = 0
close(6)                                = 0
openat(AT_FDCWD, "/root/.cargo/registry/index/index.crates.io-6f17d22bba15001f/.cache/wi/nd/windows_x86_64_msvc", O_RDONLY|O_CLOEXEC) = 6
statx(6, "", AT_STATX_SYNC_AS_STAT|AT_EMPTY_PATH, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0644, stx_size=6417, ...}) = 0
read(6, "\3\2\0\0\0etag: W/\"c7fcc23e2767f5f8ba"..., 6417) = 6417
read(6, "", 32)                         = 0
close(6)                                = 0
openat(AT_FDCWD, "/root/.cargo/registry/index/index.crates.io-6f17d22bba15001f/.cache/on/ce/once_cell", O_RDONLY|O_CLOEXEC) = 6
statx(6, "", AT_STATX_SYNC_AS_STAT|AT_EMPTY_PATH, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0644, stx_size=43778, ...}) = 0
read(6, "\3\2\0\0\0etag: W/\"ffc9d9f2c804a9650a"..., 43778) = 43778
read(6, "", 32)                         = 0
close(6)                                = 0
brk(0xaaaaeadd0000)                     = 0xaaaaeadd0000
openat(AT_FDCWD, "/root/.cargo/registry/index/index.crates.io-6f17d22bba15001f/.cache/cf/g-/cfg-if", O_RDONLY|O_CLOEXEC) = 6
statx(6, "", AT_STATX_SYNC_AS_STAT|AT_EMPTY_PATH, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0644, stx_size=2864, ...}) = 0
read(6, "\3\2\0\0\0etag: W/\"f6afe56eab663c00a5"..., 2864) = 2864
read(6, "", 32)                         = 0
close(6)                                = 0
openat(AT_FDCWD, "/root/.cargo/registry/index/index.crates.io-6f17d22bba15001f/.cache/wa/sm/wasm-bindgen-macro", O_RDONLY|O_CLOEXEC) = 6
statx(6, "", AT_STATX_SYNC_AS_STAT|AT_EMPTY_PATH, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0644, stx_size=83726, ...}) = 0
read(6, "\3\2\0\0\0etag: W/\"6b378349cf9dab76df"..., 83726) = 83726
read(6, "", 32)                         = 0
close(6)                                = 0
brk(0xaaaaeadf1000)                     = 0xaaaaeadf1000
openat(AT_FDCWD, "/root/.cargo/registry/index/index.crates.io-6f17d22bba15001f/.cache/qu/ot/quote", O_RDONLY|O_CLOEXEC) = 6
statx(6, "", AT_STATX_SYNC_AS_STAT|AT_EMPTY_PATH, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0644, stx_size=34103, ...}) = 0
read(6, "\3\2\0\0\0etag: W/\"13261e4a526df5948f"..., 34103) = 34103
read(6, "", 32)                         = 0
close(6)                                = 0
openat(AT_FDCWD, "/root/.cargo/registry/index/index.crates.io-6f17d22bba15001f/.cache/wa/sm/wasm-bindgen-macro-support", O_RDONLY|O_CLOEXEC) = 6
statx(6, "", AT_STATX_SYNC_AS_STAT|AT_EMPTY_PATH, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0644, stx_size=76598, ...}) = 0
brk(0xaaaaeae16000)                     = 0xaaaaeae16000
read(6, "\3\2\0\0\0etag: W/\"d5541654f853428da5"..., 76598) = 76598
read(6, "", 32)                         = 0
close(6)                                = 0
openat(AT_FDCWD, "/root/.cargo/registry/index/index.crates.io-6f17d22bba15001f/.cache/pr/oc/proc-macro2", O_RDONLY|O_CLOEXEC) = 6
statx(6, "", AT_STATX_SYNC_AS_STAT|AT_EMPTY_PATH, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0644, stx_size=75203, ...}) = 0
read(6, "\3\2\0\0\0etag: W/\"7c0912c0b825591527"..., 75203) = 75203
read(6, "", 32)                         = 0
close(6)                                = 0
openat(AT_FDCWD, "/root/.cargo/registry/index/index.crates.io-6f17d22bba15001f/.cache/3/s/syn", O_RDONLY|O_CLOEXEC) = 6
statx(6, "", AT_STATX_SYNC_AS_STAT|AT_EMPTY_PATH, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0644, stx_size=577408, ...}) = 0
mmap(NULL, 577536, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_ANONYMOUS, -1, 0) = 0xffffa8cec000
read(6, "\3\2\0\0\0etag: W/\"c51b93809f54198987"..., 577408) = 577408
read(6, "", 32)                         = 0
close(6)                                = 0
brk(0xaaaaeae3b000)                     = 0xaaaaeae3b000
openat(AT_FDCWD, "/root/.cargo/registry/index/index.crates.io-6f17d22bba15001f/.cache/wa/sm/wasm-bindgen-backend", O_RDONLY|O_CLOEXEC) = 6
statx(6, "", AT_STATX_SYNC_AS_STAT|AT_EMPTY_PATH, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0644, stx_size=101717, ...}) = 0
brk(0xaaaaeae60000)                     = 0xaaaaeae60000
read(6, "\3\2\0\0\0etag: W/\"e91ff8991d0e6eb9ef"..., 101717) = 101717
read(6, "", 32)                         = 0
close(6)                                = 0
openat(AT_FDCWD, "/root/.cargo/registry/index/index.crates.io-6f17d22bba15001f/.cache/wa/sm/wasm-bindgen-shared", O_RDONLY|O_CLOEXEC) = 6
statx(6, "", AT_STATX_SYNC_AS_STAT|AT_EMPTY_PATH, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0644, stx_size=26216, ...}) = 0
read(6, "\3\2\0\0\0etag: W/\"c5e27c07d587bf8830"..., 26216) = 26216
read(6, "", 32)                         = 0
close(6)                                = 0
openat(AT_FDCWD, "/root/.cargo/registry/index/index.crates.io-6f17d22bba15001f/.cache/un/ic/unicode-ident", O_RDONLY|O_CLOEXEC) = 6
statx(6, "", AT_STATX_SYNC_AS_STAT|AT_EMPTY_PATH, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0644, stx_size=15173, ...}) = 0
read(6, "\3\2\0\0\0etag: W/\"1e15bc258865f5245a"..., 15173) = 15173
read(6, "", 32)                         = 0
close(6)                                = 0
openat(AT_FDCWD, "/root/.cargo/registry/index/index.crates.io-6f17d22bba15001f/.cache/bu/mp/bumpalo", O_RDONLY|O_CLOEXEC) = 6
statx(6, "", AT_STATX_SYNC_AS_STAT|AT_EMPTY_PATH, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0644, stx_size=26402, ...}) = 0
read(6, "\3\2\0\0\0etag: W/\"ce3ecc16b657e38836"..., 26402) = 26402
read(6, "", 32)                         = 0
close(6)                                = 0
openat(AT_FDCWD, "/root/.cargo/registry/index/index.crates.io-6f17d22bba15001f/.cache/3/l/log", O_RDONLY|O_CLOEXEC) = 6
statx(6, "", AT_STATX_SYNC_AS_STAT|AT_EMPTY_PATH, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0644, stx_size=46441, ...}) = 0
read(6, "\3\2\0\0\0etag: W/\"a9583755f46c25252d"..., 46441) = 46441
read(6, "", 32)                         = 0
close(6)                                = 0
brk(0xaaaaeae81000)                     = 0xaaaaeae81000
brk(0xaaaaeaea2000)                     = 0xaaaaeaea2000
newfstatat(AT_FDCWD, "/root", {st_mode=S_IFDIR|0700, st_size=4096, ...}, AT_SYMLINK_NOFOLLOW) = 0
newfstatat(AT_FDCWD, "/root/.cargo", {st_mode=S_IFDIR|0755, st_size=4096, ...}, AT_SYMLINK_NOFOLLOW) = 0
newfstatat(AT_FDCWD, "/root/.cargo/.global-cache", {st_mode=S_IFREG|0644, st_size=57344, ...}, AT_SYMLINK_NOFOLLOW) = 0
getpid()                                = 749
getpid()                                = 749
openat(AT_FDCWD, "/root/.cargo/.global-cache", O_RDWR|O_CREAT|O_NOFOLLOW|O_CLOEXEC, 0644) = 6
fstat(6, {st_mode=S_IFREG|0644, st_size=57344, ...}) = 0
fstat(6, {st_mode=S_IFREG|0644, st_size=57344, ...}) = 0
newfstatat(AT_FDCWD, "/root/.cargo/.global-cache", {st_mode=S_IFREG|0644, st_size=57344, ...}, 0) = 0
pread64(6, "SQLite format 3\0\20\0\1\1\0@  \0\0\0!\0\0\0\16"..., 100, 0) = 100
fcntl(6, F_SETLK, {l_type=F_RDLCK, l_whence=SEEK_SET, l_start=1073741824, l_len=1}) = 0
fcntl(6, F_SETLK, {l_type=F_RDLCK, l_whence=SEEK_SET, l_start=1073741826, l_len=510}) = 0
fcntl(6, F_SETLK, {l_type=F_UNLCK, l_whence=SEEK_SET, l_start=1073741824, l_len=1}) = 0
newfstatat(AT_FDCWD, "/root/.cargo/.global-cache-journal", 0xffffc61b8510, 0) = -1 ENOENT (No such file or directory)
newfstatat(AT_FDCWD, "/root/.cargo/.global-cache-wal", 0xffffc61b8510, 0) = -1 ENOENT (No such file or directory)
fstat(6, {st_mode=S_IFREG|0644, st_size=57344, ...}) = 0
pread64(6, "SQLite format 3\0\20\0\1\1\0@  \0\0\0!\0\0\0\16"..., 4096, 0) = 4096
fcntl(6, F_SETLK, {l_type=F_WRLCK, l_whence=SEEK_SET, l_start=1073741825, l_len=1}) = 0
fcntl(6, F_SETLK, {l_type=F_WRLCK, l_whence=SEEK_SET, l_start=1073741824, l_len=1}) = 0
fcntl(6, F_SETLK, {l_type=F_WRLCK, l_whence=SEEK_SET, l_start=1073741826, l_len=510}) = 0
fcntl(6, F_SETLK, {l_type=F_RDLCK, l_whence=SEEK_SET, l_start=1073741826, l_len=510}) = 0
fcntl(6, F_SETLK, {l_type=F_UNLCK, l_whence=SEEK_SET, l_start=1073741824, l_len=2}) = 0
fcntl(6, F_SETLK, {l_type=F_UNLCK, l_whence=SEEK_SET, l_start=0, l_len=0}) = 0
fcntl(6, F_SETLK, {l_type=F_RDLCK, l_whence=SEEK_SET, l_start=1073741824, l_len=1}) = 0
fcntl(6, F_SETLK, {l_type=F_RDLCK, l_whence=SEEK_SET, l_start=1073741826, l_len=510}) = 0
fcntl(6, F_SETLK, {l_type=F_UNLCK, l_whence=SEEK_SET, l_start=1073741824, l_len=1}) = 0
newfstatat(AT_FDCWD, "/root/.cargo/.global-cache-journal", 0xffffc61b86b0, 0) = -1 ENOENT (No such file or directory)
pread64(6, "\0\0\0!\0\0\0\16\0\0\0\0\0\0\0\0", 16, 24) = 16
newfstatat(AT_FDCWD, "/root/.cargo/.global-cache-wal", 0xffffc61b86b0, 0) = -1 ENOENT (No such file or directory)
fstat(6, {st_mode=S_IFREG|0644, st_size=57344, ...}) = 0
pread64(6, "\n\0\0\0\1\17\334\0\17\334\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"..., 4096, 8192) = 4096
pread64(6, "\r\0\0\0\1\17\326\0\17\326\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"..., 4096, 4096) = 4096
fcntl(6, F_SETLK, {l_type=F_UNLCK, l_whence=SEEK_SET, l_start=0, l_len=0}) = 0
flock(3, LOCK_UN)                       = 0
close(3)                                = 0
openat(AT_FDCWD, "/hdp/Cargo.lock", O_RDONLY|O_CLOEXEC) = 3
statfs("/hdp/Cargo.lock", {f_type=OVERLAYFS_SUPER_MAGIC, f_bsize=4096, f_blocks=263940461, f_bfree=261336862, f_bavail=247911006, f_files=67108864, f_ffree=66876332, f_fsid={val=[0x25291b5, 0x7d4d1bca]}, f_namelen=255, f_frsize=4096, f_flags=ST_VALID|ST_RELATIME}) = 0
flock(3, LOCK_SH|LOCK_NB)               = 0
read(3, "# This file is automatically @ge", 32) = 32
read(3, "nerated by Cargo.\n# It is not in", 32) = 32
read(3, "tended for manual editing.\nversi"..., 64) = 64
read(3, "\"\nversion = \"0.2.14\"\nsource = \"r"..., 128) = 128
read(3, "dd3f37e93092cbf377614828a319d5fe"..., 256) = 256
read(3, "1fb82e453128456482613d36\"\n\n[[pac"..., 512) = 512
read(3, "ub.com/rust-lang/crates.io-index"..., 1024) = 1024
read(3, "dba1cbe1c6ab0b15e29fff3f6c077fd9"..., 2048) = 2048
read(3, "version = \"0.1.0\"\nsource = \"regi"..., 4096) = 4096
read(3, "rust-lang/crates.io-index\"\ncheck"..., 8192) = 921
read(3, "", 7271)                       = 0
flock(3, LOCK_UN)                       = 0
close(3)                                = 0
openat(AT_FDCWD, "/root/.cargo/.package-cache", O_RDWR|O_CREAT|O_CLOEXEC, 0666) = 3
statfs("/root/.cargo/.package-cache", {f_type=OVERLAYFS_SUPER_MAGIC, f_bsize=4096, f_blocks=263940461, f_bfree=261336862, f_bavail=247911006, f_files=67108864, f_ffree=66876332, f_fsid={val=[0x25291b5, 0x7d4d1bca]}, f_namelen=255, f_frsize=4096, f_flags=ST_VALID|ST_RELATIME}) = 0
flock(3, LOCK_EX|LOCK_NB)               = 0
brk(0xaaaaeaec3000)                     = 0xaaaaeaec3000
flock(3, LOCK_UN)                       = 0
close(3)                                = 0
socketpair(AF_UNIX, SOCK_STREAM|SOCK_CLOEXEC|SOCK_NONBLOCK, 0, [3, 7]) = 0
openat(AT_FDCWD, "/root/.cargo/.package-cache", O_RDWR|O_CREAT|O_CLOEXEC, 0666) = 8
statfs("/root/.cargo/.package-cache", {f_type=OVERLAYFS_SUPER_MAGIC, f_bsize=4096, f_blocks=263940461, f_bfree=261336862, f_bavail=247911006, f_files=67108864, f_ffree=66876332, f_fsid={val=[0x25291b5, 0x7d4d1bca]}, f_namelen=255, f_frsize=4096, f_flags=ST_VALID|ST_RELATIME}) = 0
flock(8, LOCK_EX|LOCK_NB)               = 0
openat(AT_FDCWD, "/root/.cargo/registry/index/index.crates.io-6f17d22bba15001f/config.json", O_RDONLY|O_CLOEXEC) = 9
statx(9, "", AT_STATX_SYNC_AS_STAT|AT_EMPTY_PATH, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0644, stx_size=76, ...}) = 0
read(9, "{\n  \"dl\": \"https://static.crates"..., 76) = 76
read(9, "", 32)                         = 0
close(9)                                = 0
openat(AT_FDCWD, "/root/.cargo/registry/cache/index.crates.io-6f17d22bba15001f/atty-0.2.14.crate", O_RDONLY|O_CLOEXEC) = 9
statx(9, "", AT_STATX_SYNC_AS_STAT|AT_EMPTY_PATH, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0644, stx_size=5470, ...}) = 0
openat(AT_FDCWD, "/root/.cargo/registry/src/index.crates.io-6f17d22bba15001f/atty-0.2.14/.cargo-ok", O_RDONLY|O_CLOEXEC) = 10
statx(10, "", AT_STATX_SYNC_AS_STAT|AT_EMPTY_PATH, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0644, stx_size=7, ...}) = 0
read(10, "{\"v\":1}", 7)                = 7
read(10, "", 32)                        = 0
close(10)                               = 0
openat(AT_FDCWD, "/root/.cargo/registry/src/index.crates.io-6f17d22bba15001f/atty-0.2.14/Cargo.toml", O_RDONLY|O_CLOEXEC) = 10
statx(10, "", AT_STATX_SYNC_AS_STAT|AT_EMPTY_PATH, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0644, stx_size=1283, ...}) = 0
read(10, "# THIS FILE IS AUTOMATICALLY GEN"..., 1283) = 1283
read(10, "", 32)                        = 0
close(10)                               = 0
statx(AT_FDCWD, "/root/.cargo/registry/src/index.crates.io-6f17d22bba15001f/atty-0.2.14/build.rs", AT_STATX_SYNC_AS_STAT, STATX_ALL, 0xffffc61b24c0) = -1 ENOENT (No such file or directory)
statx(AT_FDCWD, "/root/.cargo/registry/src/index.crates.io-6f17d22bba15001f/atty-0.2.14/src/lib.rs", AT_STATX_SYNC_AS_STAT, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0644, stx_size=5460, ...}) = 0
statx(AT_FDCWD, "/root/.cargo/registry/src/index.crates.io-6f17d22bba15001f/atty-0.2.14/src/main.rs", AT_STATX_SYNC_AS_STAT, STATX_ALL, 0xffffc61b2e70) = -1 ENOENT (No such file or directory)
openat(AT_FDCWD, "/root/.cargo/registry/src/index.crates.io-6f17d22bba15001f/atty-0.2.14/src/bin", O_RDONLY|O_NONBLOCK|O_CLOEXEC|O_DIRECTORY) = -1 ENOENT (No such file or directory)
openat(AT_FDCWD, "/root/.cargo/registry/src/index.crates.io-6f17d22bba15001f/atty-0.2.14/examples", O_RDONLY|O_NONBLOCK|O_CLOEXEC|O_DIRECTORY) = 10
newfstatat(10, "", {st_mode=S_IFDIR|0755, st_size=4096, ...}, AT_EMPTY_PATH) = 0
brk(0xaaaaeaee5000)                     = 0xaaaaeaee5000
getdents64(10, 0xaaaaeaebc100 /* 3 entries */, 32768) = 80
getdents64(10, 0xaaaaeaebc100 /* 0 entries */, 32768) = 0
brk(0xaaaaeaedd000)                     = 0xaaaaeaedd000
close(10)                               = 0
openat(AT_FDCWD, "/root/.cargo/registry/src/index.crates.io-6f17d22bba15001f/atty-0.2.14/tests", O_RDONLY|O_NONBLOCK|O_CLOEXEC|O_DIRECTORY) = -1 ENOENT (No such file or directory)
openat(AT_FDCWD, "/root/.cargo/registry/src/index.crates.io-6f17d22bba15001f/atty-0.2.14/benches", O_RDONLY|O_NONBLOCK|O_CLOEXEC|O_DIRECTORY) = -1 ENOENT (No such file or directory)
statx(AT_FDCWD, "/root/.cargo/registry/src/index.crates.io-6f17d22bba15001f/atty-0.2.14", AT_STATX_SYNC_AS_STAT, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFDIR|0755, stx_size=4096, ...}) = 0
close(9)                                = 0
openat(AT_FDCWD, "/root/.cargo/registry/cache/index.crates.io-6f17d22bba15001f/cc-1.2.14.crate", O_RDONLY|O_CLOEXEC) = 9
statx(9, "", AT_STATX_SYNC_AS_STAT|AT_EMPTY_PATH, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0644, stx_size=103164, ...}) = 0
openat(AT_FDCWD, "/root/.cargo/registry/src/index.crates.io-6f17d22bba15001f/cc-1.2.14/.cargo-ok", O_RDONLY|O_CLOEXEC) = 10
statx(10, "", AT_STATX_SYNC_AS_STAT|AT_EMPTY_PATH, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0644, stx_size=7, ...}) = 0
read(10, "{\"v\":1}", 7)                = 7
read(10, "", 32)                        = 0
close(10)                               = 0
openat(AT_FDCWD, "/root/.cargo/registry/src/index.crates.io-6f17d22bba15001f/cc-1.2.14/Cargo.toml", O_RDONLY|O_CLOEXEC) = 10
statx(10, "", AT_STATX_SYNC_AS_STAT|AT_EMPTY_PATH, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0644, stx_size=1608, ...}) = 0
read(10, "# THIS FILE IS AUTOMATICALLY GEN"..., 1608) = 1608
read(10, "", 32)                        = 0
close(10)                               = 0
statx(AT_FDCWD, "/root/.cargo/registry/src/index.crates.io-6f17d22bba15001f/cc-1.2.14", AT_STATX_SYNC_AS_STAT, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFDIR|0755, stx_size=4096, ...}) = 0
close(9)                                = 0
openat(AT_FDCWD, "/root/.cargo/registry/cache/index.crates.io-6f17d22bba15001f/libc-0.2.169.crate", O_RDONLY|O_CLOEXEC) = 9
statx(9, "", AT_STATX_SYNC_AS_STAT|AT_EMPTY_PATH, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0644, stx_size=757901, ...}) = 0
openat(AT_FDCWD, "/root/.cargo/registry/src/index.crates.io-6f17d22bba15001f/libc-0.2.169/.cargo-ok", O_RDONLY|O_CLOEXEC) = 10
statx(10, "", AT_STATX_SYNC_AS_STAT|AT_EMPTY_PATH, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0644, stx_size=7, ...}) = 0
read(10, "{\"v\":1}", 7)                = 7
read(10, "", 32)                        = 0
close(10)                               = 0
openat(AT_FDCWD, "/root/.cargo/registry/src/index.crates.io-6f17d22bba15001f/libc-0.2.169/Cargo.toml", O_RDONLY|O_CLOEXEC) = 10
statx(10, "", AT_STATX_SYNC_AS_STAT|AT_EMPTY_PATH, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0644, stx_size=5014, ...}) = 0
read(10, "# THIS FILE IS AUTOMATICALLY GEN"..., 5014) = 5014
read(10, "", 32)                        = 0
close(10)                               = 0
statx(AT_FDCWD, "/root/.cargo/registry/src/index.crates.io-6f17d22bba15001f/libc-0.2.169", AT_STATX_SYNC_AS_STAT, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFDIR|0755, stx_size=4096, ...}) = 0
close(9)                                = 0
openat(AT_FDCWD, "/root/.cargo/registry/cache/index.crates.io-6f17d22bba15001f/shlex-1.3.0.crate", O_RDONLY|O_CLOEXEC) = 9
statx(9, "", AT_STATX_SYNC_AS_STAT|AT_EMPTY_PATH, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0644, stx_size=18713, ...}) = 0
openat(AT_FDCWD, "/root/.cargo/registry/src/index.crates.io-6f17d22bba15001f/shlex-1.3.0/.cargo-ok", O_RDONLY|O_CLOEXEC) = 10
statx(10, "", AT_STATX_SYNC_AS_STAT|AT_EMPTY_PATH, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0644, stx_size=7, ...}) = 0
read(10, "{\"v\":1}", 7)                = 7
read(10, "", 32)                        = 0
close(10)                               = 0
openat(AT_FDCWD, "/root/.cargo/registry/src/index.crates.io-6f17d22bba15001f/shlex-1.3.0/Cargo.toml", O_RDONLY|O_CLOEXEC) = 10
statx(10, "", AT_STATX_SYNC_AS_STAT|AT_EMPTY_PATH, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0644, stx_size=1074, ...}) = 0
read(10, "# THIS FILE IS AUTOMATICALLY GEN"..., 1074) = 1074
read(10, "", 32)                        = 0
close(10)                               = 0
statx(AT_FDCWD, "/root/.cargo/registry/src/index.crates.io-6f17d22bba15001f/shlex-1.3.0/build.rs", AT_STATX_SYNC_AS_STAT, STATX_ALL, 0xffffc61b24c0) = -1 ENOENT (No such file or directory)
statx(AT_FDCWD, "/root/.cargo/registry/src/index.crates.io-6f17d22bba15001f/shlex-1.3.0/src/lib.rs", AT_STATX_SYNC_AS_STAT, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0644, stx_size=13338, ...}) = 0
statx(AT_FDCWD, "/root/.cargo/registry/src/index.crates.io-6f17d22bba15001f/shlex-1.3.0/src/main.rs", AT_STATX_SYNC_AS_STAT, STATX_ALL, 0xffffc61b2e70) = -1 ENOENT (No such file or directory)
openat(AT_FDCWD, "/root/.cargo/registry/src/index.crates.io-6f17d22bba15001f/shlex-1.3.0/src/bin", O_RDONLY|O_NONBLOCK|O_CLOEXEC|O_DIRECTORY) = -1 ENOENT (No such file or directory)
openat(AT_FDCWD, "/root/.cargo/registry/src/index.crates.io-6f17d22bba15001f/shlex-1.3.0/examples", O_RDONLY|O_NONBLOCK|O_CLOEXEC|O_DIRECTORY) = -1 ENOENT (No such file or directory)
openat(AT_FDCWD, "/root/.cargo/registry/src/index.crates.io-6f17d22bba15001f/shlex-1.3.0/tests", O_RDONLY|O_NONBLOCK|O_CLOEXEC|O_DIRECTORY) = -1 ENOENT (No such file or directory)
openat(AT_FDCWD, "/root/.cargo/registry/src/index.crates.io-6f17d22bba15001f/shlex-1.3.0/benches", O_RDONLY|O_NONBLOCK|O_CLOEXEC|O_DIRECTORY) = -1 ENOENT (No such file or directory)
statx(AT_FDCWD, "/root/.cargo/registry/src/index.crates.io-6f17d22bba15001f/shlex-1.3.0", AT_STATX_SYNC_AS_STAT, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFDIR|0755, stx_size=4096, ...}) = 0
close(9)                                = 0
openat(AT_FDCWD, "/root/.cargo/registry/cache/index.crates.io-6f17d22bba15001f/socket2-0.5.8.crate", O_RDONLY|O_CLOEXEC) = 9
statx(9, "", AT_STATX_SYNC_AS_STAT|AT_EMPTY_PATH, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0644, stx_size=56309, ...}) = 0
openat(AT_FDCWD, "/root/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.8/.cargo-ok", O_RDONLY|O_CLOEXEC) = 10
statx(10, "", AT_STATX_SYNC_AS_STAT|AT_EMPTY_PATH, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0644, stx_size=7, ...}) = 0
read(10, "{\"v\":1}", 7)                = 7
read(10, "", 32)                        = 0
close(10)                               = 0
openat(AT_FDCWD, "/root/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.8/Cargo.toml", O_RDONLY|O_CLOEXEC) = 10
statx(10, "", AT_STATX_SYNC_AS_STAT|AT_EMPTY_PATH, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0644, stx_size=2227, ...}) = 0
read(10, "# THIS FILE IS AUTOMATICALLY GEN"..., 2227) = 2227
read(10, "", 32)                        = 0
close(10)                               = 0
brk(0xaaaaeaf00000)                     = 0xaaaaeaf00000
statx(AT_FDCWD, "/root/.cargo/registry/src/index.crates.io-6f17d22bba15001f/socket2-0.5.8", AT_STATX_SYNC_AS_STAT, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFDIR|0755, stx_size=4096, ...}) = 0
close(9)                                = 0
openat(AT_FDCWD, "/root/.cargo/registry/cache/index.crates.io-6f17d22bba15001f/sys-info-0.9.1.crate", O_RDONLY|O_CLOEXEC) = 9
statx(9, "", AT_STATX_SYNC_AS_STAT|AT_EMPTY_PATH, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0644, stx_size=20589, ...}) = 0
openat(AT_FDCWD, "/root/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sys-info-0.9.1/.cargo-ok", O_RDONLY|O_CLOEXEC) = 10
statx(10, "", AT_STATX_SYNC_AS_STAT|AT_EMPTY_PATH, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0644, stx_size=7, ...}) = 0
read(10, "{\"v\":1}", 7)                = 7
read(10, "", 32)                        = 0
close(10)                               = 0
openat(AT_FDCWD, "/root/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sys-info-0.9.1/Cargo.toml", O_RDONLY|O_CLOEXEC) = 10
statx(10, "", AT_STATX_SYNC_AS_STAT|AT_EMPTY_PATH, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0644, stx_size=1106, ...}) = 0
read(10, "# THIS FILE IS AUTOMATICALLY GEN"..., 1106) = 1106
read(10, "", 32)                        = 0
close(10)                               = 0
statx(AT_FDCWD, "/root/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sys-info-0.9.1/src/lib.rs", AT_STATX_SYNC_AS_STAT, STATX_ALL, 0xffffc61b2e70) = -1 ENOENT (No such file or directory)
statx(AT_FDCWD, "/root/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sys-info-0.9.1/src/main.rs", AT_STATX_SYNC_AS_STAT, STATX_ALL, 0xffffc61b2e70) = -1 ENOENT (No such file or directory)
openat(AT_FDCWD, "/root/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sys-info-0.9.1/src/bin", O_RDONLY|O_NONBLOCK|O_CLOEXEC|O_DIRECTORY) = -1 ENOENT (No such file or directory)
openat(AT_FDCWD, "/root/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sys-info-0.9.1/examples", O_RDONLY|O_NONBLOCK|O_CLOEXEC|O_DIRECTORY) = -1 ENOENT (No such file or directory)
openat(AT_FDCWD, "/root/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sys-info-0.9.1/tests", O_RDONLY|O_NONBLOCK|O_CLOEXEC|O_DIRECTORY) = -1 ENOENT (No such file or directory)
openat(AT_FDCWD, "/root/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sys-info-0.9.1/benches", O_RDONLY|O_NONBLOCK|O_CLOEXEC|O_DIRECTORY) = -1 ENOENT (No such file or directory)
statx(AT_FDCWD, "/root/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sys-info-0.9.1", AT_STATX_SYNC_AS_STAT, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFDIR|0755, stx_size=4096, ...}) = 0
close(9)                                = 0
openat(AT_FDCWD, "/root/.cargo/registry/cache/index.crates.io-6f17d22bba15001f/whoami-1.5.2.crate", O_RDONLY|O_CLOEXEC) = 9
statx(9, "", AT_STATX_SYNC_AS_STAT|AT_EMPTY_PATH, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0644, stx_size=24204, ...}) = 0
openat(AT_FDCWD, "/root/.cargo/registry/src/index.crates.io-6f17d22bba15001f/whoami-1.5.2/.cargo-ok", O_RDONLY|O_CLOEXEC) = 10
statx(10, "", AT_STATX_SYNC_AS_STAT|AT_EMPTY_PATH, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0644, stx_size=7, ...}) = 0
read(10, "{\"v\":1}", 7)                = 7
read(10, "", 32)                        = 0
close(10)                               = 0
openat(AT_FDCWD, "/root/.cargo/registry/src/index.crates.io-6f17d22bba15001f/whoami-1.5.2/Cargo.toml", O_RDONLY|O_CLOEXEC) = 10
statx(10, "", AT_STATX_SYNC_AS_STAT|AT_EMPTY_PATH, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0644, stx_size=1829, ...}) = 0
read(10, "# THIS FILE IS AUTOMATICALLY GEN"..., 1829) = 1829
read(10, "", 32)                        = 0
close(10)                               = 0
brk(0xaaaaeaf22000)                     = 0xaaaaeaf22000
statx(AT_FDCWD, "/root/.cargo/registry/src/index.crates.io-6f17d22bba15001f/whoami-1.5.2/src/lib.rs", AT_STATX_SYNC_AS_STAT, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0644, stx_size=2629, ...}) = 0
statx(AT_FDCWD, "/root/.cargo/registry/src/index.crates.io-6f17d22bba15001f/whoami-1.5.2", AT_STATX_SYNC_AS_STAT, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFDIR|0755, stx_size=4096, ...}) = 0
close(9)                                = 0
fcntl(6, F_SETLK, {l_type=F_RDLCK, l_whence=SEEK_SET, l_start=1073741824, l_len=1}) = 0
fcntl(6, F_SETLK, {l_type=F_RDLCK, l_whence=SEEK_SET, l_start=1073741826, l_len=510}) = 0
fcntl(6, F_SETLK, {l_type=F_UNLCK, l_whence=SEEK_SET, l_start=1073741824, l_len=1}) = 0
newfstatat(AT_FDCWD, "/root/.cargo/.global-cache-journal", 0xffffc61b9050, 0) = -1 ENOENT (No such file or directory)
pread64(6, "\0\0\0!\0\0\0\16\0\0\0\0\0\0\0\0", 16, 24) = 16
newfstatat(AT_FDCWD, "/root/.cargo/.global-cache-wal", 0xffffc61b9050, 0) = -1 ENOENT (No such file or directory)
fstat(6, {st_mode=S_IFREG|0644, st_size=57344, ...}) = 0
fcntl(6, F_SETLK, {l_type=F_WRLCK, l_whence=SEEK_SET, l_start=1073741825, l_len=1}) = 0
pread64(6, "\r\0\0\0\7\17&\0\17\342\17\301\17\241\17\200\17c\17D\17&\0\0\0\0\0\0\0\0\0\0"..., 4096, 16384) = 4096
pread64(6, "\n\0\0\0\7\17[\0\17\352\17\212\17\271\17[\17\321\17\237\17r\0\0\0\0\0\0\0\0\0\0"..., 4096, 20480) = 4096
pread64(6, "\r\0\0\0\7\17M\0\17\350\17\316\17\263\17\231\17\200\17i\17M\0\0\0\0\0\0\0\0\0\0"..., 4096, 24576) = 4096
pread64(6, "\n\0\0\0\7\17\205\0\17\360\17\231\17\336\17\250\17\313\17\205\17\271\0\0\0\0\0\0\0\0\0\0"..., 4096, 28672) = 4096
fcntl(6, F_SETLK, {l_type=F_WRLCK, l_whence=SEEK_SET, l_start=1073741824, l_len=1}) = 0
fcntl(6, F_SETLK, {l_type=F_WRLCK, l_whence=SEEK_SET, l_start=1073741826, l_len=510}) = 0
fcntl(6, F_SETLK, {l_type=F_RDLCK, l_whence=SEEK_SET, l_start=1073741826, l_len=510}) = 0
fcntl(6, F_SETLK, {l_type=F_UNLCK, l_whence=SEEK_SET, l_start=1073741824, l_len=2}) = 0
fcntl(6, F_SETLK, {l_type=F_UNLCK, l_whence=SEEK_SET, l_start=0, l_len=0}) = 0
flock(8, LOCK_UN)                       = 0
close(8)                                = 0
openat(AT_FDCWD, "/root/.cargo/.package-cache", O_RDWR|O_CREAT|O_CLOEXEC, 0666) = 8
statfs("/root/.cargo/.package-cache", {f_type=OVERLAYFS_SUPER_MAGIC, f_bsize=4096, f_blocks=263940461, f_bfree=261336858, f_bavail=247911002, f_files=67108864, f_ffree=66876332, f_fsid={val=[0x25291b5, 0x7d4d1bca]}, f_namelen=255, f_frsize=4096, f_flags=ST_VALID|ST_RELATIME}) = 0
flock(8, LOCK_EX|LOCK_NB)               = 0
flock(8, LOCK_UN)                       = 0
close(8)                                = 0
statx(AT_FDCWD, "/root/.cargo/bin/rustc", AT_STATX_SYNC_AS_STAT, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0755, stx_size=15187976, ...}) = 0
statx(AT_FDCWD, "/root/.cargo/bin/rustup", AT_STATX_SYNC_AS_STAT, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0755, stx_size=15187976, ...}) = 0
statx(AT_FDCWD, "/root/.cargo/bin/rustc", AT_STATX_SYNC_AS_STAT, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0755, stx_size=15187976, ...}) = 0
statx(AT_FDCWD, "/root/.cargo/bin/rustup", AT_STATX_SYNC_AS_STAT, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0755, stx_size=15187976, ...}) = 0
getcwd("/hdp", 512)                     = 5
statx(AT_FDCWD, "/root/.rustup/toolchains/stable-aarch64-unknown-linux-gnu/bin/rustc", AT_STATX_SYNC_AS_STAT, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0755, stx_size=13632, ...}) = 0
statx(AT_FDCWD, "/root/.rustup/toolchains/stable-aarch64-unknown-linux-gnu/bin/rustc", AT_STATX_SYNC_AS_STAT, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0755, stx_size=13632, ...}) = 0
statx(AT_FDCWD, "/root/.rustup/toolchains/stable-aarch64-unknown-linux-gnu/bin/rustc", AT_STATX_SYNC_AS_STAT, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0755, stx_size=13632, ...}) = 0
openat(AT_FDCWD, "/hdp/target/.rustc_info.json", O_RDONLY|O_CLOEXEC) = 8
statx(8, "", AT_STATX_SYNC_AS_STAT|AT_EMPTY_PATH, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0644, stx_size=1015, ...}) = 0
read(8, "{\"rustc_fingerprint\":15931981652"..., 1015) = 1015
read(8, "", 32)                         = 0
close(8)                                = 0
pipe2([8, 9], O_CLOEXEC)                = 0
fcntl(9, F_SETFL, O_RDONLY|O_NONBLOCK)  = 0
write(9, "||||||||||", 10)              = 10
fcntl(9, F_SETFL, O_RDONLY)             = 0
read(8, "|", 1)                         = 1
openat(AT_FDCWD, "/root/.cargo/.package-cache-mutate", O_RDWR|O_CREAT|O_CLOEXEC, 0666) = 10
statfs("/root/.cargo/.package-cache-mutate", {f_type=OVERLAYFS_SUPER_MAGIC, f_bsize=4096, f_blocks=263940461, f_bfree=261336857, f_bavail=247911001, f_files=67108864, f_ffree=66876332, f_fsid={val=[0x25291b5, 0x7d4d1bca]}, f_namelen=255, f_frsize=4096, f_flags=ST_VALID|ST_RELATIME}) = 0
flock(10, LOCK_SH|LOCK_NB)              = 0
statx(AT_FDCWD, "/hdp/target", AT_STATX_SYNC_AS_STAT, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFDIR|0755, stx_size=4096, ...}) = 0
mkdirat(AT_FDCWD, "/hdp/target/debug", 0777) = -1 EEXIST (File exists)
statx(AT_FDCWD, "/hdp/target/debug", AT_STATX_SYNC_AS_STAT, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFDIR|0755, stx_size=4096, ...}) = 0
openat(AT_FDCWD, "/hdp/target/debug/.cargo-lock", O_RDWR|O_CREAT|O_CLOEXEC, 0666) = 11
statfs("/hdp/target/debug/.cargo-lock", {f_type=OVERLAYFS_SUPER_MAGIC, f_bsize=4096, f_blocks=263940461, f_bfree=261336857, f_bavail=247911001, f_files=67108864, f_ffree=66876332, f_fsid={val=[0x25291b5, 0x7d4d1bca]}, f_namelen=255, f_frsize=4096, f_flags=ST_VALID|ST_RELATIME}) = 0
flock(11, LOCK_EX|LOCK_NB)              = 0
mkdirat(AT_FDCWD, "/hdp/target/debug/deps", 0777) = -1 EEXIST (File exists)
statx(AT_FDCWD, "/hdp/target/debug/deps", AT_STATX_SYNC_AS_STAT, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFDIR|0755, stx_size=40960, ...}) = 0
mkdirat(AT_FDCWD, "/hdp/target/debug/incremental", 0777) = -1 EEXIST (File exists)
statx(AT_FDCWD, "/hdp/target/debug/incremental", AT_STATX_SYNC_AS_STAT, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFDIR|0755, stx_size=4096, ...}) = 0
mkdirat(AT_FDCWD, "/hdp/target/debug/.fingerprint", 0777) = -1 EEXIST (File exists)
statx(AT_FDCWD, "/hdp/target/debug/.fingerprint", AT_STATX_SYNC_AS_STAT, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFDIR|0755, stx_size=4096, ...}) = 0
mkdirat(AT_FDCWD, "/hdp/target/debug/examples", 0777) = -1 EEXIST (File exists)
statx(AT_FDCWD, "/hdp/target/debug/examples", AT_STATX_SYNC_AS_STAT, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFDIR|0755, stx_size=4096, ...}) = 0
mkdirat(AT_FDCWD, "/hdp/target/debug/build", 0777) = -1 EEXIST (File exists)
statx(AT_FDCWD, "/hdp/target/debug/build", AT_STATX_SYNC_AS_STAT, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFDIR|0755, stx_size=4096, ...}) = 0
openat(AT_FDCWD, "/hdp/target/debug/build/libc-18822760ea5a2116/root-output", O_RDONLY|O_CLOEXEC) = 12
statx(12, "", AT_STATX_SYNC_AS_STAT|AT_EMPTY_PATH, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0644, stx_size=49, ...}) = 0
read(12, "/hdp/target/debug/build/libc-188"..., 49) = 49
read(12, "", 32)                        = 0
close(12)                               = 0
openat(AT_FDCWD, "/hdp/target/debug/build/libc-18822760ea5a2116/output", O_RDONLY|O_CLOEXEC) = 12
statx(12, "", AT_STATX_SYNC_AS_STAT|AT_EMPTY_PATH, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0644, stx_size=915, ...}) = 0
read(12, "cargo:rerun-if-changed=build.rs\n"..., 915) = 915
read(12, "", 32)                        = 0
close(12)                               = 0
openat(AT_FDCWD, "/hdp/target/debug/build/sys-info-35d1c893f526b9bf/root-output", O_RDONLY|O_CLOEXEC) = 12
statx(12, "", AT_STATX_SYNC_AS_STAT|AT_EMPTY_PATH, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0644, stx_size=53, ...}) = 0
read(12, "/hdp/target/debug/build/sys-info"..., 53) = 53
read(12, "", 32)                        = 0
close(12)                               = 0
openat(AT_FDCWD, "/hdp/target/debug/build/sys-info-35d1c893f526b9bf/output", O_RDONLY|O_CLOEXEC) = 12
statx(12, "", AT_STATX_SYNC_AS_STAT|AT_EMPTY_PATH, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0644, stx_size=1693, ...}) = 0
read(12, "OUT_DIR = Some(/hdp/target/debug"..., 1693) = 1693
read(12, "", 32)                        = 0
close(12)                               = 0
statx(AT_FDCWD, "/hdp/target/debug/.fingerprint/hdp-6a2dbd9cb939e6d3", AT_STATX_SYNC_AS_STAT, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFDIR|0755, stx_size=4096, ...}) = 0
readlinkat(AT_FDCWD, "/proc/self/exe", "/root/.rustup/toolchains/stable-"..., 256) = 67
readlinkat(AT_FDCWD, "/root", 0xffffc61b88a0, 1023) = -1 EINVAL (Invalid argument)
readlinkat(AT_FDCWD, "/root/.rustup", 0xffffc61b88a0, 1023) = -1 EINVAL (Invalid argument)
readlinkat(AT_FDCWD, "/root/.rustup/toolchains", 0xffffc61b88a0, 1023) = -1 EINVAL (Invalid argument)
readlinkat(AT_FDCWD, "/root/.rustup/toolchains/stable-aarch64-unknown-linux-gnu", 0xffffc61b88a0, 1023) = -1 EINVAL (Invalid argument)
readlinkat(AT_FDCWD, "/root/.rustup/toolchains/stable-aarch64-unknown-linux-gnu/bin", 0xffffc61b88a0, 1023) = -1 EINVAL (Invalid argument)
readlinkat(AT_FDCWD, "/root/.rustup/toolchains/stable-aarch64-unknown-linux-gnu/bin/cargo", 0xffffc61b88a0, 1023) = -1 EINVAL (Invalid argument)
statx(AT_FDCWD, "/hdp/target/debug/build/libc-8943005962d500b5/build_script_build-8943005962d500b5", AT_STATX_SYNC_AS_STAT, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0755, stx_size=4283376, ...}) = 0
openat(AT_FDCWD, "/hdp/target/debug/.fingerprint/libc-8943005962d500b5/dep-build-script-build-script-build", O_RDONLY|O_CLOEXEC) = 12
statx(12, "", AT_STATX_SYNC_AS_STAT|AT_EMPTY_PATH, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0644, stx_size=14, ...}) = 0
read(12, "\1\0\0\0\377\1\0\0\0\0\0\0\0\0", 14) = 14
read(12, "", 32)                        = 0
close(12)                               = 0
statx(AT_FDCWD, "/hdp/target/debug/.fingerprint/libc-8943005962d500b5/dep-build-script-build-script-build", AT_STATX_SYNC_AS_STAT, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0644, stx_size=14, ...}) = 0
getcwd("/hdp", 512)                     = 5
statx(AT_FDCWD, "/hdp/target/debug/build/libc-18822760ea5a2116/output", AT_STATX_SYNC_AS_STAT, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0644, stx_size=915, ...}) = 0
statx(AT_FDCWD, "/hdp/target/debug/build/libc-18822760ea5a2116/output", AT_STATX_SYNC_AS_STAT, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0644, stx_size=915, ...}) = 0
getcwd("/hdp", 512)                     = 5
statx(AT_FDCWD, "/hdp/target/debug/deps/liblibc-d1dc81e31d292338.rlib", AT_STATX_SYNC_AS_STAT, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0644, stx_size=3419532, ...}) = 0
statx(AT_FDCWD, "/hdp/target/debug/deps/liblibc-d1dc81e31d292338.rmeta", AT_STATX_SYNC_AS_STAT, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0644, stx_size=3402206, ...}) = 0
openat(AT_FDCWD, "/hdp/target/debug/.fingerprint/libc-d1dc81e31d292338/dep-lib-libc", O_RDONLY|O_CLOEXEC) = 12
statx(12, "", AT_STATX_SYNC_AS_STAT|AT_EMPTY_PATH, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0644, stx_size=14, ...}) = 0
read(12, "\1\0\0\0\377\1\0\0\0\0\0\0\0\0", 14) = 14
read(12, "", 32)                        = 0
close(12)                               = 0
statx(AT_FDCWD, "/hdp/target/debug/.fingerprint/libc-d1dc81e31d292338/dep-lib-libc", AT_STATX_SYNC_AS_STAT, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0644, stx_size=14, ...}) = 0
getcwd("/hdp", 512)                     = 5
statx(AT_FDCWD, "/hdp/target/debug/deps/libatty-6ee81772ed9252b2.rlib", AT_STATX_SYNC_AS_STAT, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0644, stx_size=11528, ...}) = 0
statx(AT_FDCWD, "/hdp/target/debug/deps/libatty-6ee81772ed9252b2.rmeta", AT_STATX_SYNC_AS_STAT, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0644, stx_size=6176, ...}) = 0
openat(AT_FDCWD, "/hdp/target/debug/.fingerprint/atty-6ee81772ed9252b2/dep-lib-atty", O_RDONLY|O_CLOEXEC) = 12
statx(12, "", AT_STATX_SYNC_AS_STAT|AT_EMPTY_PATH, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0644, stx_size=14, ...}) = 0
read(12, "\1\0\0\0\377\1\0\0\0\0\0\0\0\0", 14) = 14
read(12, "", 32)                        = 0
close(12)                               = 0
statx(AT_FDCWD, "/hdp/target/debug/.fingerprint/atty-6ee81772ed9252b2/dep-lib-atty", AT_STATX_SYNC_AS_STAT, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0644, stx_size=14, ...}) = 0
getcwd("/hdp", 512)                     = 5
statx(AT_FDCWD, "/hdp/target/debug/deps/libsocket2-53cddd3167b9f708.rlib", AT_STATX_SYNC_AS_STAT, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0644, stx_size=1804338, ...}) = 0
statx(AT_FDCWD, "/hdp/target/debug/deps/libsocket2-53cddd3167b9f708.rmeta", AT_STATX_SYNC_AS_STAT, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0644, stx_size=422778, ...}) = 0
openat(AT_FDCWD, "/hdp/target/debug/.fingerprint/socket2-53cddd3167b9f708/dep-lib-socket2", O_RDONLY|O_CLOEXEC) = 12
statx(12, "", AT_STATX_SYNC_AS_STAT|AT_EMPTY_PATH, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0644, stx_size=14, ...}) = 0
read(12, "\1\0\0\0\377\1\0\0\0\0\0\0\0\0", 14) = 14
read(12, "", 32)                        = 0
close(12)                               = 0
statx(AT_FDCWD, "/hdp/target/debug/.fingerprint/socket2-53cddd3167b9f708/dep-lib-socket2", AT_STATX_SYNC_AS_STAT, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0644, stx_size=14, ...}) = 0
getcwd("/hdp", 512)                     = 5
statx(AT_FDCWD, "/hdp/target/debug/deps/libshlex-339df17943e93f39.rlib", AT_STATX_SYNC_AS_STAT, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0644, stx_size=148932, ...}) = 0
statx(AT_FDCWD, "/hdp/target/debug/deps/libshlex-339df17943e93f39.rmeta", AT_STATX_SYNC_AS_STAT, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0644, stx_size=53438, ...}) = 0
openat(AT_FDCWD, "/hdp/target/debug/.fingerprint/shlex-339df17943e93f39/dep-lib-shlex", O_RDONLY|O_CLOEXEC) = 12
statx(12, "", AT_STATX_SYNC_AS_STAT|AT_EMPTY_PATH, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0644, stx_size=14, ...}) = 0
read(12, "\1\0\0\0\377\1\0\0\0\0\0\0\0\0", 14) = 14
read(12, "", 32)                        = 0
close(12)                               = 0
statx(AT_FDCWD, "/hdp/target/debug/.fingerprint/shlex-339df17943e93f39/dep-lib-shlex", AT_STATX_SYNC_AS_STAT, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0644, stx_size=14, ...}) = 0
getcwd("/hdp", 512)                     = 5
statx(AT_FDCWD, "/hdp/target/debug/deps/libcc-c413f6aff6aeb93f.rlib", AT_STATX_SYNC_AS_STAT, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0644, stx_size=3223710, ...}) = 0
statx(AT_FDCWD, "/hdp/target/debug/deps/libcc-c413f6aff6aeb93f.rmeta", AT_STATX_SYNC_AS_STAT, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0644, stx_size=612406, ...}) = 0
openat(AT_FDCWD, "/hdp/target/debug/.fingerprint/cc-c413f6aff6aeb93f/dep-lib-cc", O_RDONLY|O_CLOEXEC) = 12
statx(12, "", AT_STATX_SYNC_AS_STAT|AT_EMPTY_PATH, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0644, stx_size=14, ...}) = 0
read(12, "\1\0\0\0\377\1\0\0\0\0\0\0\0\0", 14) = 14
read(12, "", 32)                        = 0
close(12)                               = 0
statx(AT_FDCWD, "/hdp/target/debug/.fingerprint/cc-c413f6aff6aeb93f/dep-lib-cc", AT_STATX_SYNC_AS_STAT, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0644, stx_size=14, ...}) = 0
getcwd("/hdp", 512)                     = 5
statx(AT_FDCWD, "/hdp/target/debug/build/sys-info-340b92157555863f/build_script_build-340b92157555863f", AT_STATX_SYNC_AS_STAT, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0755, stx_size=5304736, ...}) = 0
openat(AT_FDCWD, "/hdp/target/debug/.fingerprint/sys-info-340b92157555863f/dep-build-script-build-script-build", O_RDONLY|O_CLOEXEC) = 12
statx(12, "", AT_STATX_SYNC_AS_STAT|AT_EMPTY_PATH, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0644, stx_size=14, ...}) = 0
read(12, "\1\0\0\0\377\1\0\0\0\0\0\0\0\0", 14) = 14
read(12, "", 32)                        = 0
close(12)                               = 0
statx(AT_FDCWD, "/hdp/target/debug/.fingerprint/sys-info-340b92157555863f/dep-build-script-build-script-build", AT_STATX_SYNC_AS_STAT, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0644, stx_size=14, ...}) = 0
getcwd("/hdp", 512)                     = 5
statx(AT_FDCWD, "/hdp/target/debug/build/sys-info-35d1c893f526b9bf/output", AT_STATX_SYNC_AS_STAT, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0644, stx_size=1693, ...}) = 0
statx(AT_FDCWD, "/hdp/target/debug/deps/libsys_info-701376c3fbdf6b8e.rlib", AT_STATX_SYNC_AS_STAT, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0644, stx_size=1734144, ...}) = 0
statx(AT_FDCWD, "/hdp/target/debug/deps/libsys_info-701376c3fbdf6b8e.rmeta", AT_STATX_SYNC_AS_STAT, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0644, stx_size=56964, ...}) = 0
openat(AT_FDCWD, "/hdp/target/debug/.fingerprint/sys-info-701376c3fbdf6b8e/dep-lib-sys_info", O_RDONLY|O_CLOEXEC) = 12
statx(12, "", AT_STATX_SYNC_AS_STAT|AT_EMPTY_PATH, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0644, stx_size=14, ...}) = 0
read(12, "\1\0\0\0\377\1\0\0\0\0\0\0\0\0", 14) = 14
read(12, "", 32)                        = 0
close(12)                               = 0
statx(AT_FDCWD, "/hdp/target/debug/.fingerprint/sys-info-701376c3fbdf6b8e/dep-lib-sys_info", AT_STATX_SYNC_AS_STAT, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0644, stx_size=14, ...}) = 0
getcwd("/hdp", 512)                     = 5
statx(AT_FDCWD, "/hdp/target/debug/deps/libwhoami-d79cea1aebd9605a.rlib", AT_STATX_SYNC_AS_STAT, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0644, stx_size=801842, ...}) = 0
statx(AT_FDCWD, "/hdp/target/debug/deps/libwhoami-d79cea1aebd9605a.rmeta", AT_STATX_SYNC_AS_STAT, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0644, stx_size=151312, ...}) = 0
openat(AT_FDCWD, "/hdp/target/debug/.fingerprint/whoami-d79cea1aebd9605a/dep-lib-whoami", O_RDONLY|O_CLOEXEC) = 12
statx(12, "", AT_STATX_SYNC_AS_STAT|AT_EMPTY_PATH, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0644, stx_size=14, ...}) = 0
read(12, "\1\0\0\0\377\1\0\0\0\0\0\0\0\0", 14) = 14
read(12, "", 32)                        = 0
close(12)                               = 0
statx(AT_FDCWD, "/hdp/target/debug/.fingerprint/whoami-d79cea1aebd9605a/dep-lib-whoami", AT_STATX_SYNC_AS_STAT, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0644, stx_size=14, ...}) = 0
getcwd("/hdp", 512)                     = 5
statx(AT_FDCWD, "/hdp/target/debug/deps/client-6a2dbd9cb939e6d3", AT_STATX_SYNC_AS_STAT, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0755, stx_size=4720560, ...}) = 0
openat(AT_FDCWD, "/hdp/target/debug/.fingerprint/hdp-6a2dbd9cb939e6d3/dep-bin-client", O_RDONLY|O_CLOEXEC) = 12
statx(12, "", AT_STATX_SYNC_AS_STAT|AT_EMPTY_PATH, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0644, stx_size=38, ...}) = 0
read(12, "\1\0\0\0\377\1\1\0\0\0\0\22\0\0\0src/client/main.r"..., 38) = 38
read(12, "", 32)                        = 0
close(12)                               = 0
statx(AT_FDCWD, "/hdp/target/debug/.fingerprint/hdp-6a2dbd9cb939e6d3/dep-bin-client", AT_STATX_SYNC_AS_STAT, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0644, stx_size=38, ...}) = 0
getcwd("/hdp", 512)                     = 5
statx(AT_FDCWD, "/hdp/src/client/main.rs", AT_STATX_SYNC_AS_STAT, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0644, stx_size=3907, ...}) = 0
openat(AT_FDCWD, "/hdp/target/debug/.fingerprint/hdp-6a2dbd9cb939e6d3/bin-client", O_RDONLY|O_CLOEXEC) = 12
statx(12, "", AT_STATX_SYNC_AS_STAT|AT_EMPTY_PATH, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0644, stx_size=16, ...}) = 0
read(12, "09ac449dac44514e", 16)        = 16
read(12, "", 32)                        = 0
close(12)                               = 0
statx(AT_FDCWD, "/hdp/target/debug/.fingerprint/atty-6ee81772ed9252b2", AT_STATX_SYNC_AS_STAT, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFDIR|0755, stx_size=4096, ...}) = 0
openat(AT_FDCWD, "/hdp/target/debug/.fingerprint/atty-6ee81772ed9252b2/lib-atty", O_RDONLY|O_CLOEXEC) = 12
statx(12, "", AT_STATX_SYNC_AS_STAT|AT_EMPTY_PATH, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0644, stx_size=16, ...}) = 0
read(12, "13e6bb25b65372cb", 16)        = 16
read(12, "", 32)                        = 0
close(12)                               = 0
statx(AT_FDCWD, "/hdp/target/debug/.fingerprint/libc-d1dc81e31d292338", AT_STATX_SYNC_AS_STAT, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFDIR|0755, stx_size=4096, ...}) = 0
openat(AT_FDCWD, "/hdp/target/debug/.fingerprint/libc-d1dc81e31d292338/lib-libc", O_RDONLY|O_CLOEXEC) = 12
statx(12, "", AT_STATX_SYNC_AS_STAT|AT_EMPTY_PATH, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0644, stx_size=16, ...}) = 0
read(12, "a6f7706aecc1015a", 16)        = 16
read(12, "", 32)                        = 0
close(12)                               = 0
statx(AT_FDCWD, "/hdp/target/debug/.fingerprint/libc-18822760ea5a2116", AT_STATX_SYNC_AS_STAT, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFDIR|0755, stx_size=4096, ...}) = 0
statx(AT_FDCWD, "/root/.cargo/bin/rustdoc", AT_STATX_SYNC_AS_STAT, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0755, stx_size=15187976, ...}) = 0
statx(AT_FDCWD, "/root/.cargo/bin/rustup", AT_STATX_SYNC_AS_STAT, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0755, stx_size=15187976, ...}) = 0
statx(AT_FDCWD, "/root/.cargo/bin/rustdoc", AT_STATX_SYNC_AS_STAT, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0755, stx_size=15187976, ...}) = 0
statx(AT_FDCWD, "/root/.cargo/bin/rustup", AT_STATX_SYNC_AS_STAT, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0755, stx_size=15187976, ...}) = 0
getcwd("/hdp", 512)                     = 5
statx(AT_FDCWD, "/root/.rustup/toolchains/stable-aarch64-unknown-linux-gnu/bin/rustdoc", AT_STATX_SYNC_AS_STAT, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0755, stx_size=17089120, ...}) = 0
openat(AT_FDCWD, "/hdp/target/debug/build/libc-18822760ea5a2116/root-output", O_RDONLY|O_CLOEXEC) = 12
statx(12, "", AT_STATX_SYNC_AS_STAT|AT_EMPTY_PATH, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0644, stx_size=49, ...}) = 0
read(12, "/hdp/target/debug/build/libc-188"..., 49) = 49
read(12, "", 32)                        = 0
close(12)                               = 0
openat(AT_FDCWD, "/hdp/target/debug/build/libc-18822760ea5a2116/output", O_RDONLY|O_CLOEXEC) = 12
statx(12, "", AT_STATX_SYNC_AS_STAT|AT_EMPTY_PATH, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0644, stx_size=915, ...}) = 0
read(12, "cargo:rerun-if-changed=build.rs\n"..., 915) = 915
read(12, "", 32)                        = 0
close(12)                               = 0
mkdirat(AT_FDCWD, "/hdp/target/debug/build/libc-8943005962d500b5", 0777) = -1 EEXIST (File exists)
statx(AT_FDCWD, "/hdp/target/debug/build/libc-8943005962d500b5", AT_STATX_SYNC_AS_STAT, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFDIR|0755, stx_size=4096, ...}) = 0
mkdirat(AT_FDCWD, "/hdp/target/debug/build/libc-18822760ea5a2116/out", 0777) = -1 EEXIST (File exists)
statx(AT_FDCWD, "/hdp/target/debug/build/libc-18822760ea5a2116/out", AT_STATX_SYNC_AS_STAT, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFDIR|0755, stx_size=4096, ...}) = 0
openat(AT_FDCWD, "/hdp/target/debug/.fingerprint/libc-18822760ea5a2116/run-build-script-build-script-build", O_RDONLY|O_CLOEXEC) = 12
statx(12, "", AT_STATX_SYNC_AS_STAT|AT_EMPTY_PATH, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0644, stx_size=16, ...}) = 0
read(12, "6a820dfa2437d4c7", 16)        = 16
read(12, "", 32)                        = 0
close(12)                               = 0
statx(AT_FDCWD, "/hdp/target/debug/.fingerprint/libc-8943005962d500b5", AT_STATX_SYNC_AS_STAT, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFDIR|0755, stx_size=4096, ...}) = 0
openat(AT_FDCWD, "/hdp/target/debug/.fingerprint/libc-8943005962d500b5/build-script-build-script-build", O_RDONLY|O_CLOEXEC) = 12
statx(12, "", AT_STATX_SYNC_AS_STAT|AT_EMPTY_PATH, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0644, stx_size=16, ...}) = 0
read(12, "75266f56db43e168", 16)        = 16
read(12, "", 32)                        = 0
close(12)                               = 0
statx(AT_FDCWD, "/hdp/target/debug/.fingerprint/socket2-53cddd3167b9f708", AT_STATX_SYNC_AS_STAT, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFDIR|0755, stx_size=4096, ...}) = 0
openat(AT_FDCWD, "/hdp/target/debug/.fingerprint/socket2-53cddd3167b9f708/lib-socket2", O_RDONLY|O_CLOEXEC) = 12
statx(12, "", AT_STATX_SYNC_AS_STAT|AT_EMPTY_PATH, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0644, stx_size=16, ...}) = 0
read(12, "7d8dc2a08d4be80d", 16)        = 16
read(12, "", 32)                        = 0
close(12)                               = 0
statx(AT_FDCWD, "/hdp/target/debug/.fingerprint/sys-info-701376c3fbdf6b8e", AT_STATX_SYNC_AS_STAT, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFDIR|0755, stx_size=4096, ...}) = 0
openat(AT_FDCWD, "/hdp/target/debug/.fingerprint/sys-info-701376c3fbdf6b8e/lib-sys_info", O_RDONLY|O_CLOEXEC) = 12
statx(12, "", AT_STATX_SYNC_AS_STAT|AT_EMPTY_PATH, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0644, stx_size=16, ...}) = 0
read(12, "818de50d5ad5f52f", 16)        = 16
read(12, "", 32)                        = 0
close(12)                               = 0
statx(AT_FDCWD, "/hdp/target/debug/.fingerprint/sys-info-35d1c893f526b9bf", AT_STATX_SYNC_AS_STAT, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFDIR|0755, stx_size=4096, ...}) = 0
openat(AT_FDCWD, "/hdp/target/debug/build/sys-info-35d1c893f526b9bf/root-output", O_RDONLY|O_CLOEXEC) = 12
statx(12, "", AT_STATX_SYNC_AS_STAT|AT_EMPTY_PATH, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0644, stx_size=53, ...}) = 0
read(12, "/hdp/target/debug/build/sys-info"..., 53) = 53
read(12, "", 32)                        = 0
close(12)                               = 0
openat(AT_FDCWD, "/hdp/target/debug/build/sys-info-35d1c893f526b9bf/output", O_RDONLY|O_CLOEXEC) = 12
statx(12, "", AT_STATX_SYNC_AS_STAT|AT_EMPTY_PATH, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0644, stx_size=1693, ...}) = 0
read(12, "OUT_DIR = Some(/hdp/target/debug"..., 1693) = 1693
read(12, "", 32)                        = 0
close(12)                               = 0
mkdirat(AT_FDCWD, "/hdp/target/debug/build/sys-info-340b92157555863f", 0777) = -1 EEXIST (File exists)
statx(AT_FDCWD, "/hdp/target/debug/build/sys-info-340b92157555863f", AT_STATX_SYNC_AS_STAT, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFDIR|0755, stx_size=4096, ...}) = 0
mkdirat(AT_FDCWD, "/hdp/target/debug/build/sys-info-35d1c893f526b9bf/out", 0777) = -1 EEXIST (File exists)
statx(AT_FDCWD, "/hdp/target/debug/build/sys-info-35d1c893f526b9bf/out", AT_STATX_SYNC_AS_STAT, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFDIR|0755, stx_size=4096, ...}) = 0
openat(AT_FDCWD, "/hdp/target/debug/.fingerprint/sys-info-35d1c893f526b9bf/run-build-script-build-script-build", O_RDONLY|O_CLOEXEC) = 12
statx(12, "", AT_STATX_SYNC_AS_STAT|AT_EMPTY_PATH, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0644, stx_size=16, ...}) = 0
read(12, "240ec9cdb656247f", 16)        = 16
read(12, "", 32)                        = 0
close(12)                               = 0
statx(AT_FDCWD, "/hdp/target/debug/.fingerprint/sys-info-340b92157555863f", AT_STATX_SYNC_AS_STAT, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFDIR|0755, stx_size=4096, ...}) = 0
openat(AT_FDCWD, "/hdp/target/debug/.fingerprint/sys-info-340b92157555863f/build-script-build-script-build", O_RDONLY|O_CLOEXEC) = 12
statx(12, "", AT_STATX_SYNC_AS_STAT|AT_EMPTY_PATH, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0644, stx_size=16, ...}) = 0
read(12, "d660d1bb71149bb8", 16)        = 16
read(12, "", 32)                        = 0
close(12)                               = 0
statx(AT_FDCWD, "/hdp/target/debug/.fingerprint/cc-c413f6aff6aeb93f", AT_STATX_SYNC_AS_STAT, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFDIR|0755, stx_size=4096, ...}) = 0
openat(AT_FDCWD, "/hdp/target/debug/.fingerprint/cc-c413f6aff6aeb93f/lib-cc", O_RDONLY|O_CLOEXEC) = 12
statx(12, "", AT_STATX_SYNC_AS_STAT|AT_EMPTY_PATH, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0644, stx_size=16, ...}) = 0
read(12, "4549c8025dd25093", 16)        = 16
read(12, "", 32)                        = 0
close(12)                               = 0
statx(AT_FDCWD, "/hdp/target/debug/.fingerprint/shlex-339df17943e93f39", AT_STATX_SYNC_AS_STAT, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFDIR|0755, stx_size=4096, ...}) = 0
openat(AT_FDCWD, "/hdp/target/debug/.fingerprint/shlex-339df17943e93f39/lib-shlex", O_RDONLY|O_CLOEXEC) = 12
statx(12, "", AT_STATX_SYNC_AS_STAT|AT_EMPTY_PATH, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0644, stx_size=16, ...}) = 0
read(12, "666ebc61703d60b4", 16)        = 16
read(12, "", 32)                        = 0
close(12)                               = 0
statx(AT_FDCWD, "/hdp/target/debug/.fingerprint/whoami-d79cea1aebd9605a", AT_STATX_SYNC_AS_STAT, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFDIR|0755, stx_size=4096, ...}) = 0
openat(AT_FDCWD, "/hdp/target/debug/.fingerprint/whoami-d79cea1aebd9605a/lib-whoami", O_RDONLY|O_CLOEXEC) = 12
statx(12, "", AT_STATX_SYNC_AS_STAT|AT_EMPTY_PATH, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0644, stx_size=16, ...}) = 0
read(12, "175d5cc942d45ed0", 16)        = 16
read(12, "", 32)                        = 0
close(12)                               = 0
rt_sigaction(SIGUSR1, {sa_handler=0xaaaab529c89c, sa_mask=[], sa_flags=SA_SIGINFO}, NULL, 8) = 0
rt_sigaction(SIGRT_1, {sa_handler=0xffffa8e5c0a0, sa_mask=[], sa_flags=SA_ONSTACK|SA_RESTART|SA_SIGINFO}, NULL, 8) = 0
rt_sigprocmask(SIG_UNBLOCK, [RTMIN RT_1], NULL, 8) = 0
mmap(NULL, 2162688, PROT_NONE, MAP_PRIVATE|MAP_ANONYMOUS|MAP_STACK, -1, 0) = 0xffffa8600000
mprotect(0xffffa8610000, 2097152, PROT_READ|PROT_WRITE) = 0
rt_sigprocmask(SIG_BLOCK, ~[], [], 8)   = 0
clone(child_stack=0xffffa880e7c0, flags=CLONE_VM|CLONE_FS|CLONE_FILES|CLONE_SIGHAND|CLONE_THREAD|CLONE_SYSVSEM|CLONE_SETTLS|CLONE_PARENT_SETTID|CLONE_CHILD_CLEARTID, parent_tid=[750], tls=0xffffa880f640, child_tidptr=0xffffa880efd0) = 750
rt_sigprocmask(SIG_SETMASK, [], NULL, 8) = 0
futex(0xaaaaeacede18, FUTEX_WAKE_PRIVATE, 1) = 1
futex(0xaaaaeacede18, FUTEX_WAKE_PRIVATE, 1) = 1
statx(AT_FDCWD, "/hdp/target/debug/.fingerprint/libc-8943005962d500b5/output-build-script-build-script-build", AT_STATX_SYNC_AS_STAT, STATX_ALL, 0xffffc61ba150) = -1 ENOENT (No such file or directory)
statx(AT_FDCWD, "/hdp/target/debug/build/libc-8943005962d500b5/build_script_build-8943005962d500b5", AT_STATX_SYNC_AS_STAT, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0755, stx_size=4283376, ...}) = 0
openat(AT_FDCWD, "/hdp/target/debug/build/libc-8943005962d500b5/build_script_build-8943005962d500b5", O_RDONLY|O_CLOEXEC) = 12
statx(12, "", AT_STATX_SYNC_AS_STAT|AT_EMPTY_PATH, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0755, stx_size=4283376, ...}) = 0
openat(AT_FDCWD, "/hdp/target/debug/build/libc-8943005962d500b5/build-script-build", O_RDONLY|O_CLOEXEC) = 13
statx(13, "", AT_STATX_SYNC_AS_STAT|AT_EMPTY_PATH, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0755, stx_size=4283376, ...}) = 0
close(13)                               = 0
close(12)                               = 0
statx(AT_FDCWD, "/hdp/target/debug/build/libc-8943005962d500b5/build_script_build-8943005962d500b5.dwp", AT_STATX_SYNC_AS_STAT, STATX_ALL, 0xffffc61b9f80) = -1 ENOENT (No such file or directory)
futex(0xaaaaeac32d90, FUTEX_WAKE_PRIVATE, 1) = 0
futex(0xaaaaeac32d94, FUTEX_WAKE_PRIVATE, 2147483647) = 0
futex(0xaaaaeacede18, FUTEX_WAKE_PRIVATE, 1) = 1
futex(0xaaaaeac32d90, FUTEX_WAKE_PRIVATE, 1) = 0
statx(AT_FDCWD, "/hdp/target/debug/.fingerprint/shlex-339df17943e93f39/output-lib-shlex", AT_STATX_SYNC_AS_STAT, STATX_ALL, 0xffffc61ba150) = -1 ENOENT (No such file or directory)
statx(AT_FDCWD, "/hdp/target/debug/deps/libshlex-339df17943e93f39.rlib", AT_STATX_SYNC_AS_STAT, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0644, stx_size=148932, ...}) = 0
statx(AT_FDCWD, "/hdp/target/debug/deps/libshlex-339df17943e93f39.rmeta", AT_STATX_SYNC_AS_STAT, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0644, stx_size=53438, ...}) = 0
futex(0xaaaaeac32d90, FUTEX_WAKE_PRIVATE, 1) = 0
futex(0xaaaaeac32d90, FUTEX_WAKE_PRIVATE, 1) = 0
statx(AT_FDCWD, "/hdp/target/debug/.fingerprint/whoami-d79cea1aebd9605a/output-lib-whoami", AT_STATX_SYNC_AS_STAT, STATX_ALL, 0xffffc61ba150) = -1 ENOENT (No such file or directory)
statx(AT_FDCWD, "/hdp/target/debug/deps/libwhoami-d79cea1aebd9605a.rlib", AT_STATX_SYNC_AS_STAT, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0644, stx_size=801842, ...}) = 0
statx(AT_FDCWD, "/hdp/target/debug/deps/libwhoami-d79cea1aebd9605a.rmeta", AT_STATX_SYNC_AS_STAT, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0644, stx_size=151312, ...}) = 0
futex(0xaaaaeac32d90, FUTEX_WAKE_PRIVATE, 1) = 0
futex(0xaaaaeac32d94, FUTEX_WAKE_PRIVATE, 2147483647) = 0
futex(0xaaaaeacede18, FUTEX_WAKE_PRIVATE, 1) = 1
statx(AT_FDCWD, "/hdp/target/debug/.fingerprint/libc-d1dc81e31d292338/output-lib-libc", AT_STATX_SYNC_AS_STAT, STATX_ALL, 0xffffc61ba150) = -1 ENOENT (No such file or directory)
statx(AT_FDCWD, "/hdp/target/debug/deps/liblibc-d1dc81e31d292338.rlib", AT_STATX_SYNC_AS_STAT, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0644, stx_size=3419532, ...}) = 0
statx(AT_FDCWD, "/hdp/target/debug/deps/liblibc-d1dc81e31d292338.rmeta", AT_STATX_SYNC_AS_STAT, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0644, stx_size=3402206, ...}) = 0
futex(0xaaaaeac32d90, FUTEX_WAKE_PRIVATE, 1) = 0
futex(0xaaaaeac32d90, FUTEX_WAKE_PRIVATE, 1) = 0
statx(AT_FDCWD, "/hdp/target/debug/.fingerprint/cc-c413f6aff6aeb93f/output-lib-cc", AT_STATX_SYNC_AS_STAT, STATX_ALL, 0xffffc61ba150) = -1 ENOENT (No such file or directory)
statx(AT_FDCWD, "/hdp/target/debug/deps/libcc-c413f6aff6aeb93f.rlib", AT_STATX_SYNC_AS_STAT, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0644, stx_size=3223710, ...}) = 0
statx(AT_FDCWD, "/hdp/target/debug/deps/libcc-c413f6aff6aeb93f.rmeta", AT_STATX_SYNC_AS_STAT, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0644, stx_size=612406, ...}) = 0
futex(0xaaaaeac32d90, FUTEX_WAKE_PRIVATE, 1) = 0
futex(0xaaaaeac32d94, FUTEX_WAKE_PRIVATE, 2147483647) = 0
futex(0xaaaaeacede18, FUTEX_WAKE_PRIVATE, 1) = 1
futex(0xaaaaeacede18, FUTEX_WAKE_PRIVATE, 1) = 1
statx(AT_FDCWD, "/hdp/target/debug/.fingerprint/sys-info-340b92157555863f/output-build-script-build-script-build", AT_STATX_SYNC_AS_STAT, STATX_ALL, 0xffffc61ba150) = -1 ENOENT (No such file or directory)
statx(AT_FDCWD, "/hdp/target/debug/build/sys-info-340b92157555863f/build_script_build-340b92157555863f", AT_STATX_SYNC_AS_STAT, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0755, stx_size=5304736, ...}) = 0
openat(AT_FDCWD, "/hdp/target/debug/build/sys-info-340b92157555863f/build_script_build-340b92157555863f", O_RDONLY|O_CLOEXEC) = 12
statx(12, "", AT_STATX_SYNC_AS_STAT|AT_EMPTY_PATH, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0755, stx_size=5304736, ...}) = 0
openat(AT_FDCWD, "/hdp/target/debug/build/sys-info-340b92157555863f/build-script-build", O_RDONLY|O_CLOEXEC) = 13
statx(13, "", AT_STATX_SYNC_AS_STAT|AT_EMPTY_PATH, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0755, stx_size=5304736, ...}) = 0
close(13)                               = 0
close(12)                               = 0
statx(AT_FDCWD, "/hdp/target/debug/build/sys-info-340b92157555863f/build_script_build-340b92157555863f.dwp", AT_STATX_SYNC_AS_STAT, STATX_ALL, 0xffffc61b9f80) = -1 ENOENT (No such file or directory)
futex(0xaaaaeac32d90, FUTEX_WAKE_PRIVATE, 1) = 0
statx(AT_FDCWD, "/hdp/target/debug/.fingerprint/atty-6ee81772ed9252b2/output-lib-atty", AT_STATX_SYNC_AS_STAT, STATX_ALL, 0xffffc61ba150) = -1 ENOENT (No such file or directory)
statx(AT_FDCWD, "/hdp/target/debug/deps/libatty-6ee81772ed9252b2.rlib", AT_STATX_SYNC_AS_STAT, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0644, stx_size=11528, ...}) = 0
statx(AT_FDCWD, "/hdp/target/debug/deps/libatty-6ee81772ed9252b2.rmeta", AT_STATX_SYNC_AS_STAT, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0644, stx_size=6176, ...}) = 0
futex(0xaaaaeac32d90, FUTEX_WAKE_PRIVATE, 1) = 0
statx(AT_FDCWD, "/hdp/target/debug/.fingerprint/socket2-53cddd3167b9f708/output-lib-socket2", AT_STATX_SYNC_AS_STAT, STATX_ALL, 0xffffc61ba150) = -1 ENOENT (No such file or directory)
statx(AT_FDCWD, "/hdp/target/debug/deps/libsocket2-53cddd3167b9f708.rlib", AT_STATX_SYNC_AS_STAT, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0644, stx_size=1804338, ...}) = 0
statx(AT_FDCWD, "/hdp/target/debug/deps/libsocket2-53cddd3167b9f708.rmeta", AT_STATX_SYNC_AS_STAT, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0644, stx_size=422778, ...}) = 0
futex(0xaaaaeac32d90, FUTEX_WAKE_PRIVATE, 1) = 0
futex(0xaaaaeac32d94, FUTEX_WAKE_PRIVATE, 2147483647) = 0
futex(0xaaaaeac32d90, FUTEX_WAKE_PRIVATE, 1) = 0
futex(0xaaaaeac32d94, FUTEX_WAKE_PRIVATE, 2147483647) = 0
statx(AT_FDCWD, "/hdp/target/debug/.fingerprint/sys-info-701376c3fbdf6b8e/output-lib-sys_info", AT_STATX_SYNC_AS_STAT, STATX_ALL, 0xffffc61ba150) = -1 ENOENT (No such file or directory)
statx(AT_FDCWD, "/hdp/target/debug/deps/libsys_info-701376c3fbdf6b8e.rlib", AT_STATX_SYNC_AS_STAT, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0644, stx_size=1734144, ...}) = 0
statx(AT_FDCWD, "/hdp/target/debug/deps/libsys_info-701376c3fbdf6b8e.rmeta", AT_STATX_SYNC_AS_STAT, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0644, stx_size=56964, ...}) = 0
futex(0xaaaaeac32d90, FUTEX_WAKE_PRIVATE, 1) = 0
futex(0xaaaaeac32d94, FUTEX_WAKE_PRIVATE, 2147483647) = 0
statx(AT_FDCWD, "/hdp/target/debug/.fingerprint/hdp-6a2dbd9cb939e6d3/output-bin-client", AT_STATX_SYNC_AS_STAT, STATX_ALL, 0xffffc61ba150) = -1 ENOENT (No such file or directory)
statx(AT_FDCWD, "/hdp/target/debug/deps/client-6a2dbd9cb939e6d3", AT_STATX_SYNC_AS_STAT, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0755, stx_size=4720560, ...}) = 0
openat(AT_FDCWD, "/hdp/target/debug/deps/client-6a2dbd9cb939e6d3", O_RDONLY|O_CLOEXEC) = 12
statx(12, "", AT_STATX_SYNC_AS_STAT|AT_EMPTY_PATH, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0755, stx_size=4720560, ...}) = 0
openat(AT_FDCWD, "/hdp/target/debug/client", O_RDONLY|O_CLOEXEC) = 13
statx(13, "", AT_STATX_SYNC_AS_STAT|AT_EMPTY_PATH, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0755, stx_size=4720560, ...}) = 0
close(13)                               = 0
close(12)                               = 0
statx(AT_FDCWD, "/hdp/target/debug/deps/client-6a2dbd9cb939e6d3.dwp", AT_STATX_SYNC_AS_STAT, STATX_ALL, 0xffffc61b9f80) = -1 ENOENT (No such file or directory)
futex(0xaaaaeac32d90, FUTEX_WAKE_PRIVATE, 1) = 0
futex(0xaaaaeac32d94, FUTEX_WAKE_PRIVATE, 2147483647) = 0
write(2, "    Finished", 12    Finished)            = 12
write(2, " `dev` profile [unoptimized + de"..., 60 `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
) = 60
write(9, "|", 1)                        = 1
write(9, "|", 1)                        = 1
write(9, "|", 1)                        = 1
write(9, "|", 1)                        = 1
write(9, "|", 1)                        = 1
write(9, "|", 1)                        = 1
futex(0xaaaaeacede18, FUTEX_WAKE_PRIVATE, 1) = 1
openat(AT_FDCWD, "/hdp/target/debug/.fingerprint/hdp-6a2dbd9cb939e6d3/dep-bin-client", O_RDONLY|O_CLOEXEC) = 12
statx(12, "", AT_STATX_SYNC_AS_STAT|AT_EMPTY_PATH, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0644, stx_size=38, ...}) = 0
read(12, "\1\0\0\0\377\1\1\0\0\0\0\22\0\0\0src/client/main.r"..., 38) = 38
read(12, "", 32)                        = 0
close(12)                               = 0
openat(AT_FDCWD, "/hdp/target/debug/client.d", O_RDONLY|O_CLOEXEC) = 12
statx(12, "", AT_STATX_SYNC_AS_STAT|AT_EMPTY_PATH, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0644, stx_size=50, ...}) = 0
read(12, "/hdp/target/debug/client: /hdp/s"..., 50) = 50
read(12, "", 32)                        = 0
close(12)                               = 0
flock(10, LOCK_UN)                      = 0
close(10)                               = 0
close(8)                                = 0
close(9)                                = 0
flock(11, LOCK_UN)                      = 0
close(11)                               = 0
close(4)                                = 0
close(5)                                = 0
munmap(0xffffa8d79000, 421888)          = 0
munmap(0xffffa8cec000, 577536)          = 0
munmap(0xffffa8a00000, 3039232)         = 0
close(3)                                = 0
close(7)                                = 0
write(2, "     Running", 12     Running)            = 12
write(2, " `target/debug/client 127.0.0.1`"..., 33 `target/debug/client 127.0.0.1`
) = 33
chdir("/hdp")                           = 0
rt_sigaction(SIGPIPE, {sa_handler=SIG_DFL, sa_mask=[PIPE], sa_flags=SA_RESTART}, {sa_handler=SIG_IGN, sa_mask=[PIPE], sa_flags=SA_RESTART}, 8) = 0
execve("target/debug/client", ["target/debug/client", "127.0.0.1"], 0xaaaaeadcdca0 /* 32 vars */) = 0
brk(NULL)                               = 0xaaaaf8356000
mmap(NULL, 8192, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_ANONYMOUS, -1, 0) = 0xffff89f40000
faccessat(AT_FDCWD, "/etc/ld.so.preload", R_OK) = -1 ENOENT (No such file or directory)
openat(AT_FDCWD, "/hdp/target/debug/build/sys-info-35d1c893f526b9bf/out/tls/aarch64/atomics/libgcc_s.so.1", O_RDONLY|O_CLOEXEC) = -1 ENOENT (No such file or directory)
newfstatat(AT_FDCWD, "/hdp/target/debug/build/sys-info-35d1c893f526b9bf/out/tls/aarch64/atomics", 0xfffff05ee570, 0) = -1 ENOENT (No such file or directory)
openat(AT_FDCWD, "/hdp/target/debug/build/sys-info-35d1c893f526b9bf/out/tls/aarch64/libgcc_s.so.1", O_RDONLY|O_CLOEXEC) = -1 ENOENT (No such file or directory)
newfstatat(AT_FDCWD, "/hdp/target/debug/build/sys-info-35d1c893f526b9bf/out/tls/aarch64", 0xfffff05ee570, 0) = -1 ENOENT (No such file or directory)
openat(AT_FDCWD, "/hdp/target/debug/build/sys-info-35d1c893f526b9bf/out/tls/atomics/libgcc_s.so.1", O_RDONLY|O_CLOEXEC) = -1 ENOENT (No such file or directory)
newfstatat(AT_FDCWD, "/hdp/target/debug/build/sys-info-35d1c893f526b9bf/out/tls/atomics", 0xfffff05ee570, 0) = -1 ENOENT (No such file or directory)
openat(AT_FDCWD, "/hdp/target/debug/build/sys-info-35d1c893f526b9bf/out/tls/libgcc_s.so.1", O_RDONLY|O_CLOEXEC) = -1 ENOENT (No such file or directory)
newfstatat(AT_FDCWD, "/hdp/target/debug/build/sys-info-35d1c893f526b9bf/out/tls", 0xfffff05ee570, 0) = -1 ENOENT (No such file or directory)
openat(AT_FDCWD, "/hdp/target/debug/build/sys-info-35d1c893f526b9bf/out/aarch64/atomics/libgcc_s.so.1", O_RDONLY|O_CLOEXEC) = -1 ENOENT (No such file or directory)
newfstatat(AT_FDCWD, "/hdp/target/debug/build/sys-info-35d1c893f526b9bf/out/aarch64/atomics", 0xfffff05ee570, 0) = -1 ENOENT (No such file or directory)
openat(AT_FDCWD, "/hdp/target/debug/build/sys-info-35d1c893f526b9bf/out/aarch64/libgcc_s.so.1", O_RDONLY|O_CLOEXEC) = -1 ENOENT (No such file or directory)
newfstatat(AT_FDCWD, "/hdp/target/debug/build/sys-info-35d1c893f526b9bf/out/aarch64", 0xfffff05ee570, 0) = -1 ENOENT (No such file or directory)
openat(AT_FDCWD, "/hdp/target/debug/build/sys-info-35d1c893f526b9bf/out/atomics/libgcc_s.so.1", O_RDONLY|O_CLOEXEC) = -1 ENOENT (No such file or directory)
newfstatat(AT_FDCWD, "/hdp/target/debug/build/sys-info-35d1c893f526b9bf/out/atomics", 0xfffff05ee570, 0) = -1 ENOENT (No such file or directory)
openat(AT_FDCWD, "/hdp/target/debug/build/sys-info-35d1c893f526b9bf/out/libgcc_s.so.1", O_RDONLY|O_CLOEXEC) = -1 ENOENT (No such file or directory)
newfstatat(AT_FDCWD, "/hdp/target/debug/build/sys-info-35d1c893f526b9bf/out", {st_mode=S_IFDIR|0755, st_size=4096, ...}, 0) = 0
openat(AT_FDCWD, "/hdp/target/debug/deps/tls/aarch64/atomics/libgcc_s.so.1", O_RDONLY|O_CLOEXEC) = -1 ENOENT (No such file or directory)
newfstatat(AT_FDCWD, "/hdp/target/debug/deps/tls/aarch64/atomics", 0xfffff05ee570, 0) = -1 ENOENT (No such file or directory)
openat(AT_FDCWD, "/hdp/target/debug/deps/tls/aarch64/libgcc_s.so.1", O_RDONLY|O_CLOEXEC) = -1 ENOENT (No such file or directory)
newfstatat(AT_FDCWD, "/hdp/target/debug/deps/tls/aarch64", 0xfffff05ee570, 0) = -1 ENOENT (No such file or directory)
openat(AT_FDCWD, "/hdp/target/debug/deps/tls/atomics/libgcc_s.so.1", O_RDONLY|O_CLOEXEC) = -1 ENOENT (No such file or directory)
newfstatat(AT_FDCWD, "/hdp/target/debug/deps/tls/atomics", 0xfffff05ee570, 0) = -1 ENOENT (No such file or directory)
openat(AT_FDCWD, "/hdp/target/debug/deps/tls/libgcc_s.so.1", O_RDONLY|O_CLOEXEC) = -1 ENOENT (No such file or directory)
newfstatat(AT_FDCWD, "/hdp/target/debug/deps/tls", 0xfffff05ee570, 0) = -1 ENOENT (No such file or directory)
openat(AT_FDCWD, "/hdp/target/debug/deps/aarch64/atomics/libgcc_s.so.1", O_RDONLY|O_CLOEXEC) = -1 ENOENT (No such file or directory)
newfstatat(AT_FDCWD, "/hdp/target/debug/deps/aarch64/atomics", 0xfffff05ee570, 0) = -1 ENOENT (No such file or directory)
openat(AT_FDCWD, "/hdp/target/debug/deps/aarch64/libgcc_s.so.1", O_RDONLY|O_CLOEXEC) = -1 ENOENT (No such file or directory)
newfstatat(AT_FDCWD, "/hdp/target/debug/deps/aarch64", 0xfffff05ee570, 0) = -1 ENOENT (No such file or directory)
openat(AT_FDCWD, "/hdp/target/debug/deps/atomics/libgcc_s.so.1", O_RDONLY|O_CLOEXEC) = -1 ENOENT (No such file or directory)
newfstatat(AT_FDCWD, "/hdp/target/debug/deps/atomics", 0xfffff05ee570, 0) = -1 ENOENT (No such file or directory)
openat(AT_FDCWD, "/hdp/target/debug/deps/libgcc_s.so.1", O_RDONLY|O_CLOEXEC) = -1 ENOENT (No such file or directory)
newfstatat(AT_FDCWD, "/hdp/target/debug/deps", {st_mode=S_IFDIR|0755, st_size=40960, ...}, 0) = 0
openat(AT_FDCWD, "/hdp/target/debug/tls/aarch64/atomics/libgcc_s.so.1", O_RDONLY|O_CLOEXEC) = -1 ENOENT (No such file or directory)
newfstatat(AT_FDCWD, "/hdp/target/debug/tls/aarch64/atomics", 0xfffff05ee570, 0) = -1 ENOENT (No such file or directory)
openat(AT_FDCWD, "/hdp/target/debug/tls/aarch64/libgcc_s.so.1", O_RDONLY|O_CLOEXEC) = -1 ENOENT (No such file or directory)
newfstatat(AT_FDCWD, "/hdp/target/debug/tls/aarch64", 0xfffff05ee570, 0) = -1 ENOENT (No such file or directory)
openat(AT_FDCWD, "/hdp/target/debug/tls/atomics/libgcc_s.so.1", O_RDONLY|O_CLOEXEC) = -1 ENOENT (No such file or directory)
newfstatat(AT_FDCWD, "/hdp/target/debug/tls/atomics", 0xfffff05ee570, 0) = -1 ENOENT (No such file or directory)
openat(AT_FDCWD, "/hdp/target/debug/tls/libgcc_s.so.1", O_RDONLY|O_CLOEXEC) = -1 ENOENT (No such file or directory)
newfstatat(AT_FDCWD, "/hdp/target/debug/tls", 0xfffff05ee570, 0) = -1 ENOENT (No such file or directory)
openat(AT_FDCWD, "/hdp/target/debug/aarch64/atomics/libgcc_s.so.1", O_RDONLY|O_CLOEXEC) = -1 ENOENT (No such file or directory)
newfstatat(AT_FDCWD, "/hdp/target/debug/aarch64/atomics", 0xfffff05ee570, 0) = -1 ENOENT (No such file or directory)
openat(AT_FDCWD, "/hdp/target/debug/aarch64/libgcc_s.so.1", O_RDONLY|O_CLOEXEC) = -1 ENOENT (No such file or directory)
newfstatat(AT_FDCWD, "/hdp/target/debug/aarch64", 0xfffff05ee570, 0) = -1 ENOENT (No such file or directory)
openat(AT_FDCWD, "/hdp/target/debug/atomics/libgcc_s.so.1", O_RDONLY|O_CLOEXEC) = -1 ENOENT (No such file or directory)
newfstatat(AT_FDCWD, "/hdp/target/debug/atomics", 0xfffff05ee570, 0) = -1 ENOENT (No such file or directory)
openat(AT_FDCWD, "/hdp/target/debug/libgcc_s.so.1", O_RDONLY|O_CLOEXEC) = -1 ENOENT (No such file or directory)
newfstatat(AT_FDCWD, "/hdp/target/debug", {st_mode=S_IFDIR|0755, st_size=4096, ...}, 0) = 0
openat(AT_FDCWD, "/root/.rustup/toolchains/stable-aarch64-unknown-linux-gnu/lib/rustlib/aarch64-unknown-linux-gnu/lib/tls/aarch64/atomics/libgcc_s.so.1", O_RDONLY|O_CLOEXEC) = -1 ENOENT (No such file or directory)
newfstatat(AT_FDCWD, "/root/.rustup/toolchains/stable-aarch64-unknown-linux-gnu/lib/rustlib/aarch64-unknown-linux-gnu/lib/tls/aarch64/atomics", 0xfffff05ee570, 0) = -1 ENOENT (No such file or directory)
openat(AT_FDCWD, "/root/.rustup/toolchains/stable-aarch64-unknown-linux-gnu/lib/rustlib/aarch64-unknown-linux-gnu/lib/tls/aarch64/libgcc_s.so.1", O_RDONLY|O_CLOEXEC) = -1 ENOENT (No such file or directory)
newfstatat(AT_FDCWD, "/root/.rustup/toolchains/stable-aarch64-unknown-linux-gnu/lib/rustlib/aarch64-unknown-linux-gnu/lib/tls/aarch64", 0xfffff05ee570, 0) = -1 ENOENT (No such file or directory)
openat(AT_FDCWD, "/root/.rustup/toolchains/stable-aarch64-unknown-linux-gnu/lib/rustlib/aarch64-unknown-linux-gnu/lib/tls/atomics/libgcc_s.so.1", O_RDONLY|O_CLOEXEC) = -1 ENOENT (No such file or directory)
newfstatat(AT_FDCWD, "/root/.rustup/toolchains/stable-aarch64-unknown-linux-gnu/lib/rustlib/aarch64-unknown-linux-gnu/lib/tls/atomics", 0xfffff05ee570, 0) = -1 ENOENT (No such file or directory)
openat(AT_FDCWD, "/root/.rustup/toolchains/stable-aarch64-unknown-linux-gnu/lib/rustlib/aarch64-unknown-linux-gnu/lib/tls/libgcc_s.so.1", O_RDONLY|O_CLOEXEC) = -1 ENOENT (No such file or directory)
newfstatat(AT_FDCWD, "/root/.rustup/toolchains/stable-aarch64-unknown-linux-gnu/lib/rustlib/aarch64-unknown-linux-gnu/lib/tls", 0xfffff05ee570, 0) = -1 ENOENT (No such file or directory)
openat(AT_FDCWD, "/root/.rustup/toolchains/stable-aarch64-unknown-linux-gnu/lib/rustlib/aarch64-unknown-linux-gnu/lib/aarch64/atomics/libgcc_s.so.1", O_RDONLY|O_CLOEXEC) = -1 ENOENT (No such file or directory)
newfstatat(AT_FDCWD, "/root/.rustup/toolchains/stable-aarch64-unknown-linux-gnu/lib/rustlib/aarch64-unknown-linux-gnu/lib/aarch64/atomics", 0xfffff05ee570, 0) = -1 ENOENT (No such file or directory)
openat(AT_FDCWD, "/root/.rustup/toolchains/stable-aarch64-unknown-linux-gnu/lib/rustlib/aarch64-unknown-linux-gnu/lib/aarch64/libgcc_s.so.1", O_RDONLY|O_CLOEXEC) = -1 ENOENT (No such file or directory)
newfstatat(AT_FDCWD, "/root/.rustup/toolchains/stable-aarch64-unknown-linux-gnu/lib/rustlib/aarch64-unknown-linux-gnu/lib/aarch64", 0xfffff05ee570, 0) = -1 ENOENT (No such file or directory)
openat(AT_FDCWD, "/root/.rustup/toolchains/stable-aarch64-unknown-linux-gnu/lib/rustlib/aarch64-unknown-linux-gnu/lib/atomics/libgcc_s.so.1", O_RDONLY|O_CLOEXEC) = -1 ENOENT (No such file or directory)
newfstatat(AT_FDCWD, "/root/.rustup/toolchains/stable-aarch64-unknown-linux-gnu/lib/rustlib/aarch64-unknown-linux-gnu/lib/atomics", 0xfffff05ee570, 0) = -1 ENOENT (No such file or directory)
openat(AT_FDCWD, "/root/.rustup/toolchains/stable-aarch64-unknown-linux-gnu/lib/rustlib/aarch64-unknown-linux-gnu/lib/libgcc_s.so.1", O_RDONLY|O_CLOEXEC) = -1 ENOENT (No such file or directory)
newfstatat(AT_FDCWD, "/root/.rustup/toolchains/stable-aarch64-unknown-linux-gnu/lib/rustlib/aarch64-unknown-linux-gnu/lib", {st_mode=S_IFDIR|0755, st_size=4096, ...}, 0) = 0
openat(AT_FDCWD, "/root/.rustup/toolchains/stable-aarch64-unknown-linux-gnu/lib/tls/aarch64/atomics/libgcc_s.so.1", O_RDONLY|O_CLOEXEC) = -1 ENOENT (No such file or directory)
newfstatat(AT_FDCWD, "/root/.rustup/toolchains/stable-aarch64-unknown-linux-gnu/lib/tls/aarch64/atomics", 0xfffff05ee570, 0) = -1 ENOENT (No such file or directory)
openat(AT_FDCWD, "/root/.rustup/toolchains/stable-aarch64-unknown-linux-gnu/lib/tls/aarch64/libgcc_s.so.1", O_RDONLY|O_CLOEXEC) = -1 ENOENT (No such file or directory)
newfstatat(AT_FDCWD, "/root/.rustup/toolchains/stable-aarch64-unknown-linux-gnu/lib/tls/aarch64", 0xfffff05ee570, 0) = -1 ENOENT (No such file or directory)
openat(AT_FDCWD, "/root/.rustup/toolchains/stable-aarch64-unknown-linux-gnu/lib/tls/atomics/libgcc_s.so.1", O_RDONLY|O_CLOEXEC) = -1 ENOENT (No such file or directory)
newfstatat(AT_FDCWD, "/root/.rustup/toolchains/stable-aarch64-unknown-linux-gnu/lib/tls/atomics", 0xfffff05ee570, 0) = -1 ENOENT (No such file or directory)
openat(AT_FDCWD, "/root/.rustup/toolchains/stable-aarch64-unknown-linux-gnu/lib/tls/libgcc_s.so.1", O_RDONLY|O_CLOEXEC) = -1 ENOENT (No such file or directory)
newfstatat(AT_FDCWD, "/root/.rustup/toolchains/stable-aarch64-unknown-linux-gnu/lib/tls", 0xfffff05ee570, 0) = -1 ENOENT (No such file or directory)
openat(AT_FDCWD, "/root/.rustup/toolchains/stable-aarch64-unknown-linux-gnu/lib/aarch64/atomics/libgcc_s.so.1", O_RDONLY|O_CLOEXEC) = -1 ENOENT (No such file or directory)
newfstatat(AT_FDCWD, "/root/.rustup/toolchains/stable-aarch64-unknown-linux-gnu/lib/aarch64/atomics", 0xfffff05ee570, 0) = -1 ENOENT (No such file or directory)
openat(AT_FDCWD, "/root/.rustup/toolchains/stable-aarch64-unknown-linux-gnu/lib/aarch64/libgcc_s.so.1", O_RDONLY|O_CLOEXEC) = -1 ENOENT (No such file or directory)
newfstatat(AT_FDCWD, "/root/.rustup/toolchains/stable-aarch64-unknown-linux-gnu/lib/aarch64", 0xfffff05ee570, 0) = -1 ENOENT (No such file or directory)
openat(AT_FDCWD, "/root/.rustup/toolchains/stable-aarch64-unknown-linux-gnu/lib/atomics/libgcc_s.so.1", O_RDONLY|O_CLOEXEC) = -1 ENOENT (No such file or directory)
newfstatat(AT_FDCWD, "/root/.rustup/toolchains/stable-aarch64-unknown-linux-gnu/lib/atomics", 0xfffff05ee570, 0) = -1 ENOENT (No such file or directory)
openat(AT_FDCWD, "/root/.rustup/toolchains/stable-aarch64-unknown-linux-gnu/lib/libgcc_s.so.1", O_RDONLY|O_CLOEXEC) = -1 ENOENT (No such file or directory)
newfstatat(AT_FDCWD, "/root/.rustup/toolchains/stable-aarch64-unknown-linux-gnu/lib", {st_mode=S_IFDIR|0755, st_size=4096, ...}, 0) = 0
openat(AT_FDCWD, "/etc/ld.so.cache", O_RDONLY|O_CLOEXEC) = 3
newfstatat(3, "", {st_mode=S_IFREG|0644, st_size=16266, ...}, AT_EMPTY_PATH) = 0
mmap(NULL, 16266, PROT_READ, MAP_PRIVATE, 3, 0) = 0xffff89f3c000
close(3)                                = 0
openat(AT_FDCWD, "/lib/aarch64-linux-gnu/libgcc_s.so.1", O_RDONLY|O_CLOEXEC) = 3
read(3, "\177ELF\2\1\1\0\0\0\0\0\0\0\0\0\3\0\267\0\1\0\0\0\0\0\0\0\0\0\0\0"..., 832) = 832
newfstatat(3, "", {st_mode=S_IFREG|0644, st_size=133448, ...}, AT_EMPTY_PATH) = 0
mmap(NULL, 262856, PROT_NONE, MAP_PRIVATE|MAP_ANONYMOUS, -1, 0) = 0xffff89ec6000
mmap(0xffff89ed0000, 197320, PROT_READ|PROT_EXEC, MAP_PRIVATE|MAP_FIXED|MAP_DENYWRITE, 3, 0) = 0xffff89ed0000
munmap(0xffff89ec6000, 40960)           = 0
munmap(0xffff89f01000, 21192)           = 0
mprotect(0xffff89ee4000, 110592, PROT_NONE) = 0
mmap(0xffff89eff000, 8192, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_FIXED|MAP_DENYWRITE, 3, 0x1f000) = 0xffff89eff000
close(3)                                = 0
openat(AT_FDCWD, "/hdp/target/debug/build/sys-info-35d1c893f526b9bf/out/libc.so.6", O_RDONLY|O_CLOEXEC) = -1 ENOENT (No such file or directory)
openat(AT_FDCWD, "/hdp/target/debug/deps/libc.so.6", O_RDONLY|O_CLOEXEC) = -1 ENOENT (No such file or directory)
openat(AT_FDCWD, "/hdp/target/debug/libc.so.6", O_RDONLY|O_CLOEXEC) = -1 ENOENT (No such file or directory)
openat(AT_FDCWD, "/root/.rustup/toolchains/stable-aarch64-unknown-linux-gnu/lib/rustlib/aarch64-unknown-linux-gnu/lib/libc.so.6", O_RDONLY|O_CLOEXEC) = -1 ENOENT (No such file or directory)
openat(AT_FDCWD, "/root/.rustup/toolchains/stable-aarch64-unknown-linux-gnu/lib/libc.so.6", O_RDONLY|O_CLOEXEC) = -1 ENOENT (No such file or directory)
openat(AT_FDCWD, "/lib/aarch64-linux-gnu/libc.so.6", O_RDONLY|O_CLOEXEC) = 3
read(3, "\177ELF\2\1\1\3\0\0\0\0\0\0\0\0\3\0\267\0\1\0\0\0000y\2\0\0\0\0\0"..., 832) = 832
newfstatat(3, "", {st_mode=S_IFREG|0755, st_size=1651408, ...}, AT_EMPTY_PATH) = 0
mmap(NULL, 1826912, PROT_NONE, MAP_PRIVATE|MAP_ANONYMOUS, -1, 0) = 0xffff89d11000
mmap(0xffff89d20000, 1761376, PROT_READ|PROT_EXEC, MAP_PRIVATE|MAP_FIXED|MAP_DENYWRITE, 3, 0) = 0xffff89d20000
munmap(0xffff89d11000, 61440)           = 0
munmap(0xffff89ecf000, 96)              = 0
mprotect(0xffff89ea7000, 86016, PROT_NONE) = 0
mmap(0xffff89ebc000, 24576, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_FIXED|MAP_DENYWRITE, 3, 0x18c000) = 0xffff89ebc000
mmap(0xffff89ec2000, 49248, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_FIXED|MAP_ANONYMOUS, -1, 0) = 0xffff89ec2000
close(3)                                = 0
mmap(NULL, 8192, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_ANONYMOUS, -1, 0) = 0xffff89f3a000
set_tid_address(0xffff89f3a0f0)         = 749
set_robust_list(0xffff89f3a100, 24)     = 0
rseq(0xffff89f3a740, 0x20, 0, 0xd428bc00) = 0
mprotect(0xffff89ebc000, 16384, PROT_READ) = 0
mprotect(0xffff89eff000, 4096, PROT_READ) = 0
mprotect(0xaaaab9c2c000, 16384, PROT_READ) = 0
mprotect(0xffff89f45000, 8192, PROT_READ) = 0
prlimit64(0, RLIMIT_STACK, NULL, {rlim_cur=8192*1024, rlim_max=RLIM64_INFINITY}) = 0
munmap(0xffff89f3c000, 16266)           = 0
ppoll([{fd=0, events=0}, {fd=1, events=0}, {fd=2, events=0}], 3, {tv_sec=0, tv_nsec=0}, NULL, 0) = 1 ([{fd=0, revents=POLLHUP}], left {tv_sec=0, tv_nsec=0})
rt_sigaction(SIGPIPE, {sa_handler=SIG_IGN, sa_mask=[PIPE], sa_flags=SA_RESTART}, {sa_handler=SIG_DFL, sa_mask=[], sa_flags=0}, 8) = 0
getrandom("\xec\x2a\x7f\x53\xc4\xfa\x24\x45", 8, GRND_NONBLOCK) = 8
brk(NULL)                               = 0xaaaaf8356000
brk(0xaaaaf8377000)                     = 0xaaaaf8377000
openat(AT_FDCWD, "/proc/self/maps", O_RDONLY|O_CLOEXEC) = 3
prlimit64(0, RLIMIT_STACK, NULL, {rlim_cur=8192*1024, rlim_max=RLIM64_INFINITY}) = 0
newfstatat(3, "", {st_mode=S_IFREG|0444, st_size=0, ...}, AT_EMPTY_PATH) = 0
read(3, "aaaab9bc0000-aaaab9c1c000 r-xp 0"..., 1024) = 1024
read(3, "59696                     /usr/l"..., 1024) = 1000
close(3)                                = 0
sched_getaffinity(749, 32, [0 1 2 3 4 5 6 7 8 9]) = 8
rt_sigaction(SIGSEGV, NULL, {sa_handler=SIG_DFL, sa_mask=[], sa_flags=0}, 8) = 0
sigaltstack(NULL, {ss_sp=NULL, ss_flags=SS_DISABLE, ss_size=0}) = 0
mmap(NULL, 20480, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_ANONYMOUS|MAP_STACK, -1, 0) = 0xffff89f35000
mprotect(0xffff89f35000, 4096, PROT_NONE) = 0
sigaltstack({ss_sp=0xffff89f36000, ss_flags=0, ss_size=16384}, NULL) = 0
rt_sigaction(SIGSEGV, {sa_handler=0xaaaab9becc98, sa_mask=[], sa_flags=SA_ONSTACK|SA_SIGINFO}, NULL, 8) = 0
rt_sigaction(SIGBUS, NULL, {sa_handler=SIG_DFL, sa_mask=[], sa_flags=0}, 8) = 0
rt_sigaction(SIGBUS, {sa_handler=0xaaaab9becc98, sa_mask=[], sa_flags=SA_ONSTACK|SA_SIGINFO}, NULL, 8) = 0
ioctl(0, TCGETS, 0xfffff05eec00)        = -1 ENOTTY (Inappropriate ioctl for device)
read(0, "asdasdasdasgfqeageagtaetea\n", 32) = 27
read(0, "", 32)                         = 0
write(1, "| Protocol Number | Succeeded (C"..., 109) = 109
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0xffffffff /* IPPROTO_??? */) = -1 EINVAL (Invalid argument)
write(1, "| -1 | \360\237\244\257 | - | - | Invalid ar"..., 55) = 55
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, IPPROTO_IP) = -1 EPROTONOSUPPORT (Protocol not supported)
write(1, "| 0 | \360\237\244\257 | - | - | Protocol no"..., 60) = 60
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, IPPROTO_ICMP) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271C\219|asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 1 | \360\237\253\241 | 1739639297068317 | "..., 41) = 41
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, IPPROTO_IGMP) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271C\306\300\255asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 2 | \360\237\253\241 | 1739639297080213 | "..., 41) = 41
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0x3 /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271D~\240\307asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 3 | \360\237\253\241 | 1739639297092264 | "..., 41) = 41
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, IPPROTO_IPIP) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271EQ\272rasdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 4 | \360\237\253\241 | 1739639297106098 | "..., 41) = 41
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0x5 /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271F(\4\330asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 5 | \360\237\253\241 | 1739639297120142 | "..., 41) = 41
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, IPPROTO_TCP) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271F\327\333\1asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 6 | \360\237\253\241 | 1739639297131666 | "..., 41) = 41
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0x7 /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271G\243mCasdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 7 | \360\237\253\241 | 1739639297145007 | "..., 41) = 41
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, IPPROTO_EGP) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271HU\0\222asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 8 | \360\237\253\241 | 1739639297156645 | "..., 41) = 41
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0x9 /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271I!\35-asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 9 | \360\237\253\241 | 1739639297170021 | "..., 41) = 41
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0xa /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271I\370\2421asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 10 | \360\237\253\241 | 1739639297184146 |"..., 42) = 42
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0xb /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271J\316\7\277asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 11 | \360\237\253\241 | 1739639297198131 |"..., 42) = 42
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, IPPROTO_PUP) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271K\243\332[asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 12 | \360\237\253\241 | 1739639297212144 |"..., 42) = 42
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0xd /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271LyKNasdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 13 | \360\237\253\241 | 1739639297226132 |"..., 42) = 42
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0xe /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271MN\310\312asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 14 | \360\237\253\241 | 1739639297240123 |"..., 42) = 42
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0xf /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271N&\216\30asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 15 | \360\237\253\241 | 1739639297254264 |"..., 42) = 42
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0x10 /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271N\371\360Zasdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 16 | \360\237\253\241 | 1739639297268117 |"..., 42) = 42
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, IPPROTO_UDP) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271O\251\326\240asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 17 | \360\237\253\241 | 1739639297279645 |"..., 42) = 42
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0x12 /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271Pu\242\323asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 18 | \360\237\253\241 | 1739639297293001 |"..., 42) = 42
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0x13 /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271QM\325\253asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 19 | \360\237\253\241 | 1739639297307170 |"..., 42) = 42
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0x14 /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271R#\tkasdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 20 | \360\237\253\241 | 1739639297321142 |"..., 42) = 42
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0x15 /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271R\370\225casdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 21 | \360\237\253\241 | 1739639297335137 |"..., 42) = 42
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, IPPROTO_IDP) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271S\2507$asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 22 | \360\237\253\241 | 1739639297346647 |"..., 42) = 42
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0x17 /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271TX\273tasdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 23 | \360\237\253\241 | 1739639297358216 |"..., 42) = 42
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0x18 /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271U,\346\22asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 24 | \360\237\253\241 | 1739639297372120 |"..., 42) = 42
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0x19 /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271V\2\210\323asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 25 | \360\237\253\241 | 1739639297386121 |"..., 42) = 42
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0x1a /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271V\330m\251asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 26 | \360\237\253\241 | 1739639297400139 |"..., 42) = 42
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0x1b /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271W\256\24\317asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 27 | \360\237\253\241 | 1739639297414141 |"..., 42) = 42
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0x1c /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271X\203Z$asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 28 | \360\237\253\241 | 1739639297428118 |"..., 42) = 42
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, IPPROTO_TP) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271YYG\304asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 29 | \360\237\253\241 | 1739639297442138 |"..., 42) = 42
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0x1e /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271Z.\331\230asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 30 | \360\237\253\241 | 1739639297456134 |"..., 42) = 42
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0x1f /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271Z\336\207\341asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 31 | \360\237\253\241 | 1739639297467648 |"..., 42) = 42
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0x20 /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271[\252\245\363asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 32 | \360\237\253\241 | 1739639297481025 |"..., 42) = 42
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, IPPROTO_DCCP) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271\\\\ \10asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 33 | \360\237\253\241 | 1739639297492656 |"..., 42) = 42
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0x22 /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271](\0jasdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 34 | \360\237\253\241 | 1739639297506017 |"..., 42) = 42
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0x23 /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271]\377\367\\asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 35 | \360\237\253\241 | 1739639297520170 |"..., 42) = 42
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0x24 /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271^\324\250\276asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 36 | \360\237\253\241 | 1739639297534110 |"..., 42) = 42
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0x25 /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271_\252\241\231asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 37 | \360\237\253\241 | 1739639297548132 |"..., 42) = 42
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0x26 /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271`\200=\256asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 38 | \360\237\253\241 | 1739639297562132 |"..., 42) = 42
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0x27 /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271a0\t\226asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 39 | \360\237\253\241 | 1739639297573653 |"..., 42) = 42
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0x28 /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271a\373\365\206asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 40 | \360\237\253\241 | 1739639297587017 |"..., 42) = 42
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, IPPROTO_IPV6) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271b\324\3kasdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 41 | \360\237\253\241 | 1739639297601176 |"..., 42) = 42
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0x2a /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271c\250\327\242asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 42 | \360\237\253\241 | 1739639297615124 |"..., 42) = 42
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, IPPROTO_ROUTING) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271d\177\31\221asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 43 | \360\237\253\241 | 1739639297629166 |"..., 42) = 42
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, IPPROTO_FRAGMENT) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271eT\36\36asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 44 | \360\237\253\241 | 1739639297643126 |"..., 42) = 42
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0x2d /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271f*\3\356asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 45 | \360\237\253\241 | 1739639297657144 |"..., 42) = 42
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, IPPROTO_RSVP) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271f\365H\341asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 46 | \360\237\253\241 | 1739639297670465 |"..., 42) = 42
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, IPPROTO_GRE) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271g\304\257\36asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 47 | \360\237\253\241 | 1739639297684058 |"..., 42) = 42
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0x30 /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271h\233\231\202asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 48 | \360\237\253\241 | 1739639297698142 |"..., 42) = 42
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0x31 /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271iKOqasdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 49 | \360\237\253\241 | 1739639297709658 |"..., 42) = 42
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, IPPROTO_ESP) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271i\363=Zasdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 50 | \360\237\253\241 | 1739639297720663 |"..., 42) = 42
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, IPPROTO_AH) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271j\276\377\"asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 51 | \360\237\253\241 | 1739639297734016 |"..., 42) = 42
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0x34 /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271k\226\303Masdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 52 | \360\237\253\241 | 1739639297748157 |"..., 42) = 42
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0x35 /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271ll\2Hasdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 53 | \360\237\253\241 | 1739639297762132 |"..., 42) = 42
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0x36 /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271m*\314hasdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 54 | \360\237\253\241 | 1739639297774636 |"..., 42) = 42
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0x37 /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271m\367\0041asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 55 | \360\237\253\241 | 1739639297788019 |"..., 42) = 42
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0x38 /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271n\316\336*asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 56 | \360\237\253\241 | 1739639297802165 |"..., 42) = 42
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0x39 /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271o\244&\300asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 57 | \360\237\253\241 | 1739639297816143 |"..., 42) = 42
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, IPPROTO_ICMPV6) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271pS\341>asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 58 | \360\237\253\241 | 1739639297827660 |"..., 42) = 42
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, IPPROTO_NONE) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271q\37\214\272asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 59 | \360\237\253\241 | 1739639297841007 |"..., 42) = 42
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, IPPROTO_DSTOPTS) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271q\367}\372asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 60 | \360\237\253\241 | 1739639297855159 |"..., 42) = 42
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0x3d /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271r\314\303yasdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 61 | \360\237\253\241 | 1739639297869136 |"..., 42) = 42
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0x3e /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271s\201W,asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 62 | \360\237\253\241 | 1739639297880971 |"..., 42) = 42
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0x3f /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271t4\246~asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 63 | \360\237\253\241 | 1739639297892722 |"..., 42) = 42
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0x40 /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271t\377!\361asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 64 | \360\237\253\241 | 1739639297905992 |"..., 42) = 42
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0x41 /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271u\327\202\205asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 65 | \360\237\253\241 | 1739639297920172 |"..., 42) = 42
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0x42 /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271v\254\236Yasdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 66 | \360\237\253\241 | 1739639297934139 |"..., 42) = 42
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0x43 /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271wYX\357asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 67 | \360\237\253\241 | 1739639297945459 |"..., 42) = 42
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0x44 /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271x(\336lasdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 68 | \360\237\253\241 | 1739639297959059 |"..., 42) = 42
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0x45 /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271y\0\27\227asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 69 | \360\237\253\241 | 1739639297973164 |"..., 42) = 42
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0x46 /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271y\325%\225asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 70 | \360\237\253\241 | 1739639297987126 |"..., 42) = 42
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0x47 /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271z\253\2\36asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 71 | \360\237\253\241 | 1739639298001142 |"..., 42) = 42
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0x48 /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271{\200\235\215asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 72 | \360\237\253\241 | 1739639298015141 |"..., 42) = 42
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0x49 /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271|V.\23asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 73 | \360\237\253\241 | 1739639298029137 |"..., 42) = 42
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0x4a /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271}$\276\37asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 74 | \360\237\253\241 | 1739639298042674 |"..., 42) = 42
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0x4b /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271}\360:\273asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 75 | \360\237\253\241 | 1739639298056010 |"..., 42) = 42
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0x4c /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271~\310\232\374asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 76 | \360\237\253\241 | 1739639298070190 |"..., 42) = 42
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0x4d /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271\177\235'\277asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 77 | \360\237\253\241 | 1739639298084120 |"..., 42) = 42
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0x4e /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271\200k\232\1asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 78 | \360\237\253\241 | 1739639298097650 |"..., 42) = 42
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0x4f /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271\2017\366\346asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 79 | \360\237\253\241 | 1739639298111043 |"..., 42) = 42
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0x50 /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271\202\17T]asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 80 | \360\237\253\241 | 1739639298125157 |"..., 42) = 42
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0x51 /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271\202\344\306!asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 81 | \360\237\253\241 | 1739639298139145 |"..., 42) = 42
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0x52 /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271\203\272/\33asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 82 | \360\237\253\241 | 1739639298153131 |"..., 42) = 42
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0x53 /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271\204j%*asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 83 | \360\237\253\241 | 1739639298164663 |"..., 42) = 42
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0x54 /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271\2055\327\320asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 84 | \360\237\253\241 | 1739639298178013 |"..., 42) = 42
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0x55 /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271\206\16\36\203asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 85 | \360\237\253\241 | 1739639298192187 |"..., 42) = 42
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0x56 /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271\206\343\6Aasdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 86 | \360\237\253\241 | 1739639298206140 |"..., 42) = 42
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0x57 /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271\207\270o\270asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 87 | \360\237\253\241 | 1739639298220126 |"..., 42) = 42
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0x58 /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271\210hc\252asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 88 | \360\237\253\241 | 1739639298231657 |"..., 42) = 42
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0x59 /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271\2113\371\252asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 89 | \360\237\253\241 | 1739639298244999 |"..., 42) = 42
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0x5a /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271\212\f&|asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 90 | \360\237\253\241 | 1739639298259166 |"..., 42) = 42
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0x5b /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271\212\3416\302asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 91 | \360\237\253\241 | 1739639298273130 |"..., 42) = 42
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, IPPROTO_MTP) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271\213\251\240Basdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 92 | \360\237\253\241 | 1739639298286264 |"..., 42) = 42
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0x5d /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271\214|\3641asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 93 | \360\237\253\241 | 1739639298300114 |"..., 42) = 42
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, IPPROTO_BEETPH) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271\215S=tasdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 94 | \360\237\253\241 | 1739639298314157 |"..., 42) = 42
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0x5f /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271\216(~:asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 95 | \360\237\253\241 | 1739639298328133 |"..., 42) = 42
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0x60 /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271\216\347p\266asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 96 | \360\237\253\241 | 1739639298340647 |"..., 42) = 42
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0x61 /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271\217\263[\6asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 97 | \360\237\253\241 | 1739639298354010 |"..., 42) = 42
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, IPPROTO_ENCAP) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271\220\213\201Uasdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 98 | \360\237\253\241 | 1739639298368176 |"..., 42) = 42
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0x63 /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271\221`B\324asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 99 | \360\237\253\241 | 1739639298382119 |"..., 42) = 42
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0x64 /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271\222\20k\377asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 100 | \360\237\253\241 | 1739639298393664 "..., 43) = 43
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0x65 /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271\222\333\361\342asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 101 | \360\237\253\241 | 1739639298407002 "..., 43) = 43
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0x66 /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271\223\215\300Gasdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 102 | \360\237\253\241 | 1739639298418655 "..., 43) = 43
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, IPPROTO_PIM) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271\224Y\2208asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 103 | \360\237\253\241 | 1739639298432012 "..., 43) = 43
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0x68 /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271\2251n\227asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 104 | \360\237\253\241 | 1739639298446159 "..., 43) = 43
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0x69 /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271\225\340\274\10asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 105 | \360\237\253\241 | 1739639298457648 "..., 43) = 43
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0x6a /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271\226\254u0asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 106 | \360\237\253\241 | 1739639298470999 "..., 43) = 43
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0x6b /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271\227\204\305\1asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 107 | \360\237\253\241 | 1739639298485175 "..., 43) = 43
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, IPPROTO_COMP) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271\230Y\244\37asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 108 | \360\237\253\241 | 1739639298499126 "..., 43) = 43
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0x6d /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271\231/\265\267asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 109 | \360\237\253\241 | 1739639298513155 "..., 43) = 43
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0x6e /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271\231\336>\213asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 110 | \360\237\253\241 | 1739639298524594 "..., 43) = 43
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0x6f /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271\232\253\303\177asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 111 | \360\237\253\241 | 1739639298538062 "..., 43) = 43
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0x70 /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271\233\202\330\334asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 112 | \360\237\253\241 | 1739639298552158 "..., 43) = 43
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0x71 /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271\2342_\301asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 113 | \360\237\253\241 | 1739639298563661 "..., 43) = 43
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0x72 /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271\234\376\0327asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 114 | \360\237\253\241 | 1739639298577013 "..., 43) = 43
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, IPPROTO_L2TP) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271\235\326\25\21asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 115 | \360\237\253\241 | 1739639298591167 "..., 43) = 43
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0x74 /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271\236\253&zasdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 116 | \360\237\253\241 | 1739639298605131 "..., 43) = 43
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0x75 /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271\237Z\374Pasdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 117 | \360\237\253\241 | 1739639298616655 "..., 43) = 43
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0x76 /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271\240&\307\334asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 118 | \360\237\253\241 | 1739639298630010 "..., 43) = 43
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0x77 /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271\240\330\215\312asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 119 | \360\237\253\241 | 1739639298641661 "..., 43) = 43
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0x78 /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271\241\243\373\227asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 120 | \360\237\253\241 | 1739639298654993 "..., 43) = 43
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0x79 /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271\242|%\316asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 121 | \360\237\253\241 | 1739639298669160 "..., 43) = 43
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0x7a /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271\243Q/\272asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 122 | \360\237\253\241 | 1739639298683121 "..., 43) = 43
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0x7b /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271\244\31\207}asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 123 | \360\237\253\241 | 1739639298696251 "..., 43) = 43
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0x7c /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271\244\355\nLasdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 124 | \360\237\253\241 | 1739639298710112 "..., 43) = 43
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0x7d /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271\245\303 Jasdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 125 | \360\237\253\241 | 1739639298724143 "..., 43) = 43
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0x7e /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271\246\230\226\37asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 126 | \360\237\253\241 | 1739639298738132 "..., 43) = 43
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0x7f /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271\247nJ\236asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 127 | \360\237\253\241 | 1739639298752138 "..., 43) = 43
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0x80 /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271\250C\322\330asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 128 | \360\237\253\241 | 1739639298766132 "..., 43) = 43
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0x81 /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271\251\31\377!asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 129 | \360\237\253\241 | 1739639298780168 "..., 43) = 43
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0x82 /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271\251\357\vUasdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 130 | \360\237\253\241 | 1739639298794130 "..., 43) = 43
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0x83 /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271\252\304\3501asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 131 | \360\237\253\241 | 1739639298808146 "..., 43) = 43
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, IPPROTO_SCTP) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271\253\232\201\5asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 132 | \360\237\253\241 | 1739639298822144 "..., 43) = 43
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0x85 /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271\254p1\234asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 133 | \360\237\253\241 | 1739639298836148 "..., 43) = 43
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0x86 /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271\255BM\262asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 134 | \360\237\253\241 | 1739639298849918 "..., 43) = 43
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, IPPROTO_MH) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271\256\f\352\326asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 135 | \360\237\253\241 | 1739639298863197 "..., 43) = 43
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, IPPROTO_UDPLITE) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271\256\336_qasdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 136 | \360\237\253\241 | 1739639298876923 "..., 43) = 43
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, IPPROTO_MPLS) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271\257\267\350\25asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 137 | \360\237\253\241 | 1739639298891180 "..., 43) = 43
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0x8a /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271\260\214\311\244asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 138 | \360\237\253\241 | 1739639298905131 "..., 43) = 43
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0x8b /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271\261bxGasdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 139 | \360\237\253\241 | 1739639298919135 "..., 43) = 43
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0x8c /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271\262%dXasdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 140 | \360\237\253\241 | 1739639298931909 "..., 43) = 43
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0x8d /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271\262\372\334Kasdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 141 | \360\237\253\241 | 1739639298945899 "..., 43) = 43
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0x8e /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271\263\261l\237asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 142 | \360\237\253\241 | 1739639298957864 "..., 43) = 43
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, IPPROTO_ETHERNET) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271\264\214<?asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 143 | \360\237\253\241 | 1739639298972204 "..., 43) = 43
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0x90 /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271\265`m\334asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 144 | \360\237\253\241 | 1739639298986110 "..., 43) = 43
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0x91 /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271\2666e\275asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 145 | \360\237\253\241 | 1739639299000133 "..., 43) = 43
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0x92 /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271\267\v\374sasdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 146 | \360\237\253\241 | 1739639299014130 "..., 43) = 43
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0x93 /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271\267\341\2036asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 147 | \360\237\253\241 | 1739639299028124 "..., 43) = 43
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0x94 /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271\270\267,\315asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 148 | \360\237\253\241 | 1739639299042127 "..., 43) = 43
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0x95 /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271\271\214\270rasdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 149 | \360\237\253\241 | 1739639299056122 "..., 43) = 43
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0x96 /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271\272Ol\10asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 150 | \360\237\253\241 | 1739639299068882 "..., 43) = 43
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0x97 /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271\273\2\357\354asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 151 | \360\237\253\241 | 1739639299080646 "..., 43) = 43
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0x98 /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271\273\316\274\360asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 152 | \360\237\253\241 | 1739639299094003 "..., 43) = 43
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0x99 /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271\274\246\220\221asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 153 | \360\237\253\241 | 1739639299108147 "..., 43) = 43
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0x9a /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271\275|;\362asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 154 | \360\237\253\241 | 1739639299122150 "..., 43) = 43
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0x9b /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271\276Q\261\236asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 155 | \360\237\253\241 | 1739639299136139 "..., 43) = 43
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0x9c /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271\277',,asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 156 | \360\237\253\241 | 1739639299150130 "..., 43) = 43
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0x9d /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271\277\375\6\301asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 157 | \360\237\253\241 | 1739639299164145 "..., 43) = 43
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0x9e /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271\300\322\307\236asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 158 | \360\237\253\241 | 1739639299178154 "..., 43) = 43
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0x9f /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271\301\202?\rasdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 159 | \360\237\253\241 | 1739639299189653 "..., 43) = 43
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0xa0 /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271\302N)\207asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 160 | \360\237\253\241 | 1739639299203017 "..., 43) = 43
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0xa1 /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271\303&\0272asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 161 | \360\237\253\241 | 1739639299217168 "..., 43) = 43
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0xa2 /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271\303\373E\275asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 162 | \360\237\253\241 | 1739639299231139 "..., 43) = 43
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0xa3 /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271\304\306\311\6asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 163 | \360\237\253\241 | 1739639299244476 "..., 43) = 43
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0xa4 /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271\305\225\353dasdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 164 | \360\237\253\241 | 1739639299258051 "..., 43) = 43
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0xa5 /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271\306H\236\311asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 165 | \360\237\253\241 | 1739639299269762 "..., 43) = 43
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0xa6 /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271\306\363\217^asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 166 | \360\237\253\241 | 1739639299280965 "..., 43) = 43
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0xa7 /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271\307\314}8asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 167 | \360\237\253\241 | 1739639299295182 "..., 43) = 43
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0xa8 /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271\310\241\236Aasdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 168 | \360\237\253\241 | 1739639299309149 "..., 43) = 43
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0xa9 /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271\311v\274\tasdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 169 | \360\237\253\241 | 1739639299323116 "..., 43) = 43
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0xaa /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271\312L\250\331asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 170 | \360\237\253\241 | 1739639299337136 "..., 43) = 43
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0xab /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271\313\"96asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 171 | \360\237\253\241 | 1739639299351132 "..., 43) = 43
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0xac /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271\313\367\265\267asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 172 | \360\237\253\241 | 1739639299365123 "..., 43) = 43
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0xad /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271\314\274\1Xasdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 173 | \360\237\253\241 | 1739639299377988 "..., 43) = 43
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0xae /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271\315nl&asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 174 | \360\237\253\241 | 1739639299389680 "..., 43) = 43
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0xaf /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271\3169\334\215asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 175 | \360\237\253\241 | 1739639299403013 "..., 43) = 43
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0xb0 /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271\316\353\2473asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 176 | \360\237\253\241 | 1739639299414665 "..., 43) = 43
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0xb1 /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271\317\230f/asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 177 | \360\237\253\241 | 1739639299425986 "..., 43) = 43
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0xb2 /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271\320K\277nasdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 178 | \360\237\253\241 | 1739639299437740 "..., 43) = 43
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0xb3 /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271\321\25\313\341asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 179 | \360\237\253\241 | 1739639299450981 "..., 43) = 43
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0xb4 /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271\321\356?\200asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 180 | \360\237\253\241 | 1739639299465166 "..., 43) = 43
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0xb5 /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271\322\303\235\25asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 181 | \360\237\253\241 | 1739639299479149 "..., 43) = 43
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0xb6 /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271\323r\3771asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 182 | \360\237\253\241 | 1739639299490643 "..., 43) = 43
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0xb7 /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271\324?4\263asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 183 | \360\237\253\241 | 1739639299504026 "..., 43) = 43
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0xb8 /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271\325\26\323\353asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 184 | \360\237\253\241 | 1739639299518157 "..., 43) = 43
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0xb9 /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271\325\354<hasdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 185 | \360\237\253\241 | 1739639299532143 "..., 43) = 43
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0xba /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271\326\234\0242asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 186 | \360\237\253\241 | 1739639299543667 "..., 43) = 43
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0xbb /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271\327g\214\223asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 187 | \360\237\253\241 | 1739639299557002 "..., 43) = 43
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0xbc /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271\330\31S\244asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 188 | \360\237\253\241 | 1739639299568653 "..., 43) = 43
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0xbd /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271\330\347P\267asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 189 | \360\237\253\241 | 1739639299582153 "..., 43) = 43
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0xbe /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271\331\236_\325asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 190 | \360\237\253\241 | 1739639299594149 "..., 43) = 43
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0xbf /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271\332R\371;asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 191 | \360\237\253\241 | 1739639299605985 "..., 43) = 43
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0xc0 /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271\333\20k\232asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 192 | \360\237\253\241 | 1739639299618401 "..., 43) = 43
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0xc1 /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271\333\301dJasdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 193 | \360\237\253\241 | 1739639299629999 "..., 43) = 43
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0xc2 /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271\334{\372\273asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 194 | \360\237\253\241 | 1739639299642227 "..., 43) = 43
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0xc3 /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271\3351\335dasdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 195 | \360\237\253\241 | 1739639299654147 "..., 43) = 43
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0xc4 /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271\335\346s\fasdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 196 | \360\237\253\241 | 1739639299665982 "..., 43) = 43
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0xc5 /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271\336\263\\\221asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 197 | \360\237\253\241 | 1739639299679411 "..., 43) = 43
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0xc6 /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271\337^\333\220asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 198 | \360\237\253\241 | 1739639299690650 "..., 43) = 43
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0xc7 /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271\340\35\273\250asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 199 | \360\237\253\241 | 1739639299703159 "..., 43) = 43
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0xc8 /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271\340\322q\212asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 200 | \360\237\253\241 | 1739639299715002 "..., 43) = 43
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0xc9 /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271\341\217\214\254asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 201 | \360\237\253\241 | 1739639299727396 "..., 43) = 43
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0xca /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271\342@\272\30asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 202 | \360\237\253\241 | 1739639299739007 "..., 43) = 43
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0xcb /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271\342\375\260qasdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 203 | \360\237\253\241 | 1739639299751391 "..., 43) = 43
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0xcc /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271\343\256\337\321asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 204 | \360\237\253\241 | 1739639299763003 "..., 43) = 43
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0xcd /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271\344iD\307asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 205 | \360\237\253\241 | 1739639299775219 "..., 43) = 43
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0xce /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271\345\37J\33asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 206 | \360\237\253\241 | 1739639299787147 "..., 43) = 43
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0xcf /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271\345\326g\17asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 207 | \360\237\253\241 | 1739639299799148 "..., 43) = 43
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0xd0 /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271\346\215\232\243asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 208 | \360\237\253\241 | 1739639299811154 "..., 43) = 43
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0xd1 /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271\347/\36uasdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 209 | \360\237\253\241 | 1739639299821739 "..., 43) = 43
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0xd2 /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271\347\332\234yasdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 210 | \360\237\253\241 | 1739639299832978 "..., 43) = 43
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0xd3 /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271\350\230\23\221asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 211 | \360\237\253\241 | 1739639299845395 "..., 43) = 43
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0xd4 /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271\351H\370\271asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 212 | \360\237\253\241 | 1739639299856988 "..., 43) = 43
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0xd5 /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271\352\3\236\240asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 213 | \360\237\253\241 | 1739639299869220 "..., 43) = 43
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0xd6 /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271\352\267\3nasdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 214 | \360\237\253\241 | 1739639299880977 "..., 43) = 43
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0xd7 /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271\353q\266\255asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 215 | \360\237\253\241 | 1739639299893213 "..., 43) = 43
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0xd8 /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271\354%@\301asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 216 | \360\237\253\241 | 1739639299904979 "..., 43) = 43
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0xd9 /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271\354\337\350Hasdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 217 | \360\237\253\241 | 1739639299917211 "..., 43) = 43
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0xda /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271\355\220n\214asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 218 | \360\237\253\241 | 1739639299928780 "..., 43) = 43
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0xdb /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271\356N\201\24asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 219 | \360\237\253\241 | 1739639299941237 "..., 43) = 43
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0xdc /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271\357\4)yasdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 220 | \360\237\253\241 | 1739639299953142 "..., 43) = 43
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0xdd /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271\357\270\320\16asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 221 | \360\237\253\241 | 1739639299964981 "..., 43) = 43
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0xde /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271\360v/\337asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 222 | \360\237\253\241 | 1739639299977392 "..., 43) = 43
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0xdf /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271\361'K\341asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 223 | \360\237\253\241 | 1739639299988999 "..., 43) = 43
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0xe0 /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271\361\344k\221asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 224 | \360\237\253\241 | 1739639300001393 "..., 43) = 43
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0xe1 /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271\362\227\265\204asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 225 | \360\237\253\241 | 1739639300013143 "..., 43) = 43
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0xe2 /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271\363N\300\344asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 226 | \360\237\253\241 | 1739639300025139 "..., 43) = 43
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0xe3 /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271\364\3\235/asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 227 | \360\237\253\241 | 1739639300036992 "..., 43) = 43
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0xe4 /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271\364\2763\311asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 228 | \360\237\253\241 | 1739639300049220 "..., 43) = 43
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0xe5 /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271\365ta\317asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 229 | \360\237\253\241 | 1739639300061160 "..., 43) = 43
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0xe6 /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271\366(\335\277asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 230 | \360\237\253\241 | 1739639300072988 "..., 43) = 43
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0xe7 /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271\366\346==asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 231 | \360\237\253\241 | 1739639300085398 "..., 43) = 43
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0xe8 /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271\367\227 \304asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 232 | \360\237\253\241 | 1739639300096991 "..., 43) = 43
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0xe9 /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271\370TY\tasdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 233 | \360\237\253\241 | 1739639300109392 "..., 43) = 43
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0xea /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271\371\4\325\334asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 234 | \360\237\253\241 | 1739639300120958 "..., 43) = 43
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0xeb /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271\371\277\350\246asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 235 | \360\237\253\241 | 1739639300133218 "..., 43) = 43
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0xec /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271\372u\325\24asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 236 | \360\237\253\241 | 1739639300145141 "..., 43) = 43
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0xed /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271\373*wmasdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 237 | \360\237\253\241 | 1739639300156979 "..., 43) = 43
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0xee /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271\373\345\337Wasdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 238 | \360\237\253\241 | 1739639300169261 "..., 43) = 43
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0xef /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271\374\223\242@asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 239 | \360\237\253\241 | 1739639300180648 "..., 43) = 43
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0xf0 /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271\375@z\362asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 240 | \360\237\253\241 | 1739639300191976 "..., 43) = 43
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0xf1 /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271\375\373j\224asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 241 | \360\237\253\241 | 1739639300204227 "..., 43) = 43
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0xf2 /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271\376\261H\206asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 242 | \360\237\253\241 | 1739639300216146 "..., 43) = 43
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0xf3 /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\271\377o|\224asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 243 | \360\237\253\241 | 1739639300228611 "..., 43) = 43
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0xf4 /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\272\0\37{nasdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 244 | \360\237\253\241 | 1739639300240145 "..., 43) = 43
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0xf5 /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\272\0\325\242Jasdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 245 | \360\237\253\241 | 1739639300252082 "..., 43) = 43
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0xf6 /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\272\1\215\274\374asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 246 | \360\237\253\241 | 1739639300264148 "..., 43) = 43
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0xf7 /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\272\2BL\310asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 247 | \360\237\253\241 | 1739639300275981 "..., 43) = 43
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0xf8 /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\272\2\377\310Easdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 248 | \360\237\253\241 | 1739639300288399 "..., 43) = 43
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0xf9 /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\272\3\260\226Pasdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 249 | \360\237\253\241 | 1739639300299986 "..., 43) = 43
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0xfa /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\272\4n\36\250asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 250 | \360\237\253\241 | 1739639300312407 "..., 43) = 43
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0xfb /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\272\5\36\274\260asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 251 | \360\237\253\241 | 1739639300323982 "..., 43) = 43
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0xfc /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\272\5\324wyasdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 252 | \360\237\253\241 | 1739639300335892 "..., 43) = 43
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0xfd /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\272\6\223J\17asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 253 | \360\237\253\241 | 1739639300348398 "..., 43) = 43
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0xfe /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\272\7DM\321asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 254 | \360\237\253\241 | 1739639300359999 "..., 43) = 43
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, IPPROTO_RAW) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\272\7\376\3465asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 255 | \360\237\253\241 | 1739639300372227 "..., 43) = 43
clock_nanosleep(CLOCK_REALTIME, 0, {tv_sec=0, tv_nsec=10000000}, 0xfffff05eec48) = 0
socket(AF_INET, SOCK_RAW|SOCK_CLOEXEC, 0x100 /* IPPROTO_??? */) = 3
setsockopt(3, SOL_IP, IP_HDRINCL, [0], 4) = 0
sendto(3, "\1\244\1\244\30$p\272\10\262S\243asdasdasdasgfqeageag"..., 39, 0, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 39
fcntl(3, F_GETFD)                       = 0x1 (flags FD_CLOEXEC)
close(3)                                = 0
write(1, "| 256 | \360\237\253\241 | 1739639300383986 "..., 43) = 43
sigaltstack({ss_sp=NULL, ss_flags=SS_DISABLE, ss_size=16384}, NULL) = 0
munmap(0xffff89f35000, 20480)           = 0
exit_group(0)                           = ?
+++ exited with 0 +++
