use std::ffi::c_void;
use std::sync::{Arc, Mutex, Condvar, OnceLock};
use std::thread::{self, JoinHandle};
use std::collections::HashMap;

pub type pthread_t = *mut c_void;


// Opaque struct for semaphore
pub struct Semaphore {
    mutex: Mutex<u32>,
    cond: Condvar,
}

impl Semaphore {
    pub fn new(value: u32) -> Self {
        Semaphore {
            mutex: Mutex::new(value),
            cond: Condvar::new(),
        }
    }

    pub fn wait(&self) {
        let mut count = self.mutex.lock().unwrap();
        while *count == 0 {
            count = self.cond.wait(count).unwrap();
        }
        *count -= 1;
    }

    pub fn signal(&self) {
        let mut count = self.mutex.lock().unwrap();
        *count += 1;
        self.cond.notify_one();
    }
}


struct ThreadHandle {
    handle: Option<JoinHandle<usize>>,
}

struct ThreadArg(*mut c_void);
unsafe impl Send for ThreadArg {}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn vp8_pthread_create(
    thread: *mut pthread_t,
    attr: *const c_void,
    start_routine: Option<unsafe extern "C" fn(*mut c_void) -> *mut c_void>,
    arg: *mut c_void,
) -> i32 {
    if let Some(routine) = start_routine {
        let routine_ptr = routine as usize;
        let arg_ptr = arg as usize;
        let handle = thread::spawn(move || {
            let r: unsafe extern "C" fn(*mut c_void) -> *mut c_void = core::mem::transmute(routine_ptr);
            r(arg_ptr as *mut c_void) as usize
        });
        
        let th = Box::new(ThreadHandle { handle: Some(handle) });
        *thread = Box::into_raw(th) as *mut c_void;
        0
    } else {
        -1
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn vp8_pthread_join(
    thread: pthread_t,
    retval: *mut *mut c_void,
) -> i32 {
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

static ONCE_MAP: OnceLock<Mutex<HashMap<usize, bool>>> = OnceLock::new();

#[unsafe(no_mangle)]
pub unsafe extern "C" fn pthread_once(
    lock: *mut c_void,
    init_routine: Option<unsafe extern "C" fn()>,
) -> i32 {
    if let Some(routine) = init_routine {
        let lock_addr = lock as usize;
        let map_mutex = ONCE_MAP.get_or_init(|| Mutex::new(HashMap::new()));
        
        let mut should_run = false;
        {
            let mut map = map_mutex.lock().unwrap();
            if !map.contains_key(&lock_addr) {
                map.insert(lock_addr, true);
                should_run = true;
            }
        }
        
        if should_run {
            routine();
        }
        0
    } else {
        -1
    }
}
