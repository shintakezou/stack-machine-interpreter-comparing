#!/bin/sh
#
# benchmark del porting Go; i binari sono compilati da make in build/go
# (vedi go/README.md). Confrontare con benchmark.sh per la versione C.
#
# Go non ha i livelli -O di gcc: l'unica manopola paragonabile a
# -march=native è GOAMD64 (v1 = default portabile, v3 ~ AVX2).

run(){
	name=$1
	(cd build/go && time -f "%e $name" "./$name")
}

benchmark(){
	run i_TTC_dispatch_switch
	run i_TTC_dispatch_if
	run i_TTC_dispatch_function_table
	run i_DCT_dispatch_function_call
	run i_TTC_dispatch_channel
	run i_TTC_dispatch_goroutines
	run native_function_init
	run native_function_call
	run native_baseline
	run native_transpiled_run400k
}

clear
echo "benchmark del porting Go (per il C: ./benchmark.sh)"
echo "remember to set the cpu performance governor, and remove other cpu usage"
echo 'echo performance | sudo tee /sys/devices/system/cpu/cpu*/cpufreq/scaling_governor'

echo "--- build: GOAMD64 default (v1) ---"
make go-all >/dev/null
benchmark

echo "--- build: GOAMD64=v3 (analogo di -march=native) ---"
GOAMD64=v3 make go-all >/dev/null
benchmark
