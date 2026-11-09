#!/bin/sh
#
# benchmark del porting Rust (vedi rust/README.md); i binari vengono
# compilati da cargo in rust/target/release. Confrontare con
# benchmark.sh (C) e benchmark_go.sh (Go, branch go-lang).
#
# Rust non ha i livelli -O di gcc a runtime: il profilo release è
# -O3+LTO; la manopola tipo -march=native è -C target-cpu=native.

run(){
	name=$1
	(cd rust/target/release && time -f "%e $name" "./$name")
}

benchmark(){
	run i_TTC_dispatch_switch
	run i_TTC_dispatch_if
	run i_TTC_dispatch_function_table
	run i_DCT_dispatch_function_call
	run i_TTC_dispatch_channel
	run i_TTC_dispatch_threads
	run native_function_init
	run native_function_call
	run native_baseline
	run native_transpiled_run400k
}

clear
echo "benchmark del porting Rust (per il C: ./benchmark.sh)"
echo "remember to set the cpu performance governor, and remove other cpu usage"
echo 'echo performance | sudo tee /sys/devices/system/cpu/cpu*/cpufreq/scaling_governor'

echo "--- build: release (default) ---"
( cd rust && cargo build --release ) >/dev/null 2>&1
benchmark

echo "--- build: release + target-cpu=native (analogo di -march=native) ---"
( cd rust && RUSTFLAGS="-C target-cpu=native" cargo build --release ) >/dev/null 2>&1
benchmark
