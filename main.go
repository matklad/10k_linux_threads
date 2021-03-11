package main

import (
	"sync"
	"time"
)

func main() {
	var wg sync.WaitGroup
	for i := uint32(0); i < 10_000; i++ {
		wg.Add(1)
		go func() {
			defer wg.Done()
			bad_hash := (i * 2654435761) % 200_000
			time.Sleep(time.Duration(bad_hash) * time.Microsecond)
			for j := 0; j < 1000; j++ {
				time.Sleep(10 * time.Millisecond)
			}
		}()
	}
	wg.Wait()
}
