#![allow(non_camel_case_types, non_snake_case, unused, non_upper_case_globals)]

// Autogenerated binding to bdwgc C library

#[derive(Default, Clone)]
pub struct Bdwgc_allocator{}

unsafe impl core::alloc::Allocator for Bdwgc_allocator{
    fn allocate(&self, layout: core::alloc::Layout) -> Result<core::ptr::NonNull<[u8]>, core::alloc::AllocError> {
        unsafe{
            let p = GC_malloc(layout.size()) as *mut u8;
            let s = core::slice::from_raw_parts_mut(p, layout.size());
            return Ok(core::ptr::NonNull::new_unchecked(s))
        }
    }
    fn allocate_zeroed(&self, layout: core::alloc::Layout) -> Result<core::ptr::NonNull<[u8]>, core::alloc::AllocError> {
        self.allocate(layout)
    }
    unsafe fn deallocate(&self, ptr: core::ptr::NonNull<u8>, layout: core::alloc::Layout) {
        GC_free(ptr.as_ptr() as _);
    }
    unsafe fn grow(
            &self,
            ptr: core::ptr::NonNull<u8>,
            old_layout: core::alloc::Layout,
            new_layout: core::alloc::Layout,
        ) -> Result<core::ptr::NonNull<[u8]>, core::alloc::AllocError> {
        unsafe{
            let p = GC_realloc(ptr.as_ptr() as _, new_layout.size()) as *mut u8;
            let s = core::slice::from_raw_parts_mut(p, new_layout.size());
            return Ok(core::ptr::NonNull::new_unchecked(s))
        }
    }
    unsafe fn grow_zeroed(
            &self,
            ptr: core::ptr::NonNull<u8>,
            old_layout: core::alloc::Layout,
            new_layout: core::alloc::Layout,
        ) -> Result<core::ptr::NonNull<[u8]>, core::alloc::AllocError> {
        return self.grow(ptr, old_layout, new_layout)
    }
}



/* automatically generated by rust-bindgen 0.65.1 */

pub const GC_TMP_VERSION_MAJOR: u32 = 8;
pub const GC_TMP_VERSION_MINOR: u32 = 3;
pub const GC_TMP_VERSION_MICRO: u32 = 0;
pub const GC_VERSION_MAJOR: u32 = 8;
pub const GC_VERSION_MINOR: u32 = 3;
pub const GC_VERSION_MICRO: u32 = 0;
pub const GC_TIME_UNLIMITED: u32 = 999999;
pub const GC_PROTECTS_POINTER_HEAP: u32 = 1;
pub const GC_PROTECTS_PTRFREE_HEAP: u32 = 2;
pub const GC_PROTECTS_STATIC_DATA: u32 = 4;
pub const GC_PROTECTS_STACK: u32 = 8;
pub const GC_PROTECTS_NONE: u32 = 0;
pub const GC_NO_MEMORY: u32 = 2;
pub const GC_SUCCESS: u32 = 0;
pub const GC_DUPLICATE: u32 = 1;
pub const GC_NO_THREADS: u32 = 2;
pub const GC_UNIMPLEMENTED: u32 = 3;
pub const GC_NOT_FOUND: u32 = 4;
pub type GC_PTR = *mut libc::c_void;
pub type GC_word = libc::c_ulonglong;
pub type GC_signed_word = libc::c_longlong;

extern "C" {
    pub fn GC_get_version() -> libc::c_uint;
}
extern "C" {
    pub static mut GC_gc_no: GC_word;
}
extern "C" {
    pub fn GC_get_gc_no() -> GC_word;
}
extern "C" {
    pub fn GC_get_parallel() -> libc::c_int;
}
extern "C" {
    pub fn GC_set_markers_count(arg1: libc::c_uint);
}
pub type GC_oom_func =
    ::core::option::Option<unsafe extern "C" fn(arg1: usize) -> *mut libc::c_void>;
extern "C" {
    pub static mut GC_oom_fn: GC_oom_func;
}
extern "C" {
    pub fn GC_set_oom_fn(arg1: GC_oom_func);
}
extern "C" {
    pub fn GC_get_oom_fn() -> GC_oom_func;
}
pub type GC_on_heap_resize_proc = ::core::option::Option<unsafe extern "C" fn(arg1: GC_word)>;

