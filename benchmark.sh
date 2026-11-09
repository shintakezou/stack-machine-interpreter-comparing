#!/bin/sh
#
# naive benchmark across optimization levels.
# binaries are built via make into build/ (see Makefile).

Optimization='-O0'
compile() {
	echo "--- compile Optimization: $Optimization ---"
	make -B OPT="$Optimization" >/dev/null
}

run(){
	name=$1
	# run from inside build/: i_TTC_dispatch_function_table_shared
	# dlopen()s "./vm_shared.so" relative to the current directory
	(cd build && time -f "%e $name" "./$name")
}

benchmark(){
	run i_TTC_dispatch_computed_goto
	run i_TTC_dispatch_switch
	run i_TTC_dispatch_if
	run i_TTC_dispatch_function_table
	run i_DCT_dispatch_function_call
	run i_TTC_dispatch_function_table_shared
	run native_function_init
	run native_function_call
	run native_baseline
}


clear
echo "naive benchmak to against different machines:"
echo "remember to set the cpu performance governor, and remove other cpu usage"
echo 'echo performance | sudo tee /sys/devices/system/cpu/cpu*/cpufreq/scaling_governor'
#read -p "Press key to continue.. " -n1 -s

# make also builds the transpiler and the transpiled reference programs
# (native_transpiled_run400k_O2 / _Ofast, at their own fixed levels)

Optimization='-O0'
compile
benchmark

Optimization='-O1'
compile
benchmark

Optimization='-O2'
compile
benchmark
run native_transpiled_run400k_O2

Optimization='-O2 -fshort-enums'
compile
benchmark

Optimization='-O2 -march=native'
compile
benchmark

Optimization='-O3 -march=native -fshort-enums'
compile
benchmark

Optimization='-Ofast -march=native -fshort-enums'
compile
benchmark
run native_transpiled_run400k_Ofast
