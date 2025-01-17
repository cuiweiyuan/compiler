#[allow(dead_code)]
pub mod miden {
    #[allow(dead_code)]
    pub mod base {
        #[allow(dead_code, clippy::all)]
        pub mod core_types {
            #[used]
            #[doc(hidden)]
            static __FORCE_SECTION_REF: fn() = super::super::super::__link_custom_section_describing_imports;
        }
    }
    #[allow(dead_code)]
    pub mod core_import {
        #[allow(dead_code, clippy::all)]
        pub mod intrinsics_mem {
            #[used]
            #[doc(hidden)]
            static __FORCE_SECTION_REF: fn() = super::super::super::__link_custom_section_describing_imports;
        }
        #[allow(dead_code, clippy::all)]
        pub mod intrinsics_felt {
            #[used]
            #[doc(hidden)]
            static __FORCE_SECTION_REF: fn() = super::super::super::__link_custom_section_describing_imports;
            use super::super::super::_rt;
            #[allow(unused_unsafe, clippy::all)]
            pub fn eq(a: f32, b: f32) -> bool {
                unsafe {
                    #[cfg(target_arch = "wasm32")]
                    #[link(
                        wasm_import_module = "miden:core-import/intrinsics-felt@1.0.0"
                    )]
                    extern "C" {
                        #[link_name = "eq"]
                        fn wit_import(_: f32, _: f32) -> i32;
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(_: f32, _: f32) -> i32 {
                        unreachable!()
                    }
                    let ret = wit_import(_rt::as_f32(&a), _rt::as_f32(&b));
                    _rt::bool_lift(ret as u8)
                }
            }
            #[allow(unused_unsafe, clippy::all)]
            pub fn from_u64_unchecked(a: u64) -> f32 {
                unsafe {
                    #[cfg(target_arch = "wasm32")]
                    #[link(
                        wasm_import_module = "miden:core-import/intrinsics-felt@1.0.0"
                    )]
                    extern "C" {
                        #[link_name = "from-u64-unchecked"]
                        fn wit_import(_: i64) -> f32;
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(_: i64) -> f32 {
                        unreachable!()
                    }
                    let ret = wit_import(_rt::as_i64(&a));
                    ret
                }
            }
            #[allow(unused_unsafe, clippy::all)]
            pub fn from_u32(a: u32) -> f32 {
                unsafe {
                    #[cfg(target_arch = "wasm32")]
                    #[link(
                        wasm_import_module = "miden:core-import/intrinsics-felt@1.0.0"
                    )]
                    extern "C" {
                        #[link_name = "from-u32"]
                        fn wit_import(_: i32) -> f32;
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(_: i32) -> f32 {
                        unreachable!()
                    }
                    let ret = wit_import(_rt::as_i32(&a));
                    ret
                }
            }
            #[allow(unused_unsafe, clippy::all)]
            pub fn assert_eq(a: f32, b: f32) {
                unsafe {
                    #[cfg(target_arch = "wasm32")]
                    #[link(
                        wasm_import_module = "miden:core-import/intrinsics-felt@1.0.0"
                    )]
                    extern "C" {
                        #[link_name = "assert-eq"]
                        fn wit_import(_: f32, _: f32);
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(_: f32, _: f32) {
                        unreachable!()
                    }
                    wit_import(_rt::as_f32(&a), _rt::as_f32(&b));
                }
            }
        }
        #[allow(dead_code, clippy::all)]
        pub mod stdlib_crypto_hashes_blake3 {
            #[used]
            #[doc(hidden)]
            static __FORCE_SECTION_REF: fn() = super::super::super::__link_custom_section_describing_imports;
        }
        #[allow(dead_code, clippy::all)]
        pub mod account {
            #[used]
            #[doc(hidden)]
            static __FORCE_SECTION_REF: fn() = super::super::super::__link_custom_section_describing_imports;
            #[allow(unused_unsafe, clippy::all)]
            /// Get the id of the currently executing account
            pub fn get_id() -> f32 {
                unsafe {
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "miden:core-import/account@1.0.0")]
                    extern "C" {
                        #[link_name = "get-id"]
                        fn wit_import() -> f32;
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import() -> f32 {
                        unreachable!()
                    }
                    let ret = wit_import();
                    ret
                }
            }
        }
        #[allow(dead_code, clippy::all)]
        pub mod note {
            #[used]
            #[doc(hidden)]
            static __FORCE_SECTION_REF: fn() = super::super::super::__link_custom_section_describing_imports;
            use super::super::super::_rt;
            #[allow(unused_unsafe, clippy::all)]
            /// Get the inputs of the currently executed note
            pub fn get_inputs(ptr: i32) -> i32 {
                unsafe {
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "miden:core-import/note@1.0.0")]
                    extern "C" {
                        #[link_name = "get-inputs"]
                        fn wit_import(_: i32) -> i32;
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(_: i32) -> i32 {
                        unreachable!()
                    }
                    let ret = wit_import(_rt::as_i32(&ptr));
                    ret
                }
            }
            #[allow(unused_unsafe, clippy::all)]
            /// Get the assets of the currently executing note
            pub fn get_assets(ptr: i32) -> i32 {
                unsafe {
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "miden:core-import/note@1.0.0")]
                    extern "C" {
                        #[link_name = "get-assets"]
                        fn wit_import(_: i32) -> i32;
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(_: i32) -> i32 {
                        unreachable!()
                    }
                    let ret = wit_import(_rt::as_i32(&ptr));
                    ret
                }
            }
        }
        #[allow(dead_code, clippy::all)]
        pub mod tx {
            #[used]
            #[doc(hidden)]
            static __FORCE_SECTION_REF: fn() = super::super::super::__link_custom_section_describing_imports;
        }
    }
}
#[allow(dead_code)]
pub mod exports {
    #[allow(dead_code)]
    pub mod miden {
        #[allow(dead_code)]
        pub mod basic_wallet {
            #[allow(dead_code, clippy::all)]
            pub mod basic_wallet {
                #[used]
                #[doc(hidden)]
                static __FORCE_SECTION_REF: fn() = super::super::super::super::__link_custom_section_describing_imports;
                use super::super::super::super::_rt;
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_receive_asset_cabi<T: Guest>(
                    arg0: f32,
                    arg1: f32,
                    arg2: f32,
                    arg3: f32,
                ) {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    T::receive_asset(miden::CoreAsset {
                        inner: miden::Word {
                            inner: (
                                miden::Felt { inner: arg0 },
                                miden::Felt { inner: arg1 },
                                miden::Felt { inner: arg2 },
                                miden::Felt { inner: arg3 },
                            ),
                        },
                    });
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_send_asset_cabi<T: Guest>(
                    arg0: f32,
                    arg1: f32,
                    arg2: f32,
                    arg3: f32,
                    arg4: f32,
                    arg5: f32,
                    arg6: f32,
                    arg7: f32,
                    arg8: f32,
                    arg9: f32,
                ) {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    T::send_asset(
                        miden::CoreAsset {
                            inner: miden::Word {
                                inner: (
                                    miden::Felt { inner: arg0 },
                                    miden::Felt { inner: arg1 },
                                    miden::Felt { inner: arg2 },
                                    miden::Felt { inner: arg3 },
                                ),
                            },
                        },
                        miden::Tag {
                            inner: miden::Felt { inner: arg4 },
                        },
                        miden::NoteType {
                            inner: miden::Felt { inner: arg5 },
                        },
                        miden::Recipient {
                            inner: miden::Word {
                                inner: (
                                    miden::Felt { inner: arg6 },
                                    miden::Felt { inner: arg7 },
                                    miden::Felt { inner: arg8 },
                                    miden::Felt { inner: arg9 },
                                ),
                            },
                        },
                    );
                }
                pub trait Guest {
                    fn receive_asset(core_asset: miden::CoreAsset);
                    fn send_asset(
                        core_asset: miden::CoreAsset,
                        tag: miden::Tag,
                        note_type: miden::NoteType,
                        recipient: miden::Recipient,
                    );
                }
                #[doc(hidden)]
                macro_rules! __export_miden_basic_wallet_basic_wallet_1_0_0_cabi {
                    ($ty:ident with_types_in $($path_to_types:tt)*) => {
                        const _ : () = { #[export_name =
                        "miden:basic-wallet/basic-wallet@1.0.0#receive-asset"] unsafe
                        extern "C" fn export_receive_asset(arg0 : f32, arg1 : f32, arg2 :
                        f32, arg3 : f32,) { $($path_to_types)*::
                        _export_receive_asset_cabi::<$ty > (arg0, arg1, arg2, arg3) }
                        #[export_name =
                        "miden:basic-wallet/basic-wallet@1.0.0#send-asset"] unsafe extern
                        "C" fn export_send_asset(arg0 : f32, arg1 : f32, arg2 : f32, arg3
                        : f32, arg4 : f32, arg5 : f32, arg6 : f32, arg7 : f32, arg8 :
                        f32, arg9 : f32,) { $($path_to_types)*::
                        _export_send_asset_cabi::<$ty > (arg0, arg1, arg2, arg3, arg4,
                        arg5, arg6, arg7, arg8, arg9) } };
                    };
                }
                #[doc(hidden)]
                pub(crate) use __export_miden_basic_wallet_basic_wallet_1_0_0_cabi;
            }
            #[allow(dead_code, clippy::all)]
            pub mod aux {
                #[used]
                #[doc(hidden)]
                static __FORCE_SECTION_REF: fn() = super::super::super::super::__link_custom_section_describing_imports;
                use super::super::super::super::_rt;
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_test_felt_intrinsics_cabi<T: Guest>(
                    arg0: f32,
                    arg1: f32,
                ) -> f32 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::test_felt_intrinsics(
                        miden::Felt { inner: arg0 },
                        miden::Felt { inner: arg1 },
                    );
                    let miden::Felt { inner: inner1 } = result0;
                    _rt::as_f32(inner1)
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_test_stdlib_cabi<T: Guest>(
                    arg0: *mut u8,
                    arg1: usize,
                ) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let len0 = arg1;
                    let result1 = T::test_stdlib(
                        _rt::Vec::from_raw_parts(arg0.cast(), len0, len0),
                    );
                    let ptr2 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    let vec3 = (result1).into_boxed_slice();
                    let ptr3 = vec3.as_ptr().cast::<u8>();
                    let len3 = vec3.len();
                    ::core::mem::forget(vec3);
                    *ptr2.add(4).cast::<usize>() = len3;
                    *ptr2.add(0).cast::<*mut u8>() = ptr3.cast_mut();
                    ptr2
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_test_stdlib<T: Guest>(arg0: *mut u8) {
                    let l0 = *arg0.add(0).cast::<*mut u8>();
                    let l1 = *arg0.add(4).cast::<usize>();
                    let base2 = l0;
                    let len2 = l1;
                    _rt::cabi_dealloc(base2, len2 * 1, 1);
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_process_list_felt_cabi<T: Guest>(
                    arg0: *mut u8,
                    arg1: usize,
                ) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let len0 = arg1;
                    let result1 = T::process_list_felt(
                        _rt::Vec::from_raw_parts(arg0.cast(), len0, len0),
                    );
                    let ptr2 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    let vec3 = (result1).into_boxed_slice();
                    let ptr3 = vec3.as_ptr().cast::<u8>();
                    let len3 = vec3.len();
                    ::core::mem::forget(vec3);
                    *ptr2.add(4).cast::<usize>() = len3;
                    *ptr2.add(0).cast::<*mut u8>() = ptr3.cast_mut();
                    ptr2
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_process_list_felt<T: Guest>(arg0: *mut u8) {
                    let l0 = *arg0.add(0).cast::<*mut u8>();
                    let l1 = *arg0.add(4).cast::<usize>();
                    let base2 = l0;
                    let len2 = l1;
                    _rt::cabi_dealloc(base2, len2 * 4, 4);
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_process_core_asset_cabi<T: Guest>(
                    arg0: f32,
                    arg1: f32,
                    arg2: f32,
                    arg3: f32,
                ) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::process_core_asset(miden::CoreAsset {
                        inner: miden::Word {
                            inner: (
                                miden::Felt { inner: arg0 },
                                miden::Felt { inner: arg1 },
                                miden::Felt { inner: arg2 },
                                miden::Felt { inner: arg3 },
                            ),
                        },
                    });
                    let ptr1 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    let miden::CoreAsset { inner: inner2 } = result0;
                    let miden::Word { inner: inner3 } = inner2;
                    let (t4_0, t4_1, t4_2, t4_3) = inner3;
                    let miden::Felt { inner: inner5 } = t4_0;
                    *ptr1.add(0).cast::<f32>() = _rt::as_f32(inner5);
                    let miden::Felt { inner: inner6 } = t4_1;
                    *ptr1.add(4).cast::<f32>() = _rt::as_f32(inner6);
                    let miden::Felt { inner: inner7 } = t4_2;
                    *ptr1.add(8).cast::<f32>() = _rt::as_f32(inner7);
                    let miden::Felt { inner: inner8 } = t4_3;
                    *ptr1.add(12).cast::<f32>() = _rt::as_f32(inner8);
                    ptr1
                }
                pub trait Guest {
                    fn test_felt_intrinsics(
                        a: miden::Felt,
                        b: miden::Felt,
                    ) -> miden::Felt;
                    fn test_stdlib(input: _rt::Vec<u8>) -> _rt::Vec<u8>;
                    fn process_list_felt(
                        input: _rt::Vec<miden::Felt>,
                    ) -> _rt::Vec<miden::Felt>;
                    fn process_core_asset(input: miden::CoreAsset) -> miden::CoreAsset;
                }
                #[doc(hidden)]
                macro_rules! __export_miden_basic_wallet_aux_1_0_0_cabi {
                    ($ty:ident with_types_in $($path_to_types:tt)*) => {
                        const _ : () = { #[export_name =
                        "miden:basic-wallet/aux@1.0.0#test-felt-intrinsics"] unsafe
                        extern "C" fn export_test_felt_intrinsics(arg0 : f32, arg1 :
                        f32,) -> f32 { $($path_to_types)*::
                        _export_test_felt_intrinsics_cabi::<$ty > (arg0, arg1) }
                        #[export_name = "miden:basic-wallet/aux@1.0.0#test-stdlib"]
                        unsafe extern "C" fn export_test_stdlib(arg0 : * mut u8, arg1 :
                        usize,) -> * mut u8 { $($path_to_types)*::
                        _export_test_stdlib_cabi::<$ty > (arg0, arg1) } #[export_name =
                        "cabi_post_miden:basic-wallet/aux@1.0.0#test-stdlib"] unsafe
                        extern "C" fn _post_return_test_stdlib(arg0 : * mut u8,) {
                        $($path_to_types)*:: __post_return_test_stdlib::<$ty > (arg0) }
                        #[export_name = "miden:basic-wallet/aux@1.0.0#process-list-felt"]
                        unsafe extern "C" fn export_process_list_felt(arg0 : * mut u8,
                        arg1 : usize,) -> * mut u8 { $($path_to_types)*::
                        _export_process_list_felt_cabi::<$ty > (arg0, arg1) }
                        #[export_name =
                        "cabi_post_miden:basic-wallet/aux@1.0.0#process-list-felt"]
                        unsafe extern "C" fn _post_return_process_list_felt(arg0 : * mut
                        u8,) { $($path_to_types)*:: __post_return_process_list_felt::<$ty
                        > (arg0) } #[export_name =
                        "miden:basic-wallet/aux@1.0.0#process-core-asset"] unsafe extern
                        "C" fn export_process_core_asset(arg0 : f32, arg1 : f32, arg2 :
                        f32, arg3 : f32,) -> * mut u8 { $($path_to_types)*::
                        _export_process_core_asset_cabi::<$ty > (arg0, arg1, arg2, arg3)
                        } };
                    };
                }
                #[doc(hidden)]
                pub(crate) use __export_miden_basic_wallet_aux_1_0_0_cabi;
                #[repr(align(4))]
                struct _RetArea([::core::mem::MaybeUninit<u8>; 16]);
                static mut _RET_AREA: _RetArea = _RetArea(
                    [::core::mem::MaybeUninit::uninit(); 16],
                );
            }
        }
    }
}
mod _rt {
    pub fn as_f32<T: AsF32>(t: T) -> f32 {
        t.as_f32()
    }
    pub trait AsF32 {
        fn as_f32(self) -> f32;
    }
    impl<'a, T: Copy + AsF32> AsF32 for &'a T {
        fn as_f32(self) -> f32 {
            (*self).as_f32()
        }
    }
    impl AsF32 for f32 {
        #[inline]
        fn as_f32(self) -> f32 {
            self as f32
        }
    }
    pub unsafe fn bool_lift(val: u8) -> bool {
        if cfg!(debug_assertions) {
            match val {
                0 => false,
                1 => true,
                _ => panic!("invalid bool discriminant"),
            }
        } else {
            val != 0
        }
    }
    pub fn as_i64<T: AsI64>(t: T) -> i64 {
        t.as_i64()
    }
    pub trait AsI64 {
        fn as_i64(self) -> i64;
    }
    impl<'a, T: Copy + AsI64> AsI64 for &'a T {
        fn as_i64(self) -> i64 {
            (*self).as_i64()
        }
    }
    impl AsI64 for i64 {
        #[inline]
        fn as_i64(self) -> i64 {
            self as i64
        }
    }
    impl AsI64 for u64 {
        #[inline]
        fn as_i64(self) -> i64 {
            self as i64
        }
    }
    pub fn as_i32<T: AsI32>(t: T) -> i32 {
        t.as_i32()
    }
    pub trait AsI32 {
        fn as_i32(self) -> i32;
    }
    impl<'a, T: Copy + AsI32> AsI32 for &'a T {
        fn as_i32(self) -> i32 {
            (*self).as_i32()
        }
    }
    impl AsI32 for i32 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for u32 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for i16 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for u16 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for i8 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for u8 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for char {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for usize {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    #[cfg(target_arch = "wasm32")]
    pub fn run_ctors_once() {
        wit_bindgen_rt::run_ctors_once();
    }
    pub use alloc_crate::vec::Vec;
    pub unsafe fn cabi_dealloc(ptr: *mut u8, size: usize, align: usize) {
        if size == 0 {
            return;
        }
        let layout = alloc::Layout::from_size_align_unchecked(size, align);
        alloc::dealloc(ptr, layout);
    }
    extern crate alloc as alloc_crate;
    pub use alloc_crate::alloc;
}
/// Generates `#[no_mangle]` functions to export the specified type as the
/// root implementation of all generated traits.
///
/// For more information see the documentation of `wit_bindgen::generate!`.
///
/// ```rust
/// # macro_rules! export{ ($($t:tt)*) => (); }
/// # trait Guest {}
/// struct MyType;
///
/// impl Guest for MyType {
///     // ...
/// }
///
/// export!(MyType);
/// ```
#[allow(unused_macros)]
#[doc(hidden)]
macro_rules! __export_basic_wallet_world_impl {
    ($ty:ident) => {
        self::export!($ty with_types_in self);
    };
    ($ty:ident with_types_in $($path_to_types_root:tt)*) => {
        $($path_to_types_root)*::
        exports::miden::basic_wallet::basic_wallet::__export_miden_basic_wallet_basic_wallet_1_0_0_cabi!($ty
        with_types_in $($path_to_types_root)*::
        exports::miden::basic_wallet::basic_wallet); $($path_to_types_root)*::
        exports::miden::basic_wallet::aux::__export_miden_basic_wallet_aux_1_0_0_cabi!($ty
        with_types_in $($path_to_types_root)*:: exports::miden::basic_wallet::aux);
    };
}
#[doc(inline)]
pub(crate) use __export_basic_wallet_world_impl as export;
#[cfg(target_arch = "wasm32")]
#[link_section = "component-type:wit-bindgen:0.31.0:miden:basic-wallet@1.0.0:basic-wallet-world:encoded world"]
#[doc(hidden)]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 1884] = *b"\
\0asm\x0d\0\x01\0\0\x19\x16wit-component-encoding\x04\0\x07\xd3\x0d\x01A\x02\x01\
A\x17\x01B\x1f\x01r\x01\x05innerv\x04\0\x04felt\x03\0\0\x01o\x04\x01\x01\x01\x01\
\x01r\x01\x05inner\x02\x04\0\x04word\x03\0\x03\x01r\x01\x05inner\x01\x04\0\x0aac\
count-id\x03\0\x05\x01r\x01\x05inner\x04\x04\0\x09recipient\x03\0\x07\x01r\x01\x05\
inner\x01\x04\0\x03tag\x03\0\x09\x01r\x01\x05inner\x04\x04\0\x0acore-asset\x03\0\
\x0b\x01r\x01\x05inner\x01\x04\0\x05nonce\x03\0\x0d\x01r\x01\x05inner\x04\x04\0\x0c\
account-hash\x03\0\x0f\x01r\x01\x05inner\x04\x04\0\x0ablock-hash\x03\0\x11\x01r\x01\
\x05inner\x04\x04\0\x0dstorage-value\x03\0\x13\x01r\x01\x05inner\x04\x04\0\x0cst\
orage-root\x03\0\x15\x01r\x01\x05inner\x04\x04\0\x11account-code-root\x03\0\x17\x01\
r\x01\x05inner\x04\x04\0\x10vault-commitment\x03\0\x19\x01r\x01\x05inner\x01\x04\
\0\x07note-id\x03\0\x1b\x01r\x01\x05inner\x01\x04\0\x09note-type\x03\0\x1d\x03\x01\
\x1bmiden:base/core-types@1.0.0\x05\0\x01B\x02\x01@\0\0z\x04\0\x09heap-base\x01\0\
\x03\x01&miden:core-import/intrinsics-mem@1.0.0\x05\x01\x01B\x0a\x01@\x02\x01av\x01\
bv\0v\x04\0\x03add\x01\0\x01@\x02\x01av\x01bv\0\x7f\x04\0\x02eq\x01\x01\x01@\x01\
\x01aw\0v\x04\0\x12from-u64-unchecked\x01\x02\x01@\x01\x01ay\0v\x04\0\x08from-u3\
2\x01\x03\x01@\x02\x01av\x01bv\x01\0\x04\0\x09assert-eq\x01\x04\x03\x01'miden:co\
re-import/intrinsics-felt@1.0.0\x05\x02\x01B\x02\x01@\x09\x02a0z\x02a1z\x02a2z\x02\
a3z\x02a4z\x02a5z\x02a6z\x02a7z\x0aresult-ptrz\x01\0\x04\0\x0fhash-one-to-one\x01\
\0\x03\x013miden:core-import/stdlib-crypto-hashes-blake3@1.0.0\x05\x03\x01B\x05\x01\
@\x05\x06asset0v\x06asset1v\x06asset2v\x06asset3v\x0aresult-ptrz\x01\0\x04\0\x09\
add-asset\x01\0\x04\0\x0cremove-asset\x01\0\x01@\0\0v\x04\0\x06get-id\x01\x01\x03\
\x01\x1fmiden:core-import/account@1.0.0\x05\x04\x01B\x03\x01@\x01\x03ptrz\0z\x04\
\0\x0aget-inputs\x01\0\x04\0\x0aget-assets\x01\0\x03\x01\x1cmiden:core-import/no\
te@1.0.0\x05\x05\x01B\x02\x01@\x0a\x06asset0v\x06asset1v\x06asset2v\x06asset3v\x03\
tagv\x09note-typev\x0arecipient0v\x0arecipient1v\x0arecipient2v\x0arecipient3v\0\
v\x04\0\x0bcreate-note\x01\0\x03\x01\x1amiden:core-import/tx@1.0.0\x05\x06\x02\x03\
\0\0\x0acore-asset\x02\x03\0\0\x03tag\x02\x03\0\0\x09recipient\x02\x03\0\0\x09no\
te-type\x02\x03\0\0\x04felt\x01B\x0e\x02\x03\x02\x01\x07\x04\0\x0acore-asset\x03\
\0\0\x02\x03\x02\x01\x08\x04\0\x03tag\x03\0\x02\x02\x03\x02\x01\x09\x04\0\x09rec\
ipient\x03\0\x04\x02\x03\x02\x01\x0a\x04\0\x09note-type\x03\0\x06\x02\x03\x02\x01\
\x0b\x04\0\x04felt\x03\0\x08\x01@\x01\x0acore-asset\x01\x01\0\x04\0\x0dreceive-a\
sset\x01\x0a\x01@\x04\x0acore-asset\x01\x03tag\x03\x09note-type\x07\x09recipient\
\x05\x01\0\x04\0\x0asend-asset\x01\x0b\x04\x01%miden:basic-wallet/basic-wallet@1\
.0.0\x05\x0c\x01B\x14\x02\x03\x02\x01\x07\x04\0\x0acore-asset\x03\0\0\x02\x03\x02\
\x01\x08\x04\0\x03tag\x03\0\x02\x02\x03\x02\x01\x09\x04\0\x09recipient\x03\0\x04\
\x02\x03\x02\x01\x0a\x04\0\x09note-type\x03\0\x06\x02\x03\x02\x01\x0b\x04\0\x04f\
elt\x03\0\x08\x01@\x02\x01a\x09\x01b\x09\0\x09\x04\0\x14test-felt-intrinsics\x01\
\x0a\x01p}\x01@\x01\x05input\x0b\0\x0b\x04\0\x0btest-stdlib\x01\x0c\x01p\x09\x01\
@\x01\x05input\x0d\0\x0d\x04\0\x11process-list-felt\x01\x0e\x01@\x01\x05input\x01\
\0\x01\x04\0\x12process-core-asset\x01\x0f\x04\x01\x1cmiden:basic-wallet/aux@1.0\
.0\x05\x0d\x04\x01+miden:basic-wallet/basic-wallet-world@1.0.0\x04\0\x0b\x18\x01\
\0\x12basic-wallet-world\x03\0\0\0G\x09producers\x01\x0cprocessed-by\x02\x0dwit-\
component\x070.216.0\x10wit-bindgen-rust\x060.31.0";
#[inline(never)]
#[doc(hidden)]
pub fn __link_custom_section_describing_imports() {
    wit_bindgen_rt::maybe_link_cabi_realloc();
}