extern "C" {
    pub static mut GC_on_heap_resize: GC_on_heap_resize_proc;
}
extern "C" {
    pub fn GC_set_on_heap_resize(arg1: GC_on_heap_resize_proc);
}
extern "C" {
    pub fn GC_get_on_heap_resize() -> GC_on_heap_resize_proc;
}
pub const GC_EventType_GC_EVENT_START: GC_EventType = 0;
pub const GC_EventType_GC_EVENT_MARK_START: GC_EventType = 1;
pub const GC_EventType_GC_EVENT_MARK_END: GC_EventType = 2;
pub const GC_EventType_GC_EVENT_RECLAIM_START: GC_EventType = 3;
pub const GC_EventType_GC_EVENT_RECLAIM_END: GC_EventType = 4;
pub const GC_EventType_GC_EVENT_END: GC_EventType = 5;
pub const GC_EventType_GC_EVENT_PRE_STOP_WORLD: GC_EventType = 6;
pub const GC_EventType_GC_EVENT_POST_STOP_WORLD: GC_EventType = 7;
pub const GC_EventType_GC_EVENT_PRE_START_WORLD: GC_EventType = 8;
pub const GC_EventType_GC_EVENT_POST_START_WORLD: GC_EventType = 9;
pub const GC_EventType_GC_EVENT_THREAD_SUSPENDED: GC_EventType = 10;
pub const GC_EventType_GC_EVENT_THREAD_UNSUSPENDED: GC_EventType = 11;
pub type GC_EventType = libc::c_int;
pub type GC_on_collection_event_proc =
    ::core::option::Option<unsafe extern "C" fn(arg1: GC_EventType)>;
extern "C" {
    pub fn GC_set_on_collection_event(arg1: GC_on_collection_event_proc);
}
extern "C" {
    pub fn GC_get_on_collection_event() -> GC_on_collection_event_proc;
}
extern "C" {
    pub static mut GC_find_leak: libc::c_int;
}
extern "C" {
    pub fn GC_set_find_leak(arg1: libc::c_int);
}
extern "C" {
    pub fn GC_get_find_leak() -> libc::c_int;
}
extern "C" {
    pub static mut GC_all_interior_pointers: libc::c_int;
}
extern "C" {
    pub fn GC_set_all_interior_pointers(arg1: libc::c_int);
}
extern "C" {
    pub fn GC_get_all_interior_pointers() -> libc::c_int;
}
extern "C" {
    pub static mut GC_finalize_on_demand: libc::c_int;
}
extern "C" {
    pub fn GC_set_finalize_on_demand(arg1: libc::c_int);
}
extern "C" {
    pub fn GC_get_finalize_on_demand() -> libc::c_int;
}
extern "C" {
    pub static mut GC_java_finalization: libc::c_int;
}
extern "C" {
    pub fn GC_set_java_finalization(arg1: libc::c_int);
}
extern "C" {
    pub fn GC_get_java_finalization() -> libc::c_int;
}
pub type GC_finalizer_notifier_proc = ::core::option::Option<unsafe extern "C" fn()>;
extern "C" {
    pub static mut GC_finalizer_notifier: GC_finalizer_notifier_proc;
}
extern "C" {
    pub fn GC_set_finalizer_notifier(arg1: GC_finalizer_notifier_proc);
}
extern "C" {
    pub fn GC_get_finalizer_notifier() -> GC_finalizer_notifier_proc;
}
pub type GC_valid_ptr_print_proc_t =
    ::core::option::Option<unsafe extern "C" fn(arg1: *mut libc::c_void)>;
pub type GC_same_obj_print_proc_t = ::core::option::Option<
    unsafe extern "C" fn(arg1: *mut libc::c_void, arg2: *mut libc::c_void),
