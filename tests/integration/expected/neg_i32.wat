(module $test_rust_4d99fac9eee91dee5488ef87e46cd372efb54cac7088374c284c7fe1d592975b.wasm
  (type (;0;) (func (param i32) (result i32)))
  (func $entrypoint (;0;) (type 0) (param i32) (result i32)
    i32.const 0
    local.get 0
    i32.sub
  )
  (memory (;0;) 16)
  (global $__stack_pointer (;0;) (mut i32) i32.const 1048576)
  (global (;1;) i32 i32.const 1048576)
  (global (;2;) i32 i32.const 1048576)
  (export "memory" (memory 0))
  (export "entrypoint" (func $entrypoint))
  (export "__data_end" (global 1))
  (export "__heap_base" (global 2))
)