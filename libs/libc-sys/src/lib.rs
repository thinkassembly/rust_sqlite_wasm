// Types (the libc crate doesn't yet support wasm32-unknown-unknown)
#[allow(non_camel_case_types)]
pub type c_float = f32;

use wasm_bindgen::JsCast;


extern crate js_sys;

#[derive(Debug)]
#[allow(non_camel_case_types)]
#[allow(dead_code)]
enum SysCallKind {
    // Generted using the following command:
// grep "^#define SYS_" obj/include/bits/syscall.h | awk '{print $3 ": \"" $2 "\"," }'
    restart_syscall,
    exit,
    fork,
    read,
    write,
    open,
    close,
    waitpid,
    SYS_creat,
    link,
    unlink,
    execve,
    chdir,
    time,
    mknod,
    chmod,
    lchown,
    BREAK,
    oldstat,
    lseek,
    getpid,
    mount,
    umount,
    setuid,
    getuid,
    stime,
    ptrace,
    alarm,
    oldfstat,
    pause,
    utime,
    stty,
    gtty,
    access,
    nice,
    ftime,
    sync,
    kill,
    rename,
    mkdir,
    rmdir,
    dup,
    pipe,
    times,
    prof,
    brk,
    setgid,
    getgid,
    signal,
    geteuid,
    getegid,
    acct,
    umount2,
    lock,
    ioctl,
    fcntl,
    mpx,
    setpgid,
    ulimit,
    oldolduname,
    umask,
    chroot,
    ustat,
    dup2,
    getppid,
    getpgrp,
    setsid,
    sigaction,
    sgetmask,
    ssetmask,
    setreuid,
    setregid,
    sigsuspend,
    sigpending,
    sethostname,
    setrlimit,
    getrlimit,
    getrusage,
    gettimeofday,
    settimeofday,
    getgroups,
    setgroups,
    select,
    symlink,
    oldlstat,
    readlink,
    uselib,
    swapon,
    reboot,
    readdir,
    mmap,
    munmap,
    truncate,
    ftruncate,
    fchmod,
    fchown,
    getpriority,
    setpriority,
    profil,
    statfs,
    fstatfs,
    ioperm,
    socketcall,
    syslog,
    setitimer,
    getitimer,
    stat,
    lstat,
    fstat,
    olduname,
    iopl,
    vhangup,
    idle,
    vm86old,
    wait4,
    swapoff,
    sysinfo,
    ipc,
    fsync,
    sigreturn,
    clone,
    setdomainname,
    uname,
    modify_ldt,
    adjtimex,
    mprotect,
    sigprocmask,
    create_module,
    init_module,
    delete_module,
    get_kernel_syms,
    quotactl,
    getpgid,
    fchdir,
    bdflush,
    sysfs,
    personality,
    afs_syscall,
    setfsuid,
    setfsgid,
    _llseek,
    getdents,
    _newselect,
    flock,
    msync,
    readv,
    writev,
    getsid,
    fdatasync,
    _sysctl,
    mlock,
    munlock,
    mlockall,
    munlockall,
    sched_setparam,
    sched_getparam,
    sched_setscheduler,
    sched_getscheduler,