>;
extern "C" {
    pub static mut GC_same_obj_print_proc: GC_same_obj_print_proc_t;
}
extern "C" {
    pub static mut GC_is_valid_displacement_print_proc: GC_valid_ptr_print_proc_t;
}
extern "C" {
    pub static mut GC_is_visible_print_proc: GC_valid_ptr_print_proc_t;
}
extern "C" {
    pub fn GC_set_same_obj_print_proc(arg1: GC_same_obj_print_proc_t);
}
extern "C" {
    pub fn GC_get_same_obj_print_proc() -> GC_same_obj_print_proc_t;
}
extern "C" {
    pub fn GC_set_is_valid_displacement_print_proc(arg1: GC_valid_ptr_print_proc_t);
}
extern "C" {
    pub fn GC_get_is_valid_displacement_print_proc() -> GC_valid_ptr_print_proc_t;
}
extern "C" {
    pub fn GC_set_is_visible_print_proc(arg1: GC_valid_ptr_print_proc_t);
}
extern "C" {
    pub fn GC_get_is_visible_print_proc() -> GC_valid_ptr_print_proc_t;
}
extern "C" {
    pub static mut GC_dont_gc: libc::c_int;
}
extern "C" {
    pub static mut GC_dont_expand: libc::c_int;
}
extern "C" {
    pub fn GC_set_dont_expand(arg1: libc::c_int);
}
extern "C" {
    pub fn GC_get_dont_expand() -> libc::c_int;
}
extern "C" {
    pub static mut GC_use_entire_heap: libc::c_int;
}
extern "C" {
    pub static mut GC_full_freq: libc::c_int;
}
extern "C" {
    pub fn GC_set_full_freq(arg1: libc::c_int);
}
extern "C" {
    pub fn GC_get_full_freq() -> libc::c_int;
}
extern "C" {
    pub static mut GC_non_gc_bytes: GC_word;
}
extern "C" {
    pub fn GC_set_non_gc_bytes(arg1: GC_word);
}
extern "C" {
    pub fn GC_get_non_gc_bytes() -> GC_word;
}
extern "C" {
    pub static mut GC_no_dls: libc::c_int;
}
extern "C" {
    pub fn GC_set_no_dls(arg1: libc::c_int);
}
extern "C" {
    pub fn GC_get_no_dls() -> libc::c_int;
}
extern "C" {
    pub static mut GC_free_space_divisor: GC_word;
}
extern "C" {
    pub fn GC_set_free_space_divisor(arg1: GC_word);
}
extern "C" {
    pub fn GC_get_free_space_divisor() -> GC_word;
}
extern "C" {
    pub static mut GC_max_retries: GC_word;
}
extern "C" {
    pub fn GC_set_max_retries(arg1: GC_word);
}
extern "C" {
    pub fn GC_get_max_retries() -> GC_word;
}
extern "C" {
    pub static mut GC_stackbottom: *mut libc::c_char;
}
extern "C" {
    pub static mut GC_dont_precollect: libc::c_int;
}
extern "C" {
    pub fn GC_set_dont_precollect(arg1: libc::c_int);
}
extern "C" {
    pub fn GC_get_dont_precollect() -> libc::c_int;
}
extern "C" {
    pub static mut GC_time_limit: libc::c_ulong;
}
extern "C" {
    pub fn GC_set_time_limit(arg1: libc::c_ulong);
}
extern "C" {
    pub fn GC_get_time_limit() -> libc::c_ulong;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GC_timeval_s {
    pub tv_ms: libc::c_ulong,
    pub tv_nsec: libc::c_ulong,
}
#[test]
fn bindgen_test_layout_GC_timeval_s() {
    const UNINIT: ::core::mem::MaybeUninit<GC_timeval_s> = ::core::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::core::mem::size_of::<GC_timeval_s>(),
        8usize,
        concat!("Size of: ", stringify!(GC_timeval_s))
    );
    assert_eq!(
        ::core::mem::align_of::<GC_timeval_s>(),
        4usize,
        concat!("Alignment of ", stringify!(GC_timeval_s))
    );
    assert_eq!(
        unsafe { ::core::ptr::addr_of!((*ptr).tv_ms) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(GC_timeval_s),
            "::",
            stringify!(tv_ms)
        )
    );
    assert_eq!(
        unsafe { ::core::ptr::addr_of!((*ptr).tv_nsec) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(GC_timeval_s),
            "::",
            stringify!(tv_nsec)
        )
    );
}
extern "C" {
    pub fn GC_set_time_limit_tv(arg1: GC_timeval_s);
}
extern "C" {
    pub fn GC_get_time_limit_tv() -> GC_timeval_s;
}
extern "C" {
    pub fn GC_set_allocd_bytes_per_finalizer(arg1: GC_word);
}
extern "C" {
    pub fn GC_get_allocd_bytes_per_finalizer() -> GC_word;
}
extern "C" {
    pub fn GC_start_performance_measurement();
}
extern "C" {
    pub fn GC_get_full_gc_total_time() -> libc::c_ulong;
}
extern "C" {
    pub fn GC_get_stopped_mark_total_time() -> libc::c_ulong;
}
extern "C" {
    pub fn GC_set_pages_executable(arg1: libc::c_int);
}
extern "C" {
    pub fn GC_get_pages_executable() -> libc::c_int;
}
extern "C" {
    pub fn GC_set_min_bytes_allocd(arg1: usize);
}
extern "C" {
    pub fn GC_get_min_bytes_allocd() -> usize;
}
extern "C" {
    pub fn GC_set_rate(arg1: libc::c_int);
}
extern "C" {
    pub fn GC_get_rate() -> libc::c_int;
}
extern "C" {
    pub fn GC_set_max_prior_attempts(arg1: libc::c_int);
}
extern "C" {
    pub fn GC_get_max_prior_attempts() -> libc::c_int;
}
extern "C" {
    pub fn GC_set_disable_automatic_collection(arg1: libc::c_int);
}
extern "C" {
    pub fn GC_get_disable_automatic_collection() -> libc::c_int;
}
extern "C" {
    pub fn GC_set_handle_fork(arg1: libc::c_int);
}
extern "C" {
    pub fn GC_atfork_prepare();
}
extern "C" {
    pub fn GC_atfork_parent();
}
extern "C" {
    pub fn GC_atfork_child();
}
extern "C" {
    pub fn GC_init();
}
extern "C" {
    pub fn GC_is_init_called() -> libc::c_int;
}
extern "C" {
    pub fn GC_deinit();
}
extern "C" {
    pub fn GC_malloc(arg1: usize) -> *mut libc::c_void;
}
extern "C" {
    pub fn GC_malloc_atomic(arg1: usize) -> *mut libc::c_void;
}
extern "C" {
    pub fn GC_strdup(arg1: *const libc::c_char) -> *mut libc::c_char;
}
extern "C" {
    pub fn GC_strndup(
        arg1: *const libc::c_char,
        arg2: usize,
    ) -> *mut libc::c_char;
}
extern "C" {
    pub fn GC_malloc_uncollectable(arg1: usize) -> *mut libc::c_void;
}
extern "C" {
    pub fn GC_malloc_stubborn(arg1: usize) -> *mut libc::c_void;
}
extern "C" {
    pub fn GC_memalign(arg1: usize, arg2: usize) -> *mut libc::c_void;
}
extern "C" {
    pub fn GC_posix_memalign(
        arg1: *mut *mut libc::c_void,
        arg2: usize,
        arg3: usize,
    ) -> libc::c_int;
}
extern "C" {
    pub fn GC_valloc(arg1: usize) -> *mut libc::c_void;
}
extern "C" {
    pub fn GC_pvalloc(arg1: usize) -> *mut libc::c_void;
}
extern "C" {
    pub fn GC_free(arg1: *mut libc::c_void);
}
extern "C" {
    pub fn GC_change_stubborn(arg1: *const libc::c_void);
}
extern "C" {
    pub fn GC_end_stubborn_change(arg1: *const libc::c_void);
}
extern "C" {
    pub fn GC_base(arg1: *mut libc::c_void) -> *mut libc::c_void;
}
extern "C" {
    pub fn GC_is_heap_ptr(arg1: *const libc::c_void) -> libc::c_int;
}
extern "C" {
    pub fn GC_size(arg1: *const libc::c_void) -> usize;
}
extern "C" {
    pub fn GC_realloc(
        arg1: *mut libc::c_void,
        arg2: usize,
    ) -> *mut libc::c_void;
}
extern "C" {
    pub fn GC_expand_hp(arg1: usize) -> libc::c_int;
}
extern "C" {
    pub fn GC_set_max_heap_size(arg1: GC_word);
}
extern "C" {
    pub fn GC_exclude_static_roots(
        arg1: *mut libc::c_void,
        arg2: *mut libc::c_void,
    );
}
extern "C" {
    pub fn GC_clear_exclusion_table();
}
extern "C" {
    pub fn GC_clear_roots();
}
extern "C" {
    pub fn GC_add_roots(arg1: *mut libc::c_void, arg2: *mut libc::c_void);
}
extern "C" {
    pub fn GC_remove_roots(arg1: *mut libc::c_void, arg2: *mut libc::c_void);
}
extern "C" {
    pub fn GC_register_displacement(arg1: usize);
}
extern "C" {
    pub fn GC_debug_register_displacement(arg1: usize);
}
extern "C" {
    pub fn GC_gcollect();
}
extern "C" {
    pub fn GC_gcollect_and_unmap();
}
pub type GC_stop_func = ::core::option::Option<unsafe extern "C" fn() -> libc::c_int>;
extern "C" {
    pub fn GC_try_to_collect(arg1: GC_stop_func) -> libc::c_int;
}
extern "C" {
    pub fn GC_set_stop_func(arg1: GC_stop_func);
}
extern "C" {
    pub fn GC_get_stop_func() -> GC_stop_func;
}
extern "C" {
    pub fn GC_get_heap_size() -> usize;
}
extern "C" {
    pub fn GC_get_free_bytes() -> usize;
}
extern "C" {
    pub fn GC_get_unmapped_bytes() -> usize;
}
extern "C" {
    pub fn GC_get_bytes_since_gc() -> usize;
}
extern "C" {
    pub fn GC_get_expl_freed_bytes_since_gc() -> usize;
}
extern "C" {
    pub fn GC_get_total_bytes() -> usize;
}
extern "C" {
    pub fn GC_get_obtained_from_os_bytes() -> usize;
}
extern "C" {
    pub fn GC_get_heap_usage_safe(
        arg1: *mut GC_word,
        arg2: *mut GC_word,
        arg3: *mut GC_word,
        arg4: *mut GC_word,
        arg5: *mut GC_word,
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GC_prof_stats_s {
    pub heapsize_full: GC_word,
    pub free_bytes_full: GC_word,
    pub unmapped_bytes: GC_word,
    pub bytes_allocd_since_gc: GC_word,
    pub allocd_bytes_before_gc: GC_word,
    pub non_gc_bytes: GC_word,
    pub gc_no: GC_word,
    pub markers_m1: GC_word,
    pub bytes_reclaimed_since_gc: GC_word,
    pub reclaimed_bytes_before_gc: GC_word,
    pub expl_freed_bytes_since_gc: GC_word,
    pub obtained_from_os_bytes: GC_word,
}
#[test]
fn bindgen_test_layout_GC_prof_stats_s() {
    const UNINIT: ::core::mem::MaybeUninit<GC_prof_stats_s> = ::core::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::core::mem::size_of::<GC_prof_stats_s>(),
        96usize,
        concat!("Size of: ", stringify!(GC_prof_stats_s))
    );
    assert_eq!(
        ::core::mem::align_of::<GC_prof_stats_s>(),
        8usize,
        concat!("Alignment of ", stringify!(GC_prof_stats_s))
    );
    assert_eq!(
        unsafe { ::core::ptr::addr_of!((*ptr).heapsize_full) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(GC_prof_stats_s),
            "::",
            stringify!(heapsize_full)
        )
    );
    assert_eq!(
        unsafe { ::core::ptr::addr_of!((*ptr).free_bytes_full) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(GC_prof_stats_s),
            "::",
            stringify!(free_bytes_full)
        )
    );
    assert_eq!(
        unsafe { ::core::ptr::addr_of!((*ptr).unmapped_bytes) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(GC_prof_stats_s),
            "::",
            stringify!(unmapped_bytes)
        )
    );
    assert_eq!(
        unsafe { ::core::ptr::addr_of!((*ptr).bytes_allocd_since_gc) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(GC_prof_stats_s),
            "::",
            stringify!(bytes_allocd_since_gc)
        )
    );
    assert_eq!(
        unsafe { ::core::ptr::addr_of!((*ptr).allocd_bytes_before_gc) as usize - ptr as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(GC_prof_stats_s),
            "::",
            stringify!(allocd_bytes_before_gc)
        )
    );
    assert_eq!(
        unsafe { ::core::ptr::addr_of!((*ptr).non_gc_bytes) as usize - ptr as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(GC_prof_stats_s),
            "::",
            stringify!(non_gc_bytes)
        )
    );
    assert_eq!(
        unsafe { ::core::ptr::addr_of!((*ptr).gc_no) as usize - ptr as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(GC_prof_stats_s),
            "::",
            stringify!(gc_no)
        )
    );
    assert_eq!(
        unsafe { ::core::ptr::addr_of!((*ptr).markers_m1) as usize - ptr as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(GC_prof_stats_s),
            "::",
            stringify!(markers_m1)
        )
    );
    assert_eq!(
        unsafe { ::core::ptr::addr_of!((*ptr).bytes_reclaimed_since_gc) as usize - ptr as usize },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(GC_prof_stats_s),
            "::",
            stringify!(bytes_reclaimed_since_gc)
        )
    );
    assert_eq!(
        unsafe { ::core::ptr::addr_of!((*ptr).reclaimed_bytes_before_gc) as usize - ptr as usize },
        72usize,
        concat!(
            "Offset of field: ",
            stringify!(GC_prof_stats_s),
            "::",
            stringify!(reclaimed_bytes_before_gc)
        )
    );
    assert_eq!(
        unsafe { ::core::ptr::addr_of!((*ptr).expl_freed_bytes_since_gc) as usize - ptr as usize },
        80usize,
        concat!(
            "Offset of field: ",
            stringify!(GC_prof_stats_s),
            "::",
            stringify!(expl_freed_bytes_since_gc)
        )
    );
    assert_eq!(
        unsafe { ::core::ptr::addr_of!((*ptr).obtained_from_os_bytes) as usize - ptr as usize },
        88usize,
        concat!(
            "Offset of field: ",
            stringify!(GC_prof_stats_s),
            "::",
            stringify!(obtained_from_os_bytes)
        )
    );
}
extern "C" {
    pub fn GC_get_prof_stats(arg1: *mut GC_prof_stats_s, arg2: usize) -> usize;
}
extern "C" {
    pub fn GC_get_size_map_at(i: libc::c_int) -> usize;
}
extern "C" {
    pub fn GC_get_memory_use() -> usize;
}
extern "C" {
    pub fn GC_disable();
}
extern "C" {
    pub fn GC_is_disabled() -> libc::c_int;
}
extern "C" {
    pub fn GC_enable();
}
extern "C" {
    pub fn GC_set_manual_vdb_allowed(arg1: libc::c_int);
}
extern "C" {
    pub fn GC_get_manual_vdb_allowed() -> libc::c_int;
}
extern "C" {
    pub fn GC_enable_incremental();
}
extern "C" {
    pub fn GC_is_incremental_mode() -> libc::c_int;
}
extern "C" {
    pub fn GC_incremental_protection_needs() -> libc::c_int;
}
extern "C" {
    pub fn GC_start_incremental_collection();
}
extern "C" {
    pub fn GC_collect_a_little() -> libc::c_int;
}
extern "C" {
    pub fn GC_malloc_ignore_off_page(arg1: usize) -> *mut libc::c_void;
}
extern "C" {
    pub fn GC_malloc_atomic_ignore_off_page(arg1: usize) -> *mut libc::c_void;
}
extern "C" {
    pub fn GC_malloc_atomic_uncollectable(arg1: usize) -> *mut libc::c_void;
}
extern "C" {
    pub fn GC_debug_malloc_atomic_uncollectable(
        arg1: usize,
        s: *const libc::c_char,
        i: libc::c_int,
    ) -> *mut libc::c_void;
}
extern "C" {
    pub fn GC_debug_malloc(
        arg1: usize,
        s: *const libc::c_char,
        i: libc::c_int,
    ) -> *mut libc::c_void;
}
extern "C" {
    pub fn GC_debug_malloc_atomic(
        arg1: usize,
        s: *const libc::c_char,
        i: libc::c_int,
    ) -> *mut libc::c_void;
}
extern "C" {
    pub fn GC_debug_strdup(
        arg1: *const libc::c_char,
        s: *const libc::c_char,
        i: libc::c_int,
    ) -> *mut libc::c_char;
}
extern "C" {
    pub fn GC_debug_strndup(
        arg1: *const libc::c_char,
        arg2: usize,
        s: *const libc::c_char,
        i: libc::c_int,
    ) -> *mut libc::c_char;
}
extern "C" {
    pub fn GC_debug_malloc_uncollectable(
        arg1: usize,
        s: *const libc::c_char,
        i: libc::c_int,
    ) -> *mut libc::c_void;
}
extern "C" {
    pub fn GC_debug_malloc_stubborn(
        arg1: usize,
        s: *const libc::c_char,
        i: libc::c_int,
    ) -> *mut libc::c_void;
}
extern "C" {
    pub fn GC_debug_malloc_ignore_off_page(
        arg1: usize,
        s: *const libc::c_char,
        i: libc::c_int,
    ) -> *mut libc::c_void;
}
extern "C" {
    pub fn GC_debug_malloc_atomic_ignore_off_page(
        arg1: usize,
        s: *const libc::c_char,
        i: libc::c_int,
    ) -> *mut libc::c_void;
}
extern "C" {
    pub fn GC_debug_free(arg1: *mut libc::c_void);
}
extern "C" {
    pub fn GC_debug_realloc(
        arg1: *mut libc::c_void,
        arg2: usize,
        s: *const libc::c_char,
        i: libc::c_int,
    ) -> *mut libc::c_void;
}
extern "C" {
    pub fn GC_debug_change_stubborn(arg1: *const libc::c_void);
}
extern "C" {
    pub fn GC_debug_end_stubborn_change(arg1: *const libc::c_void);
}
extern "C" {
    pub fn GC_debug_malloc_replacement(arg1: usize) -> *mut libc::c_void;
}
extern "C" {
    pub fn GC_debug_realloc_replacement(
        arg1: *mut libc::c_void,
        arg2: usize,
    ) -> *mut libc::c_void;
}
pub type GC_finalization_proc = ::core::option::Option<
    unsafe extern "C" fn(arg1: *mut libc::c_void, arg2: *mut libc::c_void),
