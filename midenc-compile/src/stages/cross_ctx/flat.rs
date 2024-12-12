//! Convertion between the Wasm CM types and the Miden cross-context ABI types.
//!
//! See https://github.com/WebAssembly/component-model/blob/main/design/mvp/CanonicalABI.md#flattening
//! and https://github.com/WebAssembly/tool-conventions/blob/main/BasicCABI.md
//! for the Wasm CM <-> core Wasm types conversion rules.

use midenc_hir::{
    types::Abi, AbiParam, ArgumentPurpose, CallConv, FunctionType, Linkage, Signature, StructType,
    Type,
};

/// Lowering the import or lifting the export function
pub enum FlatteningDirection {
    Lift,
    Lower,
}

/// Flattens the given type into a list of ABI parameters.
pub fn flatten_type(ty: &Type) -> Vec<AbiParam> {
    // see https://github.com/WebAssembly/component-model/blob/main/design/mvp/CanonicalABI.md#flattening
    match ty {
        Type::Unknown => todo!(),
        Type::Unit => todo!(),
        Type::Never => todo!(),
        Type::I1 => todo!(),
        Type::I8 => todo!(),
        Type::U8 => todo!(),
        Type::I16 => todo!(),
        Type::U16 => todo!(),
        Type::I32 => todo!(),
        Type::U32 => todo!(),
        Type::I64 => todo!(),
        Type::U64 => todo!(),
        Type::I128 => todo!(),
        Type::U128 => todo!(),
        Type::U256 => todo!(),
        Type::F64 => todo!(),
        Type::Felt => vec![AbiParam::new(Type::Felt)],
        Type::Ptr(_) => todo!(),
        Type::NativePtr(..) => todo!(),
        Type::Struct(struct_ty) => {
            struct_ty.fields().iter().flat_map(|field| flatten_type(&field.ty)).collect()
        }
        Type::Array(..) => todo!(),
        Type::List(_) => todo!(),
    }
}

/// Flattens the given list of types into a list of ABI parameters.
pub fn flatten_types(tys: &[Type]) -> Vec<AbiParam> {
    tys.iter().flat_map(flatten_type).collect()
}

/// Flattens the given Wasm CM level function type
pub fn flatten_function_type(func_ty: &FunctionType, dir: FlatteningDirection) -> Signature {
    // from https://github.com/WebAssembly/component-model/blob/main/design/mvp/CanonicalABI.md#flattening
    //
    // For a variety of practical reasons, we need to limit the total number of flattened
    // parameters and results, falling back to storing everything in linear memory. The number of
    // flattened results is currently limited to 1 due to various parts of the toolchain (notably
    // the C ABI) not yet being able to express multi-value returns. Hopefully this limitation is
    // temporary and can be lifted before the Component Model is fully standardized.
    assert_eq!(func_ty.abi, Abi::Wasm, "expected Wasm CM type");
    const MAX_FLAT_PARAMS: usize = 16;
    const MAX_FLAT_RESULTS: usize = 1;
    let mut flat_params = flatten_types(&func_ty.params);
    let mut flat_results = flatten_types(&func_ty.results);
    if flat_params.len() > MAX_FLAT_PARAMS {
        // from https://github.com/WebAssembly/component-model/blob/main/design/mvp/CanonicalABI.md#flattening
        //
        // When there are too many flat values, in general, a single `i32` pointer can be passed instead
        // (pointing to a tuple in linear memory). When lowering into linear memory, this requires the
        // Canonical ABI to call `realloc` to allocate space to put the tuple.
        let tuple = Type::Struct(StructType::new(func_ty.params.clone()));
        flat_params = vec![AbiParam::sret(Type::Ptr(tuple.into()))];
    }
    if flat_results.len() > MAX_FLAT_RESULTS {
        // from https://github.com/WebAssembly/component-model/blob/main/design/mvp/CanonicalABI.md#flattening
        //
        // As an optimization, when lowering the return value of an imported function (via `canon
        // lower`), the caller can have already allocated space for the return value (e.g.,
        // efficiently on the stack), passing in an `i32` pointer as an parameter instead of
        // returning an `i32` as a return value.
        assert_eq!(func_ty.results.len(), 1, "expected a single result");
        let result = func_ty.results.first().expect("unexpected empty results").clone();
        match dir {
            FlatteningDirection::Lift => {
                flat_results = vec![AbiParam::sret(Type::Ptr(result.into()))];
            }
            FlatteningDirection::Lower => {
                flat_params.push(AbiParam::sret(Type::Ptr(result.into())));
                flat_results = vec![];
            }
        }
    }
    Signature {
        params: flat_params,
        results: flat_results,
        // TODO: add and use CallConv::CrossCtx here
        cc: CallConv::SystemV,
        linkage: Linkage::External,
    }
}

/// Checks if the given function signature needs to be transformed, i.e., if it contains a pointer
pub fn needs_transformation(sig: &Signature) -> bool {
    sig.params().iter().any(|param| param.purpose == ArgumentPurpose::StructReturn)
        || sig
            .results()
            .iter()
            .any(|result| result.purpose == ArgumentPurpose::StructReturn)
}

/// Asserts that the given core Wasm signature is equivalent to the given flattened signature
/// This checks that we flattened the Wasm CM function type correctly.
pub fn assert_core_wasm_signature_equivalence(
    wasm_core_sig: &Signature,
    flattened_sig: &Signature,
) {
    assert_eq!(
        wasm_core_sig.params().len(),
        flattened_sig.params().len(),
        "expected the same number of params"
    );
    assert_eq!(
        wasm_core_sig.results().len(),
        flattened_sig.results().len(),
        "expected the same number of results"
    );
    for (wasm_core_param, flattened_param) in
        wasm_core_sig.params().iter().zip(flattened_sig.params())
    {
        assert_eq!(wasm_core_param.ty, flattened_param.ty, "expected the same param type");
    }
}
