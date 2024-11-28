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
    #[allow(dead_code)]
    pub mod cross_ctx_account {
        #[allow(dead_code, clippy::all)]
        pub mod foo {
            #[used]
            #[doc(hidden)]
            static __FORCE_SECTION_REF: fn() = super::super::super::__link_custom_section_describing_imports;
            use super::super::super::_rt;
            #[allow(unused_unsafe, clippy::all)]
            pub fn process_felt(input: miden::Felt) -> miden::Felt {
                unsafe {
                    let miden::Felt { inner: inner0 } = input;
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "miden:cross-ctx-account/foo@1.0.0")]
                    extern "C" {
                        #[link_name = "process-felt"]
                        fn wit_import(_: f32) -> f32;
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(_: f32) -> f32 {
                        unreachable!()
                    }
                    let ret = wit_import(_rt::as_f32(inner0));
                    miden::Felt { inner: ret }
                }
            }
        }
    }
}
#[allow(dead_code)]
pub mod exports {
    #[allow(dead_code)]
    pub mod miden {
        #[allow(dead_code)]
        pub mod base {
            #[allow(dead_code, clippy::all)]
            pub mod note_script {
                #[used]
                #[doc(hidden)]
                static __FORCE_SECTION_REF: fn() = super::super::super::super::__link_custom_section_describing_imports;
                use super::super::super::super::_rt;
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_note_script_cabi<T: Guest>() {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    T::note_script();
                }
                pub trait Guest {
                    fn note_script();
                }
                #[doc(hidden)]
                macro_rules! __export_miden_base_note_script_1_0_0_cabi {
                    ($ty:ident with_types_in $($path_to_types:tt)*) => {
                        const _ : () = { #[export_name =
                        "miden:base/note-script@1.0.0#note-script"] unsafe extern "C" fn
                        export_note_script() { $($path_to_types)*::
                        _export_note_script_cabi::<$ty > () } };
                    };
                }
                #[doc(hidden)]
                pub(crate) use __export_miden_base_note_script_1_0_0_cabi;
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
macro_rules! __export_cross_ctx_note_world_impl {
    ($ty:ident) => {
        self::export!($ty with_types_in self);
    };
    ($ty:ident with_types_in $($path_to_types_root:tt)*) => {
        $($path_to_types_root)*::
        exports::miden::base::note_script::__export_miden_base_note_script_1_0_0_cabi!($ty
        with_types_in $($path_to_types_root)*:: exports::miden::base::note_script);
    };
}
#[doc(inline)]
pub(crate) use __export_cross_ctx_note_world_impl as export;
#[cfg(target_arch = "wasm32")]
#[link_section = "component-type:wit-bindgen:0.31.0:miden:cross-ctx-note@1.0.0:cross-ctx-note-world:encoded world"]
#[doc(hidden)]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 1466] = *b"\
\0asm\x0d\0\x01\0\0\x19\x16wit-component-encoding\x04\0\x07\xaf\x0a\x01A\x02\x01\
A\x13\x01B\x1f\x01r\x01\x05innerv\x04\0\x04felt\x03\0\0\x01o\x04\x01\x01\x01\x01\
\x01r\x01\x05inner\x02\x04\0\x04word\x03\0\x03\x01r\x01\x05inner\x01\x04\0\x0aac\
count-id\x03\0\x05\x01r\x01\x05inner\x04\x04\0\x09recipient\x03\0\x07\x01r\x01\x05\
inner\x01\x04\0\x03tag\x03\0\x09\x01r\x01\x05inner\x04\x04\0\x0acore-asset\x03\0\
\x0b\x01r\x01\x05inner\x01\x04\0\x05nonce\x03\0\x0d\x01r\x01\x05inner\x04\x04\0\x0c\
account-hash\x03\0\x0f\x01r\x01\x05inner\x04\x04\0\x0ablock-hash\x03\0\x11\x01r\x01\
\x05inner\x04\x04\0\x0dstorage-value\x03\0\x13\x01r\x01\x05inner\x04\x04\0\x0cst\
orage-root\x03\0\x15\x01r\x01\x05inner\x04\x04\0\x11account-code-root\x03\0\x17\x01\
r\x01\x05inner\x04\x04\0\x10vault-commitment\x03\0\x19\x01r\x01\x05inner\x01\x04\
\0\x07note-id\x03\0\x1b\x01r\x01\x05inner\x01\x04\0\x09note-type\x03\0\x1d\x03\x01\
\x1bmiden:base/core-types@1.0.0\x05\0\x02\x03\0\0\x04felt\x01B\x04\x02\x03\x02\x01\
\x01\x04\0\x04felt\x03\0\0\x01@\x01\x05input\x01\0\x01\x04\0\x0cprocess-felt\x01\
\x02\x03\x01!miden:cross-ctx-account/foo@1.0.0\x05\x02\x01B\x02\x01@\0\0z\x04\0\x09\
heap-base\x01\0\x03\x01&miden:core-import/intrinsics-mem@1.0.0\x05\x03\x01B\x08\x01\
@\x02\x01av\x01bv\0v\x04\0\x03add\x01\0\x01@\x02\x01av\x01bv\0\x7f\x04\0\x02eq\x01\
\x01\x01@\x01\x01aw\0v\x04\0\x12from-u64-unchecked\x01\x02\x01@\x02\x01av\x01bv\x01\
\0\x04\0\x09assert-eq\x01\x03\x03\x01'miden:core-import/intrinsics-felt@1.0.0\x05\
\x04\x01B\x02\x01@\x09\x02a0z\x02a1z\x02a2z\x02a3z\x02a4z\x02a5z\x02a6z\x02a7z\x0a\
result-ptrz\x01\0\x04\0\x0fhash-one-to-one\x01\0\x03\x013miden:core-import/stdli\
b-crypto-hashes-blake3@1.0.0\x05\x05\x01B\x05\x01@\x05\x06asset0v\x06asset1v\x06\
asset2v\x06asset3v\x0aresult-ptrz\x01\0\x04\0\x09add-asset\x01\0\x04\0\x0cremove\
-asset\x01\0\x01@\0\0v\x04\0\x06get-id\x01\x01\x03\x01\x1fmiden:core-import/acco\
unt@1.0.0\x05\x06\x01B\x03\x01@\x01\x03ptrz\0z\x04\0\x0aget-inputs\x01\0\x04\0\x0a\
get-assets\x01\0\x03\x01\x1cmiden:core-import/note@1.0.0\x05\x07\x01B\x02\x01@\x0a\
\x06asset0v\x06asset1v\x06asset2v\x06asset3v\x03tagv\x09note-typev\x0arecipient0\
v\x0arecipient1v\x0arecipient2v\x0arecipient3v\0v\x04\0\x0bcreate-note\x01\0\x03\
\x01\x1amiden:core-import/tx@1.0.0\x05\x08\x01B\x02\x01@\0\x01\0\x04\0\x0bnote-s\
cript\x01\0\x04\x01\x1cmiden:base/note-script@1.0.0\x05\x09\x04\x01/miden:cross-\
ctx-note/cross-ctx-note-world@1.0.0\x04\0\x0b\x1a\x01\0\x14cross-ctx-note-world\x03\
\0\0\0G\x09producers\x01\x0cprocessed-by\x02\x0dwit-component\x070.216.0\x10wit-\
bindgen-rust\x060.31.0";
#[inline(never)]
#[doc(hidden)]
pub fn __link_custom_section_describing_imports() {
    wit_bindgen_rt::maybe_link_cabi_realloc();
}
