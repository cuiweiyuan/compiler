(component
  (core module (;0;)
    (type (;0;) (func))
    (type (;1;) (func (param i32 i32) (result i32)))
    (type (;2;) (func (param i32 i32 i32 i32) (result i32)))
    (type (;3;) (func (param i32 i32 i32) (result i32)))
    (type (;4;) (func (param i32 i32 i32 i32)))
    (func $__wasm_call_ctors (;0;) (type 0))
    (func $add_wasm_component::bindings::__link_custom_section_describing_imports (;1;) (type 0))
    (func $__rust_alloc (;2;) (type 1) (param i32 i32) (result i32)
      i32.const 1048584
      local.get 1
      local.get 0
      call $<wee_alloc::WeeAlloc as core::alloc::global::GlobalAlloc>::alloc
    )
    (func $__rust_realloc (;3;) (type 2) (param i32 i32 i32 i32) (result i32)
      (local i32)
      block ;; label = @1
        i32.const 1048584
        local.get 2
        local.get 3
        call $<wee_alloc::WeeAlloc as core::alloc::global::GlobalAlloc>::alloc
        local.tee 4
        i32.eqz
        br_if 0 (;@1;)
        local.get 4
        local.get 0
        local.get 1
        local.get 3
        local.get 1
        local.get 3
        i32.lt_u
        select
        memory.copy
        i32.const 1048584
        local.get 0
        local.get 2
        local.get 1
        call $<wee_alloc::WeeAlloc as core::alloc::global::GlobalAlloc>::dealloc
      end
      local.get 4
    )
    (func $miden:add-package/add-interface@1.0.0#add (;4;) (type 1) (param i32 i32) (result i32)
      call $wit_bindgen_rt::run_ctors_once
      local.get 1
      local.get 0
      i32.add
    )
    (func $wit_bindgen_rt::cabi_realloc (;5;) (type 2) (param i32 i32 i32 i32) (result i32)
      block ;; label = @1
        block ;; label = @2
          block ;; label = @3
            local.get 1
            br_if 0 (;@3;)
            local.get 3
            i32.eqz
            br_if 2 (;@1;)
            i32.const 0
            i32.load8_u offset=1048588
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
    (func $wit_bindgen_rt::run_ctors_once (;6;) (type 0)
      block ;; label = @1
        i32.const 0
        i32.load8_u offset=1048589
        br_if 0 (;@1;)
        call $__wasm_call_ctors
        i32.const 0
        i32.const 1
        i32.store8 offset=1048589
      end
    )
    (func $wee_alloc::alloc_first_fit (;7;) (type 3) (param i32 i32 i32) (result i32)
      (local i32 i32 i32 i32 i32 i32 i32)
      block ;; label = @1
        local.get 2
        i32.load
        local.tee 3
        br_if 0 (;@1;)
        i32.const 0
        return
      end
      local.get 1
      i32.const -1
      i32.add
      local.set 4
      i32.const 0
      local.get 1
      i32.sub
      local.set 5
      local.get 0
      i32.const 2
      i32.shl
      local.set 6
      loop ;; label = @1
        block ;; label = @2
          block ;; label = @3
            local.get 3
            i32.load offset=8
            local.tee 1
            i32.const 1
            i32.and
            br_if 0 (;@3;)
            local.get 3
            i32.const 8
            i32.add
            local.set 0
            br 1 (;@2;)
          end
          loop ;; label = @3
            local.get 3
            local.get 1
            i32.const -2
            i32.and
            i32.store offset=8
            block ;; label = @4
              block ;; label = @5
                local.get 3
                i32.load offset=4
                local.tee 7
                i32.const -4
                i32.and
                local.tee 0
                br_if 0 (;@5;)
                i32.const 0
                local.set 8
                br 1 (;@4;)
              end
              i32.const 0
              local.get 0
              local.get 0
              i32.load8_u
              i32.const 1
              i32.and
              select
              local.set 8
            end
            block ;; label = @4
              local.get 3
              i32.load
              local.tee 1
              i32.const -4
              i32.and
              local.tee 9
              i32.eqz
              br_if 0 (;@4;)
              local.get 1
              i32.const 2
              i32.and
              br_if 0 (;@4;)
              local.get 9
              local.get 9
              i32.load offset=4
              i32.const 3
              i32.and
              local.get 0
              i32.or
              i32.store offset=4
              local.get 3
              i32.load offset=4
              local.tee 7
              i32.const -4
              i32.and
              local.set 0
              local.get 3
              i32.load
              local.set 1
            end
            block ;; label = @4
              local.get 0
              i32.eqz
              br_if 0 (;@4;)
              local.get 0
              local.get 0
              i32.load
              i32.const 3
              i32.and
              local.get 1
              i32.const -4
              i32.and
              i32.or
              i32.store
              local.get 3
              i32.load offset=4
              local.set 7
              local.get 3
              i32.load
              local.set 1
            end
            local.get 3
            local.get 7
            i32.const 3
            i32.and
            i32.store offset=4
            local.get 3
            local.get 1
            i32.const 3
            i32.and
            i32.store
            block ;; label = @4
              local.get 1
              i32.const 2
              i32.and
              i32.eqz
              br_if 0 (;@4;)
              local.get 8
              local.get 8
              i32.load
              i32.const 2
              i32.or
              i32.store
            end
            local.get 2
            local.get 8
            i32.store
            local.get 8
            local.set 3
            local.get 8
            i32.load offset=8
            local.tee 1
            i32.const 1
            i32.and
            br_if 0 (;@3;)
          end
          local.get 8
          i32.const 8
          i32.add
          local.set 0
          local.get 8
          local.set 3
        end
        block ;; label = @2
          local.get 3
          i32.load
          i32.const -4
          i32.and
          local.tee 8
          local.get 0
          i32.sub
          local.get 6
          i32.lt_u
          br_if 0 (;@2;)
          block ;; label = @3
            block ;; label = @4
              local.get 0
              i32.const 72
              i32.add
              local.get 8
              local.get 6
              i32.sub
              local.get 5
              i32.and
              local.tee 8
              i32.le_u
              br_if 0 (;@4;)
              local.get 4
              local.get 0
              i32.and
              br_if 2 (;@2;)
              local.get 2
              local.get 1
              i32.const -4
              i32.and
              i32.store
              local.get 3
              i32.load
              local.set 0
              local.get 3
              local.set 1
              br 1 (;@3;)
            end
            i32.const 0
            local.set 7
            local.get 8
            i32.const 0
            i32.store
            local.get 8
            i32.const -8
            i32.add
            local.tee 1
            i64.const 0
            i64.store align=4
            local.get 1
            local.get 3
            i32.load
            i32.const -4
            i32.and
            i32.store
            block ;; label = @4
              local.get 3
              i32.load
              local.tee 9
              i32.const -4
              i32.and
              local.tee 8
              i32.eqz
              br_if 0 (;@4;)
              local.get 9
              i32.const 2
              i32.and
              br_if 0 (;@4;)
              local.get 8
              local.get 8
              i32.load offset=4
              i32.const 3
              i32.and
              local.get 1
              i32.or
              i32.store offset=4
              local.get 1
              i32.load offset=4
              i32.const 3
              i32.and
              local.set 7
            end
            local.get 1
            local.get 7
            local.get 3
            i32.or
            i32.store offset=4
            local.get 0
            local.get 0
            i32.load
            i32.const -2
            i32.and
            i32.store
            local.get 3
            local.get 3
            i32.load
            local.tee 0
            i32.const 3
            i32.and
            local.get 1
            i32.or
            local.tee 8
            i32.store
            block ;; label = @4
              local.get 0
              i32.const 2
              i32.and
              br_if 0 (;@4;)
              local.get 1
              i32.load
              local.set 0
              br 1 (;@3;)
            end
            local.get 3
            local.get 8
            i32.const -3
            i32.and
            i32.store
            local.get 1
            i32.load
            i32.const 2
            i32.or
            local.set 0
          end
          local.get 1
          local.get 0
          i32.const 1
          i32.or
          i32.store
          local.get 1
          i32.const 8
          i32.add
          return
        end
        local.get 2
        local.get 1
        i32.store
        local.get 1
        local.set 3
        local.get 1
        br_if 0 (;@1;)
      end
      i32.const 0
    )
    (func $<wee_alloc::WeeAlloc as core::alloc::global::GlobalAlloc>::alloc (;8;) (type 3) (param i32 i32 i32) (result i32)
      (local i32 i32 i32)
      global.get $__stack_pointer
      i32.const 16
      i32.sub
      local.tee 3
      global.set $__stack_pointer
      block ;; label = @1
        block ;; label = @2
          local.get 2
          br_if 0 (;@2;)
          local.get 1
          local.set 2
          br 1 (;@1;)
        end
        local.get 3
        local.get 0
        i32.load
        i32.store offset=12
        block ;; label = @2
          local.get 2
          i32.const 3
          i32.add
          local.tee 4
          i32.const 2
          i32.shr_u
          local.tee 5
          local.get 1
          local.get 3
          i32.const 12
          i32.add
          call $wee_alloc::alloc_first_fit
          local.tee 2
          br_if 0 (;@2;)
          block ;; label = @3
            local.get 4
            i32.const -4
            i32.and
            local.tee 2
            local.get 1
            i32.const 3
            i32.shl
            i32.const 512
            i32.add
            local.tee 4
            local.get 2
            local.get 4
            i32.gt_u
            select
            i32.const 65543
            i32.add
            local.tee 4
            i32.const 16
            i32.shr_u
            memory.grow
            local.tee 2
            i32.const -1
            i32.ne
            br_if 0 (;@3;)
            i32.const 0
            local.set 2
            br 1 (;@2;)
          end
          local.get 2
          i32.const 16
          i32.shl
          local.tee 2
          i32.const 0
          i32.store offset=4
          local.get 2
          local.get 3
          i32.load offset=12
          i32.store offset=8
          local.get 2
          local.get 2
          local.get 4
          i32.const -65536
          i32.and
          i32.add
          i32.const 2
          i32.or
          i32.store
          local.get 3
          local.get 2
          i32.store offset=12
          local.get 5
          local.get 1
          local.get 3
          i32.const 12
          i32.add
          call $wee_alloc::alloc_first_fit
          local.set 2
        end
        local.get 0
        local.get 3
        i32.load offset=12
        i32.store
      end
      local.get 3
      i32.const 16
      i32.add
      global.set $__stack_pointer
      local.get 2
    )
    (func $<wee_alloc::WeeAlloc as core::alloc::global::GlobalAlloc>::dealloc (;9;) (type 4) (param i32 i32 i32 i32)
      (local i32 i32 i32 i32 i32 i32 i32)
      block ;; label = @1
        local.get 1
        i32.eqz
        br_if 0 (;@1;)
        local.get 3
        i32.eqz
        br_if 0 (;@1;)
        local.get 0
        i32.load
        local.set 4
        local.get 1
        i32.const 0
        i32.store
        local.get 1
        i32.const -8
        i32.add
        local.tee 3
        local.get 3
        i32.load
        local.tee 5
        i32.const -2
        i32.and
        local.tee 6
        i32.store
        block ;; label = @2
          block ;; label = @3
            block ;; label = @4
              block ;; label = @5
                local.get 1
                i32.const -4
                i32.add
                local.tee 7
                i32.load
                i32.const -4
                i32.and
                local.tee 8
                i32.eqz
                br_if 0 (;@5;)
                local.get 8
                i32.load
                local.tee 9
                i32.const 1
                i32.and
                br_if 0 (;@5;)
                block ;; label = @6
                  block ;; label = @7
                    block ;; label = @8
                      local.get 5
                      i32.const -4
                      i32.and
                      local.tee 10
                      br_if 0 (;@8;)
                      local.get 8
                      local.set 1
                      br 1 (;@7;)
                    end
                    local.get 8
                    local.set 1
                    local.get 5
                    i32.const 2
                    i32.and
                    br_if 0 (;@7;)
                    local.get 10
                    local.get 10
                    i32.load offset=4
                    i32.const 3
                    i32.and
                    local.get 8
                    i32.or
                    i32.store offset=4
                    local.get 3
                    i32.load
                    local.set 6
                    local.get 7
                    i32.load
                    local.tee 5
                    i32.const -4
                    i32.and
                    local.tee 1
                    i32.eqz
                    br_if 1 (;@6;)
                    local.get 1
                    i32.load
                    local.set 9
                  end
                  local.get 1
                  local.get 6
                  i32.const -4
                  i32.and
                  local.get 9
                  i32.const 3
                  i32.and
                  i32.or
                  i32.store
                  local.get 7
                  i32.load
                  local.set 5
                  local.get 3
                  i32.load
                  local.set 6
                end
                local.get 7
                local.get 5
                i32.const 3
                i32.and
                i32.store
                local.get 3
                local.get 6
                i32.const 3
                i32.and
                i32.store
                local.get 6
                i32.const 2
                i32.and
                i32.eqz
                br_if 1 (;@4;)
                local.get 8
                local.get 8
                i32.load
                i32.const 2
                i32.or
                i32.store
                br 1 (;@4;)
              end
              local.get 5
              i32.const 2
              i32.and
              br_if 1 (;@3;)
              local.get 5
              i32.const -4
              i32.and
              local.tee 5
              i32.eqz
              br_if 1 (;@3;)
              local.get 5
              i32.load8_u
              i32.const 1
              i32.and
              br_if 1 (;@3;)
              local.get 1
              local.get 5
              i32.load offset=8
              i32.const -4
              i32.and
              i32.store
              local.get 5
              local.get 3
              i32.const 1
              i32.or
              i32.store offset=8
            end
            local.get 4
            local.set 3
            br 1 (;@2;)
          end
          local.get 1
          local.get 4
          i32.store
        end
        local.get 0
        local.get 3
        i32.store
      end
    )
    (func $cabi_realloc (;10;) (type 2) (param i32 i32 i32 i32) (result i32)
      local.get 0
      local.get 1
      local.get 2
      local.get 3
      call $wit_bindgen_rt::cabi_realloc
    )
    (table (;0;) 3 3 funcref)
    (memory (;0;) 17)
    (global $__stack_pointer (;0;) (mut i32) i32.const 1048576)
    (export "memory" (memory 0))
    (export "miden:add-package/add-interface@1.0.0#add" (func $miden:add-package/add-interface@1.0.0#add))
    (export "cabi_realloc" (func $cabi_realloc))
    (export "cabi_realloc_wit_bindgen_0_28_0" (func $wit_bindgen_rt::cabi_realloc))
    (elem (;0;) (i32.const 1) func $add_wasm_component::bindings::__link_custom_section_describing_imports $cabi_realloc)
    (data $.rodata (;0;) (i32.const 1048576) "\01\00\00\00\02\00\00\00")
  )
  (core instance (;0;) (instantiate 0))
  (alias core export 0 "memory" (core memory (;0;)))
  (alias core export 0 "cabi_realloc" (core func (;0;)))
  (type (;0;) (func (param "a" u32) (param "b" u32) (result u32)))
  (alias core export 0 "miden:add-package/add-interface@1.0.0#add" (core func (;1;)))
  (func (;0;) (type 0) (canon lift (core func 1)))
  (component (;0;)
    (type (;0;) (func (param "a" u32) (param "b" u32) (result u32)))
    (import "import-func-add" (func (;0;) (type 0)))
    (type (;1;) (func (param "a" u32) (param "b" u32) (result u32)))
    (export (;1;) "add" (func 0) (func (type 1)))
  )
  (instance (;0;) (instantiate 0
      (with "import-func-add" (func 0))
    )
  )
  (export (;1;) "miden:add-package/add-interface@1.0.0" (instance 0))
)