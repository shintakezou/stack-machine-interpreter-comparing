// Porting Go di native_baseline.c
// baseline number crunching di test3: stessa elaborazione espressa in
// modo idiomatico, per stimare quanto il compilatore riesce a
// semplificare (gcc a -Ofast la azzera del tutto; vediamo gc).
package main

func run() {
	r := 0.0
	for i := 0; i < 2000; i++ {
		r -= 1
	}
	r = r * 2
	_ = r
}

func main() {
	for times := 400000; times > 0; times-- {
		run()
	}
}