    /*
    158: "SYS_sched_yield",
    159: "SYS_sched_get_priority_max",
    160: "SYS_sched_get_priority_min",
    161: "SYS_sched_rr_get_interval",
    162: "SYS_nanosleep",
    163: "SYS_mremap",
    164: "SYS_setresuid",
    165: "SYS_getresuid",
    166: "SYS_vm86",
    167: "SYS_query_module",
    168: "SYS_poll",
    169: "SYS_nfsservctl",
    170: "SYS_setresgid",
    171: "SYS_getresgid",
    172: "SYS_prctl",
    173: "SYS_rt_sigreturn",
    174: "SYS_rt_sigaction",
    175: "SYS_rt_sigprocmask",
    176: "SYS_rt_sigpending",
    177: "SYS_rt_sigtimedwait",
    178: "SYS_rt_sigqueueinfo",
    179: "SYS_rt_sigsuspend",
    180: "SYS_pread64",
    181: "SYS_pwrite64",
    182: "SYS_chown",
    183: "SYS_getcwd",
    184: "SYS_capget",
    185: "SYS_capset",
    186: "SYS_sigaltstack",
    187: "SYS_sendfile",
    188: "SYS_getpmsg",
    189: "SYS_putpmsg",
    190: "SYS_vfork",
    191: "SYS_ugetrlimit",
    */
    mmap2,
    /*
    193: "SYS_truncate64",
    194: "SYS_ftruncate64",
    195: "SYS_stat64",
    196: "SYS_lstat64",
    197: "SYS_fstat64",
    198: "SYS_lchown32",
    199: "SYS_getuid32",
    200: "SYS_getgid32",
    201: "SYS_geteuid32",
    202: "SYS_getegid32",
    203: "SYS_setreuid32",
    204: "SYS_setregid32",
    205: "SYS_getgroups32",
    206: "SYS_setgroups32",
    207: "SYS_fchown32",
    208: "SYS_setresuid32",
    209: "SYS_getresuid32",
    210: "SYS_setresgid32",
    211: "SYS_getresgid32",
    212: "SYS_chown32",
    213: "SYS_setuid32",
    214: "SYS_setgid32",
    215: "SYS_setfsuid32",
    216: "SYS_setfsgid32",
    217: "SYS_pivot_root",
    218: "SYS_mincore",
    219: "SYS_madvise",
    220: "SYS_getdents64",
    */
    fcntl64,
    /*
    224: "SYS_gettid",
    225: "SYS_readahead",
    226: "SYS_setxattr",
    227: "SYS_lsetxattr",
    228: "SYS_fsetxattr",
    229: "SYS_getxattr",
    230: "SYS_lgetxattr",
    231: "SYS_fgetxattr",
    232: "SYS_listxattr",
    233: "SYS_llistxattr",
    234: "SYS_flistxattr",
    235: "SYS_removexattr",
    236: "SYS_lremovexattr",
    237: "SYS_fremovexattr",
    238: "SYS_tkill",
    239: "SYS_sendfile64",
    240: "SYS_futex",
    241: "SYS_sched_setaffinity",
    242: "SYS_sched_getaffinity",
    243: "SYS_set_thread_area",
    244: "SYS_get_thread_area",
    245: "SYS_io_setup",
    246: "SYS_io_destroy",
    247: "SYS_io_getevents",
    248: "SYS_io_submit",
    249: "SYS_io_cancel",
    250: "SYS_fadvise64",
    252: "SYS_exit_group",
    253: "SYS_lookup_dcookie",
    254: "SYS_epoll_create",
    255: "SYS_epoll_ctl",
    256: "SYS_epoll_wait",
    257: "SYS_remap_file_pages",
    258: "SYS_set_tid_address",
    259: "SYS_timer_create",
    260: "SYS_timer_settime",
    261: "SYS_timer_gettime",
    262: "SYS_timer_getoverrun",
    263: "SYS_timer_delete",

    264:clock_settime,
    */

