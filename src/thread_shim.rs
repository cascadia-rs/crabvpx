use std::ffi::c_void;
use std::sync::{Condvar, Mutex};
use std::thread::{self, JoinHandle};

pub type pthread_t = *mut ::core::ffi::c_void;
pub type semaphore_t = *mut ::core::ffi::c_void;

// Opaque struct for semaphore
struct Semaphore {
    mutex: Mutex<u32>,
    cond: Condvar,
}

#[unsafe(no_mangle)]
pub unsafe fn vp8_semaphore_create(
    _task: u32,
    sem: *mut semaphore_t,
    _policy: i32,
    value: i32,
) -> i32 {
    unsafe {
        let semaphore = Box::new(Semaphore {
            mutex: Mutex::new(value as u32),
            cond: Condvar::new(),
        });
        *sem = Box::into_raw(semaphore) as *mut c_void;
        0 // SUCCESS
    }
}

#[unsafe(no_mangle)]
pub unsafe fn vp8_semaphore_destroy(_task: u32, sem: semaphore_t) -> i32 {
    unsafe {
        if !sem.is_null() {
            let _ = Box::from_raw(sem as *mut Semaphore);
        }
        0
    }
}

#[unsafe(no_mangle)]
pub unsafe fn vp8_semaphore_wait(sem: semaphore_t) -> i32 {
    unsafe {
        if sem.is_null() {
            return -1;
        }
        let semaphore = &*(sem as *const Semaphore);
        let mut count = semaphore.mutex.lock().unwrap();
        while *count == 0 {
            count = semaphore.cond.wait(count).unwrap();
        }
        *count -= 1;
        0
    }
}

#[unsafe(no_mangle)]
pub unsafe fn vp8_semaphore_signal(sem: semaphore_t) -> i32 {
    unsafe {
        if sem.is_null() {
            return -1;
        }
        let semaphore = &*(sem as *const Semaphore);
        let mut count = semaphore.mutex.lock().unwrap();
        *count += 1;
        semaphore.cond.notify_one();
        0
    }
}

struct ThreadHandle {
    handle: Option<JoinHandle<usize>>,
}

struct ThreadArg(*mut c_void);
unsafe impl Send for ThreadArg {}

#[unsafe(no_mangle)]
pub unsafe fn vp8_pthread_create(
    thread: *mut pthread_t,
    _attr: *const c_void,
    start_routine: Option<unsafe fn(*mut c_void) -> *mut c_void>,
    arg: *mut c_void,
) -> i32 {
    unsafe {
        if let Some(routine) = start_routine {
            let routine_ptr = routine as usize;
            let arg_ptr = arg as usize;
            let handle = thread::spawn(move || {
                eprintln!("Thread started!");
                let r: unsafe fn(*mut c_void) -> *mut c_void =
                    core::mem::transmute(routine_ptr);
                r(arg_ptr as *mut c_void) as usize
            });

            let th = Box::new(ThreadHandle {
                handle: Some(handle),
            });
            *thread = Box::into_raw(th) as *mut c_void;
            0
        } else {
            -1
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe fn vp8_pthread_join(thread: pthread_t, retval: *mut *mut c_void) -> i32 {
    unsafe {
        if thread.is_null() {
            return -1;
        }
        let mut th = Box::from_raw(thread as *mut ThreadHandle);
        if let Some(handle) = th.handle.take() {
            let res = handle.join().unwrap_or(0);
            if !retval.is_null() {
                *retval = res as *mut c_void;
            }
        }
        0
    }
}