>;
extern "C" {
    pub fn GC_register_finalizer(
        arg1: *mut libc::c_void,
        arg2: GC_finalization_proc,
        arg3: *mut libc::c_void,
        arg4: *mut GC_finalization_proc,
        arg5: *mut *mut libc::c_void,
    );
}
extern "C" {
    pub fn GC_debug_register_finalizer(
        arg1: *mut libc::c_void,
        arg2: GC_finalization_proc,
        arg3: *mut libc::c_void,
        arg4: *mut GC_finalization_proc,
        arg5: *mut *mut libc::c_void,
    );
}
extern "C" {
    pub fn GC_register_finalizer_ignore_self(
        arg1: *mut libc::c_void,
        arg2: GC_finalization_proc,
        arg3: *mut libc::c_void,
        arg4: *mut GC_finalization_proc,
        arg5: *mut *mut libc::c_void,
    );
}
extern "C" {
    pub fn GC_debug_register_finalizer_ignore_self(
        arg1: *mut libc::c_void,
        arg2: GC_finalization_proc,
        arg3: *mut libc::c_void,
        arg4: *mut GC_finalization_proc,
        arg5: *mut *mut libc::c_void,
    );
}
extern "C" {
    pub fn GC_register_finalizer_no_order(
        arg1: *mut libc::c_void,
        arg2: GC_finalization_proc,
        arg3: *mut libc::c_void,
        arg4: *mut GC_finalization_proc,
        arg5: *mut *mut libc::c_void,
    );
}
extern "C" {
    pub fn GC_debug_register_finalizer_no_order(
        arg1: *mut libc::c_void,
        arg2: GC_finalization_proc,
        arg3: *mut libc::c_void,
        arg4: *mut GC_finalization_proc,
        arg5: *mut *mut libc::c_void,
    );
}
extern "C" {
    pub fn GC_register_finalizer_unreachable(
        arg1: *mut libc::c_void,
        arg2: GC_finalization_proc,
        arg3: *mut libc::c_void,
        arg4: *mut GC_finalization_proc,
        arg5: *mut *mut libc::c_void,
    );
}
extern "C" {
    pub fn GC_debug_register_finalizer_unreachable(
        arg1: *mut libc::c_void,
        arg2: GC_finalization_proc,
        arg3: *mut libc::c_void,
        arg4: *mut GC_finalization_proc,
        arg5: *mut *mut libc::c_void,
    );
}
extern "C" {
    pub fn GC_register_disappearing_link(
        arg1: *mut *mut libc::c_void,
    ) -> libc::c_int;
}
extern "C" {
    pub fn GC_general_register_disappearing_link(
        arg1: *mut *mut libc::c_void,
        arg2: *const libc::c_void,
    ) -> libc::c_int;
}
extern "C" {
    pub fn GC_move_disappearing_link(
        arg1: *mut *mut libc::c_void,
        arg2: *mut *mut libc::c_void,
    ) -> libc::c_int;
}
extern "C" {
    pub fn GC_unregister_disappearing_link(
        arg1: *mut *mut libc::c_void,
    ) -> libc::c_int;
}
extern "C" {
    pub fn GC_register_long_link(
        arg1: *mut *mut libc::c_void,
        arg2: *const libc::c_void,
    ) -> libc::c_int;
}
extern "C" {
    pub fn GC_move_long_link(
        arg1: *mut *mut libc::c_void,
        arg2: *mut *mut libc::c_void,
    ) -> libc::c_int;
}
extern "C" {
    pub fn GC_unregister_long_link(arg1: *mut *mut libc::c_void)
        -> libc::c_int;
}
pub const GC_ToggleRefStatus_GC_TOGGLE_REF_DROP: GC_ToggleRefStatus = 0;
pub const GC_ToggleRefStatus_GC_TOGGLE_REF_STRONG: GC_ToggleRefStatus = 1;
pub const GC_ToggleRefStatus_GC_TOGGLE_REF_WEAK: GC_ToggleRefStatus = 2;
pub type GC_ToggleRefStatus = libc::c_int;
pub type GC_toggleref_func = ::core::option::Option<
    unsafe extern "C" fn(arg1: *mut libc::c_void) -> GC_ToggleRefStatus,
