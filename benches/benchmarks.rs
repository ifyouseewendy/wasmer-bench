#[macro_use]
extern crate criterion;

extern crate wasmer_runtime_core;

extern crate wasmer_clif_backend;
extern crate wasmer_llvm_backend;

use std::str;

use wasmer_runtime_core::{import::ImportObject, Func};

static WASM: &'static [u8] = include_bytes!(
    "../benchmarks/target/wasm32-unknown-unknown/release/wasm_bench_benchmarks.wasm"
);

use criterion::black_box;
use criterion::Criterion;
use wasm_bench_benchmarks;
use wasmer_clif_backend::CraneliftCompiler;
use wasmer_llvm_backend::LLVMCompiler;

fn compile_benchmark(c: &mut Criterion) {
    c.bench_function("clif compile benchmarks", |b| {
        b.iter(|| {
            black_box(
                wasmer_runtime_core::compile_with(WASM, &CraneliftCompiler::new())
                    .expect("should compile"),
            )
        })
    });

    c.bench_function("llvm compile benchmarks", |b| {
        b.iter(|| {
            black_box(
                wasmer_runtime_core::compile_with(WASM, &LLVMCompiler::new())
                    .expect("should compile"),
            )
        })
    });
}

fn sum_benchmark(c: &mut Criterion) {
    c.bench_function("native sum 1, 2", |b| {
        b.iter(|| black_box(wasm_bench_benchmarks::sum(1, 2)))
    });

    c.bench_function("clif func.call sum 1, 2", |b| {
        let module = wasmer_runtime_core::compile_with(WASM, &CraneliftCompiler::new())
            .expect("should compile");
        let instance = module
            .instantiate(&ImportObject::new())
            .expect("should instantiate");
        let sum: Func<(i32, i32), i32> = instance.func("sum").unwrap();
        b.iter(|| black_box(sum.call(1, 2)))
    });

    c.bench_function("llvm func.call sum 1, 2", |b| {
        let module =
            wasmer_runtime_core::compile_with(WASM, &LLVMCompiler::new()).expect("should compile");
        let instance = module
            .instantiate(&ImportObject::new())
            .expect("should instantiate");
        let sum: Func<(i32, i32), i32> = instance.func("sum").unwrap();
        b.iter(|| black_box(sum.call(1, 2)))
    });
}

fn fib_benchmark(c: &mut Criterion) {
    c.bench_function("native fib 30", |b| {
        b.iter(|| black_box(wasm_bench_benchmarks::fib(30)))
    });

    c.bench_function("clif func.call fib 30", |b| {
        let module = wasmer_runtime_core::compile_with(WASM, &CraneliftCompiler::new())
            .expect("should compile");
        let instance = module
            .instantiate(&ImportObject::new())
            .expect("should instantiate");
        let fib: Func<(i64), i64> = instance.func("fib").unwrap();
        b.iter(|| black_box(fib.call(30)))
    });

    c.bench_function("llvm func.call fib 30", |b| {
        let module =
            wasmer_runtime_core::compile_with(WASM, &LLVMCompiler::new()).expect("should compile");
        let instance = module
            .instantiate(&ImportObject::new())
            .expect("should instantiate");
        let fib: Func<(i64), i64> = instance.func("fib").unwrap();
        b.iter(|| black_box(fib.call(30)))
    });
}

fn nbody_benchmark(c: &mut Criterion) {
    c.bench_function("native nbody", |b| {
        b.iter(|| black_box(unsafe { wasm_bench_benchmarks::nbody::bench(5000) }))
    });

    c.bench_function("clif func.call nbody", |b| {
        let module = wasmer_runtime_core::compile_with(WASM, &CraneliftCompiler::new())
            .expect("should compile");
        let instance = module
            .instantiate(&ImportObject::new())
            .expect("should instantiate");
        let func: Func<(i32)> = instance.func("bench").unwrap();
        b.iter(|| black_box(func.call(5000)))
    });

    c.bench_function("llvm func.call nbody", |b| {
        let module =
            wasmer_runtime_core::compile_with(WASM, &LLVMCompiler::new()).expect("should compile");
        let instance = module
            .instantiate(&ImportObject::new())
            .expect("should instantiate");
        let func: Func<(i32)> = instance.func("bench").unwrap();
        b.iter(|| black_box(func.call(5000)))
    });
}

// criterion_group!(benches, nbody_benchmark);

criterion_group!(
    benches,
    fib_benchmark,
    sum_benchmark,
    nbody_benchmark,
    compile_benchmark
);
criterion_main!(benches);

#[cfg(test)]
mod tests {

    #[test]
    fn test_sum() {
        assert_eq!(3, wasm_bench_benchmarks::sum(1, 2));
    }

}