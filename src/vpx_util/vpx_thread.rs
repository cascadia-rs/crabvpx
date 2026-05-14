extern "C" {
    fn memset(
        __b: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __len: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strncpy(
        __dst: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> *mut ::core::ffi::c_char;
    fn vpx_calloc(num: size_t, size: size_t) -> *mut ::core::ffi::c_void;
    fn vpx_free(memblk: *mut ::core::ffi::c_void);
    fn pthread_cond_destroy(_: *mut pthread_cond_t) -> ::core::ffi::c_int;
    fn pthread_cond_init(
        _: *mut pthread_cond_t,
        _: *const pthread_condattr_t,
    ) -> ::core::ffi::c_int;
    fn pthread_cond_signal(_: *mut pthread_cond_t) -> ::core::ffi::c_int;
    fn pthread_cond_wait(_: *mut pthread_cond_t, _: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    fn pthread_create(
        _: *mut pthread_t,
        _: *const pthread_attr_t,
        _: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void>,
        _: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    fn pthread_join(_: pthread_t, _: *mut *mut ::core::ffi::c_void) -> ::core::ffi::c_int;
    fn pthread_mutex_destroy(_: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    fn pthread_mutex_init(
        _: *mut pthread_mutex_t,
        _: *const pthread_mutexattr_t,
    ) -> ::core::ffi::c_int;
    fn pthread_mutex_lock(_: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    fn pthread_mutex_unlock(_: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    fn pthread_setname_np(_: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
}
pub type __darwin_size_t = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __darwin_pthread_handler_rec {
    pub __routine: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    pub __arg: *mut ::core::ffi::c_void,
    pub __next: *mut __darwin_pthread_handler_rec,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _opaque_pthread_attr_t {
    pub __sig: ::core::ffi::c_long,
    pub __opaque: [::core::ffi::c_char; 56],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _opaque_pthread_cond_t {
    pub __sig: ::core::ffi::c_long,
    pub __opaque: [::core::ffi::c_char; 40],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _opaque_pthread_condattr_t {
    pub __sig: ::core::ffi::c_long,
    pub __opaque: [::core::ffi::c_char; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _opaque_pthread_mutex_t {
    pub __sig: ::core::ffi::c_long,
    pub __opaque: [::core::ffi::c_char; 56],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _opaque_pthread_mutexattr_t {
    pub __sig: ::core::ffi::c_long,
    pub __opaque: [::core::ffi::c_char; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _opaque_pthread_t {
    pub __sig: ::core::ffi::c_long,
    pub __cleanup_stack: *mut __darwin_pthread_handler_rec,
    pub __opaque: [::core::ffi::c_char; 8176],
}
pub type __darwin_pthread_attr_t = _opaque_pthread_attr_t;
pub type __darwin_pthread_cond_t = _opaque_pthread_cond_t;
pub type __darwin_pthread_condattr_t = _opaque_pthread_condattr_t;
pub type __darwin_pthread_mutex_t = _opaque_pthread_mutex_t;
pub type __darwin_pthread_mutexattr_t = _opaque_pthread_mutexattr_t;
pub type __darwin_pthread_t = *mut _opaque_pthread_t;
pub type size_t = __darwin_size_t;
pub type VPxWorkerStatus = ::core::ffi::c_uint;
pub const VPX_WORKER_STATUS_WORKING: VPxWorkerStatus = 2;
pub const VPX_WORKER_STATUS_OK: VPxWorkerStatus = 1;
pub const VPX_WORKER_STATUS_NOT_OK: VPxWorkerStatus = 0;
pub type VPxWorkerHook = Option<
    unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void) -> ::core::ffi::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VPxWorkerImpl {
    pub mutex_: pthread_mutex_t,
    pub condition_: pthread_cond_t,
    pub thread_: pthread_t,
}
pub type pthread_t = __darwin_pthread_t;
pub type pthread_cond_t = __darwin_pthread_cond_t;
pub type pthread_mutex_t = __darwin_pthread_mutex_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VPxWorker {
    pub impl_: *mut VPxWorkerImpl,
    pub status_: VPxWorkerStatus,
    pub thread_name: *const ::core::ffi::c_char,
    pub hook: VPxWorkerHook,
    pub data1: *mut ::core::ffi::c_void,
    pub data2: *mut ::core::ffi::c_void,
    pub had_error: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VPxWorkerInterface {
    pub init: Option<unsafe extern "C" fn(*mut VPxWorker) -> ()>,
    pub reset: Option<unsafe extern "C" fn(*mut VPxWorker) -> ::core::ffi::c_int>,
    pub sync: Option<unsafe extern "C" fn(*mut VPxWorker) -> ::core::ffi::c_int>,
    pub launch: Option<unsafe extern "C" fn(*mut VPxWorker) -> ()>,
    pub execute: Option<unsafe extern "C" fn(*mut VPxWorker) -> ()>,
    pub end: Option<unsafe extern "C" fn(*mut VPxWorker) -> ()>,
}
pub type pthread_attr_t = __darwin_pthread_attr_t;
pub type pthread_condattr_t = __darwin_pthread_condattr_t;
pub type pthread_mutexattr_t = __darwin_pthread_mutexattr_t;
pub const __DARWIN_NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;
pub const THREAD_EXIT_SUCCESS: *mut ::core::ffi::c_void = NULL;
unsafe extern "C" fn thread_loop(mut ptr: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void {
    let worker: *mut VPxWorker = ptr as *mut VPxWorker;
    if !(*worker).thread_name.is_null() {
        let mut thread_name: [::core::ffi::c_char; 64] = [0; 64];
        strncpy(
            &raw mut thread_name as *mut ::core::ffi::c_char,
            (*worker).thread_name,
            (::core::mem::size_of::<[::core::ffi::c_char; 64]>() as size_t)
                .wrapping_sub(1 as size_t),
        );
        thread_name[(::core::mem::size_of::<[::core::ffi::c_char; 64]>() as usize)
            .wrapping_sub(1 as usize) as usize] = '\0' as i32 as ::core::ffi::c_char;
        pthread_setname_np(
            &raw mut thread_name as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        );
    }
    pthread_mutex_lock(&raw mut (*(*worker).impl_).mutex_);
    loop {
        while (*worker).status_ as ::core::ffi::c_uint
            == VPX_WORKER_STATUS_OK as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            pthread_cond_wait(
                &raw mut (*(*worker).impl_).condition_,
                &raw mut (*(*worker).impl_).mutex_,
            );
        }
        if !((*worker).status_ as ::core::ffi::c_uint
            == VPX_WORKER_STATUS_WORKING as ::core::ffi::c_int as ::core::ffi::c_uint)
        {
            break;
        }
        pthread_mutex_unlock(&raw mut (*(*worker).impl_).mutex_);
        execute(worker);
        pthread_mutex_lock(&raw mut (*(*worker).impl_).mutex_);
        (*worker).status_ = VPX_WORKER_STATUS_OK;
        pthread_cond_signal(&raw mut (*(*worker).impl_).condition_);
    }
    pthread_mutex_unlock(&raw mut (*(*worker).impl_).mutex_);
    return THREAD_EXIT_SUCCESS;
}
unsafe extern "C" fn change_state(worker: *mut VPxWorker, mut new_status: VPxWorkerStatus) {
    if (*worker).impl_.is_null() {
        return;
    }
    pthread_mutex_lock(&raw mut (*(*worker).impl_).mutex_);
    if (*worker).status_ as ::core::ffi::c_uint
        >= VPX_WORKER_STATUS_OK as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        while (*worker).status_ as ::core::ffi::c_uint
            != VPX_WORKER_STATUS_OK as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            pthread_cond_wait(
                &raw mut (*(*worker).impl_).condition_,
                &raw mut (*(*worker).impl_).mutex_,
            );
        }
        if new_status as ::core::ffi::c_uint
            != VPX_WORKER_STATUS_OK as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            (*worker).status_ = new_status;
            pthread_cond_signal(&raw mut (*(*worker).impl_).condition_);
        }
    }
    pthread_mutex_unlock(&raw mut (*(*worker).impl_).mutex_);
}
unsafe extern "C" fn init(worker: *mut VPxWorker) {
    memset(
        worker as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<VPxWorker>() as size_t,
    );
    (*worker).status_ = VPX_WORKER_STATUS_NOT_OK;
}
unsafe extern "C" fn sync(worker: *mut VPxWorker) -> ::core::ffi::c_int {
    change_state(worker, VPX_WORKER_STATUS_OK);
    return ((*worker).had_error == 0) as ::core::ffi::c_int;
}
unsafe extern "C" fn reset(worker: *mut VPxWorker) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut ok: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    (*worker).had_error = 0 as ::core::ffi::c_int;
    if ((*worker).status_ as ::core::ffi::c_uint)
        < VPX_WORKER_STATUS_OK as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        (*worker).impl_ = vpx_calloc(
            1 as size_t,
            ::core::mem::size_of::<VPxWorkerImpl>() as size_t,
        ) as *mut VPxWorkerImpl;
        if (*worker).impl_.is_null() {
            return 0 as ::core::ffi::c_int;
        }
        if pthread_mutex_init(
            &raw mut (*(*worker).impl_).mutex_,
            ::core::ptr::null::<pthread_mutexattr_t>(),
        ) != 0
        {
            current_block = 9186383463710890590;
        } else if pthread_cond_init(
            &raw mut (*(*worker).impl_).condition_,
            ::core::ptr::null::<pthread_condattr_t>(),
        ) != 0
        {
            pthread_mutex_destroy(&raw mut (*(*worker).impl_).mutex_);
            current_block = 9186383463710890590;
        } else {
            pthread_mutex_lock(&raw mut (*(*worker).impl_).mutex_);
            ok = (pthread_create(
                &raw mut (*(*worker).impl_).thread_,
                ::core::ptr::null::<pthread_attr_t>(),
                Some(
                    thread_loop
                        as unsafe extern "C" fn(
                            *mut ::core::ffi::c_void,
                        )
                            -> *mut ::core::ffi::c_void,
                ),
                worker as *mut ::core::ffi::c_void,
            ) == 0) as ::core::ffi::c_int;
            if ok != 0 {
                (*worker).status_ = VPX_WORKER_STATUS_OK;
            }
            pthread_mutex_unlock(&raw mut (*(*worker).impl_).mutex_);
            if ok == 0 {
                pthread_mutex_destroy(&raw mut (*(*worker).impl_).mutex_);
                pthread_cond_destroy(&raw mut (*(*worker).impl_).condition_);
                current_block = 9186383463710890590;
            } else {
                current_block = 13242334135786603907;
            }
        }
        match current_block {
            13242334135786603907 => {}
            _ => {
                vpx_free((*worker).impl_ as *mut ::core::ffi::c_void);
                (*worker).impl_ = ::core::ptr::null_mut::<VPxWorkerImpl>();
                return 0 as ::core::ffi::c_int;
            }
        }
    } else if (*worker).status_ as ::core::ffi::c_uint
        > VPX_WORKER_STATUS_OK as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        ok = sync(worker);
    }
    return ok;
}
unsafe extern "C" fn execute(worker: *mut VPxWorker) {
    if (*worker).hook.is_some() {
        (*worker).had_error |=
            ((*worker).hook.expect("non-null function pointer")((*worker).data1, (*worker).data2)
                == 0) as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn launch(worker: *mut VPxWorker) {
    change_state(worker, VPX_WORKER_STATUS_WORKING);
}
unsafe extern "C" fn end(worker: *mut VPxWorker) {
    if !(*worker).impl_.is_null() {
        change_state(worker, VPX_WORKER_STATUS_NOT_OK);
        pthread_join(
            (*(*worker).impl_).thread_ as pthread_t,
            ::core::ptr::null_mut::<*mut ::core::ffi::c_void>(),
        );
        pthread_mutex_destroy(&raw mut (*(*worker).impl_).mutex_);
        pthread_cond_destroy(&raw mut (*(*worker).impl_).condition_);
        vpx_free((*worker).impl_ as *mut ::core::ffi::c_void);
        (*worker).impl_ = ::core::ptr::null_mut::<VPxWorkerImpl>();
    }
}
static mut g_worker_interface: VPxWorkerInterface = unsafe {
    VPxWorkerInterface {
        init: Some(init as unsafe extern "C" fn(*mut VPxWorker) -> ()),
        reset: Some(reset as unsafe extern "C" fn(*mut VPxWorker) -> ::core::ffi::c_int),
        sync: Some(sync as unsafe extern "C" fn(*mut VPxWorker) -> ::core::ffi::c_int),
        launch: Some(launch as unsafe extern "C" fn(*mut VPxWorker) -> ()),
        execute: Some(execute as unsafe extern "C" fn(*mut VPxWorker) -> ()),
        end: Some(end as unsafe extern "C" fn(*mut VPxWorker) -> ()),
    }
};
#[no_mangle]
pub unsafe extern "C" fn vpx_set_worker_interface(
    winterface: *const VPxWorkerInterface,
) -> ::core::ffi::c_int {
    if winterface.is_null()
        || (*winterface).init.is_none()
        || (*winterface).reset.is_none()
        || (*winterface).sync.is_none()
        || (*winterface).launch.is_none()
        || (*winterface).execute.is_none()
        || (*winterface).end.is_none()
    {
        return 0 as ::core::ffi::c_int;
    }
    g_worker_interface = *winterface;
    return 1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn vpx_get_worker_interface() -> *const VPxWorkerInterface {
    return &raw mut g_worker_interface;
}