>;
extern "C" {
    pub fn GC_set_toggleref_func(arg1: GC_toggleref_func);
}
extern "C" {
    pub fn GC_get_toggleref_func() -> GC_toggleref_func;
}
extern "C" {
    pub fn GC_toggleref_add(
        arg1: *mut libc::c_void,
        arg2: libc::c_int,
    ) -> libc::c_int;
}
pub type GC_await_finalize_proc =
    ::core::option::Option<unsafe extern "C" fn(arg1: *mut libc::c_void)>;
extern "C" {
    pub fn GC_set_await_finalize_proc(arg1: GC_await_finalize_proc);
}
extern "C" {
    pub fn GC_get_await_finalize_proc() -> GC_await_finalize_proc;
}
extern "C" {
    pub fn GC_should_invoke_finalizers() -> libc::c_int;
}
extern "C" {
    pub fn GC_set_interrupt_finalizers(arg1: libc::c_uint);
}
extern "C" {
    pub fn GC_get_interrupt_finalizers() -> libc::c_uint;
}
extern "C" {
    pub fn GC_invoke_finalizers() -> libc::c_int;
}
extern "C" {
    pub fn GC_noop1(arg1: GC_word);
}
pub type GC_warn_proc =
    ::core::option::Option<unsafe extern "C" fn(arg1: *mut libc::c_char, arg2: GC_word)>;