    clock_gettime,
    /*
    266: "SYS_clock_getres",
    267: "SYS_clock_nanosleep",
    268: "SYS_statfs64",
    269: "SYS_fstatfs64",
    270: "SYS_tgkill",
    271: "SYS_utimes",
    272: "SYS_fadvise64_64",
    273: "SYS_vserver",
    274: "SYS_mbind",
    275: "SYS_get_mempolicy",
    276: "SYS_set_mempolicy",
    277: "SYS_mq_open",
    278: "SYS_mq_unlink",
    279: "SYS_mq_timedsend",
    280: "SYS_mq_timedreceive",
    281: "SYS_mq_notify",
    282: "SYS_mq_getsetattr",
    283: "SYS_kexec_load",
    284: "SYS_waitid",
    286: "SYS_add_key",
    287: "SYS_request_key",
    288: "SYS_keyctl",
    289: "SYS_ioprio_set",
    290: "SYS_ioprio_get",
    291: "SYS_inotify_init",
    292: "SYS_inotify_add_watch",
    293: "SYS_inotify_rm_watch",
    294: "SYS_migrate_pages",
    295: "SYS_openat",
    296: "SYS_mkdirat",
    297: "SYS_mknodat",
    298: "SYS_fchownat",
    299: "SYS_futimesat",
    300: "SYS_fstatat64",
    301: "SYS_unlinkat",
    302: "SYS_renameat",
    303: "SYS_linkat",
    304: "SYS_symlinkat",
    305: "SYS_readlinkat",
    306: "SYS_fchmodat",
    307: "SYS_faccessat",
    308: "SYS_pselect6",
    309: "SYS_ppoll",
    310: "SYS_unshare",
    311: "SYS_set_robust_list",
    312: "SYS_get_robust_list",
    313: "SYS_splice",
    314: "SYS_sync_file_range",
    315: "SYS_tee",
    316: "SYS_vmsplice",
    317: "SYS_move_pages",
    318: "SYS_getcpu",
    319: "SYS_epoll_pwait",
    320: "SYS_utimensat",
    321: "SYS_signalfd",
    322: "SYS_timerfd_create",
    323: "SYS_eventfd",
    324: "SYS_fallocate",
    325: "SYS_timerfd_settime",
    326: "SYS_timerfd_gettime",
    327: "SYS_signalfd4",
    328: "SYS_eventfd2",
    329: "SYS_epoll_create1",
    330: "SYS_dup3",
    331: "SYS_pipe2",
    332: "SYS_inotify_init1",
    333: "SYS_preadv",
    334: "SYS_pwritev",
    335: "SYS_rt_tgsigqueueinfo",
    336: "SYS_perf_event_open",
    337: "SYS_recvmmsg",
    338: "SYS_fanotify_init",
    339: "SYS_fanotify_mark",
    340: "SYS_prlimit64",
    341: "SYS_name_to_handle_at",
    342: "SYS_open_by_handle_at",
    343: "SYS_clock_adjtime",
    344: "SYS_syncfs",
    345: "SYS_sendmmsg",
    346: "SYS_setns",
    347: "SYS_process_vm_readv",
    348: "SYS_process_vm_writev",
    349: "SYS_kcmp",
    350: "SYS_finit_module",
    351: "SYS_sched_setattr",
    352: "SYS_sched_getattr",
    353: "SYS_renameat2",
    354: "SYS_seccomp",
    355: "SYS_getrandom",
    356: "SYS_memfd_create",
    357: "SYS_bpf",
    358: "SYS_execveat",
    359: "SYS_socket",
    360: "SYS_socketpair",
    361: "SYS_bind",
    362: "SYS_connect",
    363: "SYS_listen",
    364: "SYS_accept4",
    365: "SYS_getsockopt",
    366: "SYS_setsockopt",
    367: "SYS_getsockname",
    368: "SYS_getpeername",
    369: "SYS_sendto",
    370: "SYS_sendmsg",
    371: "SYS_recvfrom",
    372: "SYS_recvmsg",
    373: "SYS_shutdown",
    374: "SYS_userfaultfd",
    375: "SYS_membarrier",
    376: "SYS_mlock2",
    377: "SYS_copy_file_range",
    378: "SYS_preadv2",
    379: "SYS_pwritev2",
    380: "SYS_pkey_mprotect",
    381: "SYS_pkey_alloc",
    382: "SYS_pkey_free",
    383: "SYS_statx",
    384: "SYS_arch_prctl", */
    Unknown(i32),
}

impl From<i32> for SysCallKind {
    fn from(item: i32) -> Self {
        match item {
            20 => SysCallKind::getpid,
            45 => SysCallKind::brk,
            192 => SysCallKind::mmap2,
            221 => SysCallKind::fcntl64,
            265 => SysCallKind::clock_gettime,
            _ => SysCallKind::Unknown(item)
        }
    }
}


extern crate wasm_bindgen;

