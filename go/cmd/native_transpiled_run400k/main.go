// Porting Go di native_transpiled_run400k.c
// esegue il programma transpilato 400k volte per il benchmark.
package main

import "stackmachine/transpiled"

func main() {
	for times := 400000; times > 0; times-- {
		transpiled.Run()
	}
}