extern "C" {
    pub fn GC_set_warn_proc(arg1: GC_warn_proc);
}
extern "C" {
    pub fn GC_get_warn_proc() -> GC_warn_proc;
}
extern "C" {
    pub fn GC_ignore_warn_proc(arg1: *mut libc::c_char, arg2: GC_word);
}
extern "C" {
    pub fn GC_set_log_fd(arg1: libc::c_int);
}
pub type GC_abort_func =
    ::core::option::Option<unsafe extern "C" fn(arg1: *const libc::c_char)>;
extern "C" {
    pub fn GC_set_abort_func(arg1: GC_abort_func);
}
extern "C" {
    pub fn GC_get_abort_func() -> GC_abort_func;
}
extern "C" {
    pub fn GC_abort_on_oom();
}
pub type GC_hidden_pointer = GC_word;
pub type GC_fn_type = ::core::option::Option<
    unsafe extern "C" fn(arg1: *mut libc::c_void) -> *mut libc::c_void,
>;
extern "C" {
    pub fn GC_call_with_alloc_lock(
        arg1: GC_fn_type,
        arg2: *mut libc::c_void,
    ) -> *mut libc::c_void;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GC_stack_base {
    pub mem_base: *mut libc::c_void,
}
#[test]
fn bindgen_test_layout_GC_stack_base() {
    const UNINIT: ::core::mem::MaybeUninit<GC_stack_base> = ::core::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::core::mem::size_of::<GC_stack_base>(),
        8usize,
        concat!("Size of: ", stringify!(GC_stack_base))
    );
    assert_eq!(
        ::core::mem::align_of::<GC_stack_base>(),
        8usize,
        concat!("Alignment of ", stringify!(GC_stack_base))
    );
    assert_eq!(
        unsafe { ::core::ptr::addr_of!((*ptr).mem_base) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(GC_stack_base),
            "::",
            stringify!(mem_base)
        )
    );
}
pub type GC_stack_base_func = ::core::option::Option<
    unsafe extern "C" fn(
        arg1: *mut GC_stack_base,
        arg2: *mut libc::c_void,
    ) -> *mut libc::c_void,