#[wasm_bindgen]
extern {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

use wasm_bindgen::prelude::*;
use js_sys::Math::ceil;
use std::panic;
use std::ffi::CStr;
use std::os::raw::{c_char, c_int};
use wasm_bindgen::__rt::std::os::raw::c_long;

#[wasm_bindgen(module = "/js/libc.js")]
extern "C" {}



type Syscall0 = Box<dyn Fn(i32) -> i32>;
type Syscall1 = Box<dyn Fn(i32, i32) -> i32>;
type Syscall2 = Box<dyn Fn(i32, i32, i32) -> i32>;
type Syscall3 = Box<dyn Fn(i32, i32, i32, i32) -> i32>;
type Syscall4 = Box<dyn Fn(i32, i32, i32, i32, i32) -> i32>;
type Syscall5 = Box<dyn Fn(i32, i32, i32, i32, i32, i32) -> i32>;
type Syscall6 = Box<dyn Fn(i32, i32, i32, i32, i32, i32, i32) -> i32>;

#[allow(unused_variables)]
pub fn unknown_syscall0(a: i32) -> i32 {
    log(&format!("Unhandled syscall6 {:?}({})", SysCallKind::from(a.clone()), a.clone()));
    -1
}

#[allow(unused_variables)]
pub fn unknown_syscall1(a: i32, b: i32) -> i32 {
    log(&format!("Unhandled syscall1 {:?}({})", SysCallKind::from(a.clone()), a.clone()));
    -1
}

#[allow(unused_variables)]
pub fn unknown_syscall2(a: i32, b: i32, c: i32) -> i32 {
    log(&format!("Unhandled syscall2 {:?}({})", SysCallKind::from(a.clone()), a.clone()));
    -1
}

#[allow(unused_variables)]
pub fn unknown_syscall3(a: i32, b: i32, c: i32, d: i32) -> i32 {
    log(&format!("Unhandled syscall6 {:?}({})", SysCallKind::from(a.clone()), a.clone()));
    -1
}

#[allow(unused_variables)]
pub fn unknown_syscall4(a: i32, b: i32, c: i32, d: i32, e: i32) -> i32 {
    log(&format!("Unhandled syscall6 {:?}({})", SysCallKind::from(a.clone()), a.clone()));
    -1
}

#[allow(unused_variables)]
pub fn unknown_syscall5(a: i32, b: i32, c: i32, d: i32, e: i32, f: i32) -> i32 {
    log(&format!("Unhandled syscall6 {:?}({})", SysCallKind::from(a.clone()), a.clone()));
    -1
}

#[allow(unused_variables)]
pub fn unknown_syscall6(a: i32, b: i32, c: i32, d: i32, e: i32, f: i32, g: i32) -> i32 {
    log(&format!("Unhandled syscall6 {:?}({})", SysCallKind::from(a.clone()), a.clone()));
    -1
}


impl From<SysCallKind> for Syscall0 {
    fn from(item: SysCallKind) -> Self {
        Box::new(match item {
            SysCallKind::getpid => syscall_getpid,
            _ => unknown_syscall0
        })
    }
}

impl From<SysCallKind> for Syscall1 {
    fn from(item: SysCallKind) -> Self {
        match item {
            SysCallKind::brk => Box::new(syscall_brk),
            _ => Box::new(unknown_syscall1)
        }
    }
}

impl From<SysCallKind> for Syscall2 {
    fn from(item: SysCallKind) -> Self {
        match item {
            SysCallKind::clock_gettime => Box::new(syscall_gettime),
            _ => Box::new(unknown_syscall2)
        }
    }
}

impl From<SysCallKind> for Syscall3 {
    fn from(item: SysCallKind) -> Self {
        Box::new(match item {
            SysCallKind::fcntl64 => syscall_fcntl64,
            _ => unknown_syscall3
        })
    }
}

impl From<SysCallKind> for Syscall4 {
    fn from(item: SysCallKind) -> Self {
        match item {
            _ => Box::new(unknown_syscall4)
        }
    }
}

impl From<SysCallKind> for Syscall5 {
    fn from(item: SysCallKind) -> Self {
        match item {
            _ => Box::new(unknown_syscall5)
        }
    }
}

impl From<SysCallKind> for Syscall6 {
    fn from(item: SysCallKind) -> Self {
        Box::new(match item {
            SysCallKind::mmap2 => syscall_mmap2,
            _ => unknown_syscall6
        })
    }
}

fn syscall_brk(_a: i32, _b: i32) -> i32 {
    0
}


#[derive(Debug)]
#[repr(C)]
struct TimeSpec {
    sec: c_long,
    nsec: c_long,
}

#[allow(unused_variables)]
fn syscall_gettime(a: i32, b: i32, c: i32) -> i32 {
    let r = c as *mut TimeSpec;

    unsafe {
        //let js_time = js_sys::Date::get_time(&js_sys::Date::new_0());
        (*r).sec = (js_sys::Date::now() as u64 / 1000) as c_long;
        (*r).nsec = 0 as c_long;
     //   log(&format!("{} {} {:?}", js_sys::Date::now(), b, *r));
    }
    0
}

#[allow(unused_variables)]
fn syscall_mmap2(a: i32, b: i32, requested: i32, d: i32, e: i32, f: i32, g: i32) -> i32 {
    let bob: js_sys::WebAssembly::Memory = wasm_bindgen::memory().into();
    let array = bob.buffer().dyn_into::<js_sys::ArrayBuffer>().unwrap();
    let cur = array.byte_length();
    let need: u32 = ceil((cur as f64 + requested as f64 - cur as f64) / 65536 as f64) as u32;

    bob.grow(need);

    cur as i32
}

#[allow(unused_variables)]
fn syscall_fcntl64(a: i32, b: i32, c: i32, d: i32) -> i32 {
    log(&format!("{} {} {} {}", a, b, c, d));

    0
}

#[allow(unused_variables)]
fn syscall_getpid(a: i32) -> i32 {
    0
}

#[allow(unused_variables)]
#[no_mangle]
pub extern "C" fn __syscall(a: i32, b: i32) -> i32 {
    Syscall1::from(SysCallKind::from(a.clone()))(a, b)
}

#[allow(unused_variables)]
#[no_mangle]
pub extern "C" fn __syscall0(a: i32) -> i32 {
    Syscall0::from(SysCallKind::from(a.clone()))(a)
}

#[allow(unused_variables)]
#[no_mangle]
pub extern "C" fn __syscall1(a: i32, b: i32) -> i32 {
    Syscall1::from(SysCallKind::from(a.clone()))(a, b)
}

#[allow(unused_variables)]
#[no_mangle]
pub extern "C" fn __syscall2(a: i32, b: i32, c: i32) -> i32 {
    Syscall2::from(SysCallKind::from(a.clone()))(a, b, c)
}

#[allow(unused_variables)]
#[no_mangle]
pub extern "C" fn __syscall3(a: i32, b: i32, c: i32, d: i32) -> i32 {
    return Syscall3::from(SysCallKind::from(a.clone()))(a, b, c, d);
}

#[allow(unused_variables)]
#[no_mangle]
pub extern "C" fn __syscall4(a: i32, b: i32, c: i32, d: i32, e: i32) -> i32 {
    Syscall4::from(SysCallKind::from(a.clone()))(a, b, c, d, e)
}

#[allow(unused_variables)]
#[no_mangle]
pub extern "C" fn __syscall5(a: i32, b: i32, c: i32, d: i32, e: i32, f: i32) -> i32 {
    Syscall5::from(SysCallKind::from(a.clone()))(a, b, c, d, e, f)
}

#[allow(unused_variables)]
#[no_mangle]
pub extern "C" fn __syscall6(a: i32, b: i32, c: i32, d: i32, e: i32, f: i32, g: i32) -> i32 {
    Syscall6::from(SysCallKind::from(a.clone()))(a, b, c, d, e, f, g)
}

#[allow(unused_variables)]
#[no_mangle]
pub extern "C" fn open(a: *const c_char, b: c_int, c: c_int) -> i32 {

    //   log(&format!("open {:?} {} {} ",unsafe {CStr::from_ptr(a)},b,c));
    unsafe {
        if CStr::from_ptr(a).to_str().unwrap() == "/dev/urandom" {
            return -1;
        }
    }
    0
}

#[allow(unused_variables)]
#[no_mangle]
pub extern "C" fn read(fd: c_int, buf: *mut c_char, count: c_int) -> i32 {
    log(&format!("read"));
    0
}



