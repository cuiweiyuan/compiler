(component
  (type (;0;)
    (instance
      (type (;0;) (record (field "inner" float32)))
      (export (;1;) "felt" (type (eq 0)))
      (type (;2;) (tuple 1 1 1 1))
      (type (;3;) (record (field "inner" 2)))
      (export (;4;) "word" (type (eq 3)))
      (type (;5;) (record (field "inner" 4)))
      (export (;6;) "core-asset" (type (eq 5)))
    )
  )
  (import "miden:base/core-types@1.0.0" (instance (;0;) (type 0)))
  (alias export 0 "core-asset" (type (;1;)))
  (type (;2;)
    (instance
      (alias outer 1 1 (type (;0;)))
      (export (;1;) "core-asset" (type (eq 0)))
      (type (;2;) (func (param "core-asset" 1)))
      (export (;0;) "receive-asset" (func (type 2)))
    )
  )
  (import "miden:basic-wallet/basic-wallet@1.0.0" (instance (;1;) (type 2)))
  (alias export 0 "felt" (type (;3;)))
  (type (;4;)
    (instance
      (alias outer 1 3 (type (;0;)))
      (export (;1;) "felt" (type (eq 0)))
      (type (;2;) (list 1))
      (type (;3;) (func (param "input" 2) (result 2)))
      (export (;0;) "process-list-felt" (func (type 3)))
    )
  )
  (import "miden:basic-wallet/aux@1.0.0" (instance (;2;) (type 4)))
  (type (;5;)
    (instance
      (type (;0;) (func (result s32)))
      (export (;0;) "heap-base" (func (type 0)))
    )
  )
  (import "miden:core-import/intrinsics-mem@1.0.0" (instance (;3;) (type 5)))
  (type (;6;)
    (instance
      (type (;0;) (func (param "a" float32) (param "b" float32)))
      (export (;0;) "assert-eq" (func (type 0)))
    )
  )
  (import "miden:core-import/intrinsics-felt@1.0.0" (instance (;4;) (type 6)))
  (type (;7;)
    (instance
      (type (;0;) (func (result float32)))
      (export (;0;) "get-id" (func (type 0)))
    )
  )
  (import "miden:core-import/account@1.0.0" (instance (;5;) (type 7)))
  (type (;8;)
    (instance
      (type (;0;) (func (param "ptr" s32) (result s32)))
      (export (;0;) "get-inputs" (func (type 0)))
    )
  )
  (import "miden:core-import/note@1.0.0" (instance (;6;) (type 8)))
  (core module (;0;)
    (type (;0;) (func (param i32 i32 i32)))
    (type (;1;) (func (param f32 f32)))
    (type (;2;) (func (param f32 f32 f32 f32)))
    (type (;3;) (func (result i32)))
    (type (;4;) (func (result f32)))
    (type (;5;) (func (param i32) (result i32)))
    (type (;6;) (func))
    (type (;7;) (func (param i32 i32) (result i32)))
    (type (;8;) (func (param i32 i32 i32 i32) (result i32)))
    (type (;9;) (func (param i32 i32 i32) (result i32)))
    (type (;10;) (func (param i32)))
    (type (;11;) (func (param i32 i32)))
    (import "miden:basic-wallet/aux@1.0.0" "process-list-felt" (func $p2id::bindings::miden::basic_wallet::aux::process_list_felt::wit_import (;0;) (type 0)))
    (import "miden:core-import/intrinsics-felt@1.0.0" "assert-eq" (func $miden_stdlib_sys::intrinsics::felt::extern_assert_eq (;1;) (type 1)))
    (import "miden:basic-wallet/basic-wallet@1.0.0" "receive-asset" (func $p2id::bindings::miden::basic_wallet::basic_wallet::receive_asset::wit_import (;2;) (type 2)))
    (import "miden:core-import/intrinsics-mem@1.0.0" "heap-base" (func $miden_sdk_alloc::heap_base (;3;) (type 3)))
    (import "miden:core-import/account@1.0.0" "get-id" (func $miden_base_sys::bindings::account::extern_account_get_id (;4;) (type 4)))
    (import "miden:core-import/note@1.0.0" "get-inputs" (func $miden_base_sys::bindings::note::extern_note_get_inputs (;5;) (type 5)))
    (func $__wasm_call_ctors (;6;) (type 6))
    (func $p2id::bindings::__link_custom_section_describing_imports (;7;) (type 6))
    (func $__rust_alloc (;8;) (type 7) (param i32 i32) (result i32)
      i32.const 1048620
      local.get 1
      local.get 0
      call $<miden_sdk_alloc::BumpAlloc as core::alloc::global::GlobalAlloc>::alloc
    )
    (func $__rust_realloc (;9;) (type 8) (param i32 i32 i32 i32) (result i32)
      block ;; label = @1
        i32.const 1048620
        local.get 2
        local.get 3
        call $<miden_sdk_alloc::BumpAlloc as core::alloc::global::GlobalAlloc>::alloc
        local.tee 2
        i32.eqz
        br_if 0 (;@1;)
        local.get 2
        local.get 0
        local.get 1
        local.get 3
        local.get 1
        local.get 3
        i32.lt_u
        select
        memory.copy
      end
      local.get 2
    )
    (func $__rust_alloc_zeroed (;10;) (type 7) (param i32 i32) (result i32)
      block ;; label = @1
        i32.const 1048620
        local.get 1
        local.get 0
        call $<miden_sdk_alloc::BumpAlloc as core::alloc::global::GlobalAlloc>::alloc
        local.tee 1
        i32.eqz
        br_if 0 (;@1;)
        local.get 1
        i32.const 0
        local.get 0
        memory.fill
      end
      local.get 1
    )
    (func $miden:base/note-script@1.0.0#note-script (;11;) (type 6)
      (local i32 i32 i32 f32)
      global.get $__stack_pointer
      i32.const 32
      i32.sub
      local.tee 0
      global.set $__stack_pointer
      call $wit_bindgen_rt::run_ctors_once
      local.get 0
      i32.const 4
      i32.add
      call $miden_base_sys::bindings::note::get_inputs
      local.get 0
      i32.load offset=12
      local.set 1
      local.get 0
      i32.load offset=8
      local.set 2
      local.get 0
      i64.const 0
      i64.store offset=16
      local.get 2
      local.get 1
      local.get 0
      i32.const 16
      i32.add
      call $p2id::bindings::miden::basic_wallet::aux::process_list_felt::wit_import
      block ;; label = @1
        local.get 0
        i32.load offset=20
        i32.eqz
        br_if 0 (;@1;)
        local.get 1
        i32.eqz
        br_if 0 (;@1;)
        local.get 0
        i32.load offset=16
        f32.load
        local.get 2
        f32.load
        call $miden_stdlib_sys::intrinsics::felt::extern_assert_eq
        local.get 2
        f32.load
        local.set 3
        call $miden_base_sys::bindings::account::get_id
        local.get 3
        call $miden_stdlib_sys::intrinsics::felt::extern_assert_eq
        local.get 0
        i32.const 16
        i32.add
        call $miden_base_sys::bindings::note::get_assets
        local.get 0
        i32.load offset=24
        i32.const 5
        i32.shl
        local.set 1
        local.get 0
        i32.load offset=20
        local.set 2
        block ;; label = @2
          loop ;; label = @3
            local.get 1
            i32.eqz
            br_if 1 (;@2;)
            local.get 2
            f32.load
            local.get 2
            f32.load offset=4
            local.get 2
            f32.load offset=8
            local.get 2
            f32.load offset=12
            call $p2id::bindings::miden::basic_wallet::basic_wallet::receive_asset::wit_import
            local.get 1
            i32.const -32
            i32.add
            local.set 1
            local.get 2
            i32.const 32
            i32.add
            local.set 2
            br 0 (;@3;)
          end
        end
        local.get 0
        i32.const 32
        i32.add
        global.set $__stack_pointer
        return
      end
      unreachable
    )
    (func $cabi_realloc_wit_bindgen_0_28_0 (;12;) (type 8) (param i32 i32 i32 i32) (result i32)
      local.get 0
      local.get 1
      local.get 2
      local.get 3
      call $wit_bindgen_rt::cabi_realloc
    )
    (func $wit_bindgen_rt::cabi_realloc (;13;) (type 8) (param i32 i32 i32 i32) (result i32)
      block ;; label = @1
        block ;; label = @2
          block ;; label = @3
            local.get 1
            br_if 0 (;@3;)
            local.get 3
            i32.eqz
            br_if 2 (;@1;)
            i32.const 0
            i32.load8_u offset=1048624
            drop
            local.get 3
            local.get 2
            call $__rust_alloc
            local.set 2
            br 1 (;@2;)
          end
          local.get 0
          local.get 1
          local.get 2
          local.get 3
          call $__rust_realloc
          local.set 2
        end
        local.get 2
        br_if 0 (;@1;)
        unreachable
      end
      local.get 2
    )
    (func $wit_bindgen_rt::run_ctors_once (;14;) (type 6)
      block ;; label = @1
        i32.const 0
        i32.load8_u offset=1048625
        br_if 0 (;@1;)
        call $__wasm_call_ctors
        i32.const 0
        i32.const 1
        i32.store8 offset=1048625
      end
    )
    (func $<miden_sdk_alloc::BumpAlloc as core::alloc::global::GlobalAlloc>::alloc (;15;) (type 9) (param i32 i32 i32) (result i32)
      (local i32 i32)
      block ;; label = @1
        local.get 1
        i32.const 32
        local.get 1
        i32.const 32
        i32.gt_u
        select
        local.tee 1
        i32.popcnt
        i32.const 1
        i32.ne
        br_if 0 (;@1;)
        i32.const -2147483648
        local.get 1
        i32.sub
        local.get 2
        i32.lt_u
        br_if 0 (;@1;)
        i32.const 0
        local.set 3
        local.get 1
        local.get 2
        i32.add
        i32.const -1
        i32.add
        i32.const 0
        local.get 1
        i32.sub
        i32.and
        local.set 2
        block ;; label = @2
          local.get 0
          i32.load
          br_if 0 (;@2;)
          local.get 0
          call $miden_sdk_alloc::heap_base
          memory.size
          i32.const 16
          i32.shl
          i32.add
          i32.store
        end
        block ;; label = @2
          i32.const 268435456
          local.get 0
          i32.load
          local.tee 4
          i32.sub
          local.get 2
          i32.lt_u
          br_if 0 (;@2;)
          local.get 0
          local.get 4
          local.get 2
          i32.add
          i32.store
          local.get 4
          local.get 1
          i32.add
          local.set 3
        end
        local.get 3
        return
      end
      unreachable
    )
    (func $miden_base_sys::bindings::account::get_id (;16;) (type 4) (result f32)
      call $miden_base_sys::bindings::account::extern_account_get_id
    )
    (func $miden_base_sys::bindings::note::get_inputs (;17;) (type 10) (param i32)
      (local i32 i32 i32)
      global.get $__stack_pointer
      i32.const 16
      i32.sub
      local.tee 1
      global.set $__stack_pointer
      local.get 1
      i32.const 4
      i32.add
      i32.const 256
      i32.const 0
      call $alloc::raw_vec::RawVec<T,A>::try_allocate_in
      local.get 1
      i32.load offset=8
      local.set 2
      block ;; label = @1
        local.get 1
        i32.load offset=4
        i32.const 1
        i32.ne
        br_if 0 (;@1;)
        local.get 2
        local.get 1
        i32.load offset=12
        call $alloc::raw_vec::handle_error
        unreachable
      end
      local.get 0
      local.get 1
      i32.load offset=12
      local.tee 3
      i32.const 4
      i32.shr_u
      call $miden_base_sys::bindings::note::extern_note_get_inputs
      i32.store offset=8
      local.get 0
      local.get 3
      i32.store offset=4
      local.get 0
      local.get 2
      i32.store
      local.get 1
      i32.const 16
      i32.add
      global.set $__stack_pointer
    )
    (func $miden_base_sys::bindings::note::get_assets (;18;) (type 10) (param i32)
      unreachable
    )
    (func $alloc::raw_vec::RawVec<T,A>::try_allocate_in (;19;) (type 0) (param i32 i32 i32)
      (local i32)
      block ;; label = @1
        block ;; label = @2
          local.get 1
          br_if 0 (;@2;)
          local.get 0
          i64.const 17179869184
          i64.store offset=4 align=4
          i32.const 0
          local.set 1
          br 1 (;@1;)
        end
        block ;; label = @2
          block ;; label = @3
            local.get 1
            i32.const 536870912
            i32.lt_u
            br_if 0 (;@3;)
            local.get 0
            i32.const 0
            i32.store offset=4
            br 1 (;@2;)
          end
          local.get 1
          i32.const 2
          i32.shl
          local.set 3
          block ;; label = @3
            block ;; label = @4
              local.get 2
              br_if 0 (;@4;)
              i32.const 0
              i32.load8_u offset=1048624
              drop
              local.get 3
              i32.const 4
              call $__rust_alloc
              local.set 2
              br 1 (;@3;)
            end
            local.get 3
            i32.const 4
            call $__rust_alloc_zeroed
            local.set 2
          end
          block ;; label = @3
            local.get 2
            i32.eqz
            br_if 0 (;@3;)
            local.get 0
            local.get 2
            i32.store offset=8
            local.get 0
            local.get 1
            i32.store offset=4
            i32.const 0
            local.set 1
            br 2 (;@1;)
          end
          local.get 0
          local.get 3
          i32.store offset=8
          local.get 0
          i32.const 4
          i32.store offset=4
        end
        i32.const 1
        local.set 1
      end
      local.get 0
      local.get 1
      i32.store
    )
    (func $alloc::raw_vec::handle_error (;20;) (type 11) (param i32 i32)
      unreachable
    )
    (func $cabi_realloc (;21;) (type 8) (param i32 i32 i32 i32) (result i32)
      local.get 0
      local.get 1
      local.get 2
      local.get 3
      call $cabi_realloc_wit_bindgen_0_28_0
    )
    (table (;0;) 3 3 funcref)
    (memory (;0;) 17)
    (global $__stack_pointer (;0;) (mut i32) i32.const 1048576)
    (export "memory" (memory 0))
    (export "miden:base/note-script@1.0.0#note-script" (func $miden:base/note-script@1.0.0#note-script))
    (export "cabi_realloc_wit_bindgen_0_28_0" (func $cabi_realloc_wit_bindgen_0_28_0))
    (export "cabi_realloc" (func $cabi_realloc))
    (elem (;0;) (i32.const 1) func $p2id::bindings::__link_custom_section_describing_imports $cabi_realloc)
    (data $.rodata (;0;) (i32.const 1048576) "\01\00\00\00\01\00\00\00\01\00\00\00\01\00\00\00\01\00\00\00\01\00\00\00\01\00\00\00\01\00\00\00\01\00\00\00\01\00\00\00\02\00\00\00")
  )
  (core module (;1;)
    (type (;0;) (func (param i32 i32 i32)))
    (func $indirect-miden:basic-wallet/aux@1.0.0-process-list-felt (;0;) (type 0) (param i32 i32 i32)
      local.get 0
      local.get 1
      local.get 2
      i32.const 0
      call_indirect (type 0)
    )
    (table (;0;) 1 1 funcref)
    (export "0" (func $indirect-miden:basic-wallet/aux@1.0.0-process-list-felt))
    (export "$imports" (table 0))
  )
  (core module (;2;)
    (type (;0;) (func (param i32 i32 i32)))
    (import "" "0" (func (;0;) (type 0)))
    (import "" "$imports" (table (;0;) 1 1 funcref))
    (elem (;0;) (i32.const 0) func 0)
  )
  (core instance (;0;) (instantiate 1))
  (alias core export 0 "0" (core func (;0;)))
  (core instance (;1;)
    (export "process-list-felt" (func 0))
  )
  (alias export 4 "assert-eq" (func (;0;)))
  (core func (;1;) (canon lower (func 0)))
  (core instance (;2;)
    (export "assert-eq" (func 1))
  )
  (alias export 1 "receive-asset" (func (;1;)))
  (core func (;2;) (canon lower (func 1)))
  (core instance (;3;)
    (export "receive-asset" (func 2))
  )
  (alias export 3 "heap-base" (func (;2;)))
  (core func (;3;) (canon lower (func 2)))
  (core instance (;4;)
    (export "heap-base" (func 3))
  )
  (alias export 5 "get-id" (func (;3;)))
  (core func (;4;) (canon lower (func 3)))
  (core instance (;5;)
    (export "get-id" (func 4))
  )
  (alias export 6 "get-inputs" (func (;4;)))
  (core func (;5;) (canon lower (func 4)))
  (core instance (;6;)
    (export "get-inputs" (func 5))
  )
  (core instance (;7;) (instantiate 0
      (with "miden:basic-wallet/aux@1.0.0" (instance 1))
      (with "miden:core-import/intrinsics-felt@1.0.0" (instance 2))
      (with "miden:basic-wallet/basic-wallet@1.0.0" (instance 3))
      (with "miden:core-import/intrinsics-mem@1.0.0" (instance 4))
      (with "miden:core-import/account@1.0.0" (instance 5))
      (with "miden:core-import/note@1.0.0" (instance 6))
    )
  )
  (alias core export 7 "memory" (core memory (;0;)))
  (alias core export 7 "cabi_realloc" (core func (;6;)))
  (alias core export 0 "$imports" (core table (;0;)))
  (alias export 2 "process-list-felt" (func (;5;)))
  (core func (;7;) (canon lower (func 5) (memory 0) (realloc 6)))
  (core instance (;8;)
    (export "$imports" (table 0))
    (export "0" (func 7))
  )
  (core instance (;9;) (instantiate 2
      (with "" (instance 8))
    )
  )
  (type (;9;) (func))
  (alias core export 7 "miden:base/note-script@1.0.0#note-script" (core func (;8;)))
  (func (;6;) (type 9) (canon lift (core func 8)))
  (component (;0;)
    (type (;0;) (func))
    (import "import-func-note-script" (func (;0;) (type 0)))
    (type (;1;) (func))
    (export (;1;) "note-script" (func 0) (func (type 1)))
  )
  (instance (;7;) (instantiate 0
      (with "import-func-note-script" (func 6))
    )
  )
  (export (;8;) "miden:base/note-script@1.0.0" (instance 7))
)