>;
extern "C" {
    pub fn GC_call_with_stack_base(
        arg1: GC_stack_base_func,
        arg2: *mut libc::c_void,
    ) -> *mut libc::c_void;
}
extern "C" {
    pub fn GC_start_mark_threads();
}
extern "C" {
    pub fn GC_do_blocking(
        arg1: GC_fn_type,
        arg2: *mut libc::c_void,
    ) -> *mut libc::c_void;
}
extern "C" {
    pub fn GC_call_with_gc_active(
        arg1: GC_fn_type,
        arg2: *mut libc::c_void,
    ) -> *mut libc::c_void;
}
extern "C" {
    pub fn GC_get_stack_base(arg1: *mut GC_stack_base) -> libc::c_int;
}
extern "C" {
    pub fn GC_get_my_stackbottom(arg1: *mut GC_stack_base) -> *mut libc::c_void;
}
extern "C" {
    pub fn GC_set_stackbottom(arg1: *mut libc::c_void, arg2: *const GC_stack_base);
}
extern "C" {
    pub fn GC_pre_incr(
        arg1: *mut *mut libc::c_void,
        ptrdiff_t: libc::c_int,
    ) -> *mut libc::c_void;
}
extern "C" {
    pub fn GC_post_incr(
        arg1: *mut *mut libc::c_void,
        ptrdiff_t: libc::c_int,
    ) -> *mut libc::c_void;
}
extern "C" {
    pub fn GC_same_obj(
        arg1: *mut libc::c_void,
        arg2: *mut libc::c_void,
    ) -> *mut libc::c_void;
}
extern "C" {
    pub fn GC_is_visible(arg1: *mut libc::c_void) -> *mut libc::c_void;
}
extern "C" {
    pub fn GC_is_valid_displacement(
        arg1: *mut libc::c_void,
    ) -> *mut libc::c_void;
}
extern "C" {
    pub fn GC_dump();
}
extern "C" {
    pub fn GC_dump_named(arg1: *const libc::c_char);
}
extern "C" {
    pub fn GC_dump_regions();
}
extern "C" {
    pub fn GC_dump_finalization();
}
extern "C" {
    pub fn GC_ptr_store_and_dirty(
        arg1: *mut libc::c_void,
        arg2: *const libc::c_void,
    );
}
extern "C" {
    pub fn GC_debug_ptr_store_and_dirty(
        arg1: *mut libc::c_void,
        arg2: *const libc::c_void,
    );
}
extern "C" {
    pub fn GC_malloc_many(arg1: usize) -> *mut libc::c_void;
}
pub type GC_has_static_roots_func = ::core::option::Option<
    unsafe extern "C" fn(
        arg1: *const libc::c_char,
        arg2: *mut libc::c_void,
        arg3: usize,
    ) -> libc::c_int,
>;
extern "C" {
    pub fn GC_register_has_static_roots_callback(arg1: GC_has_static_roots_func);
}
extern "C" {
    pub fn GC_set_force_unmap_on_gcollect(arg1: libc::c_int);
}
extern "C" {
    pub fn GC_get_force_unmap_on_gcollect() -> libc::c_int;
}

#[cfg(windows)]
extern "C" {
    pub fn GC_win32_free_heap();
}

#[test]
fn test_alloc(){
    unsafe{
        let f = GC_malloc(18);
        //println!("hello");
        GC_free(f);
    }
    
}