Creating 10_000 threads on Linux

```
13:32:50|~/tmp/manythreads|HEAD⚡?
λ t cargo run --release
    Finished release [optimized] target(s) in 0.00s
     Running `target/release/ten-thousand-threads`
real 10.27s
user 4.61s
sys  18.46s
rss  88472k

13:33:23|~/tmp/manythreads|HEAD⚡?
λ t cargo run --release -- --pin-to-core
    Finished release [optimized] target(s) in 0.00s
     Running `target/release/ten-thousand-threads --pin-to-core`
real 10.32s
user 2.99s
sys  11.29s
rss  95380k

14:26:29|~/tmp/manythreads|master⚡?
λ go build main.go && t ./main
real 10.70s
user 13.26s
sys  0.47s
rss  34496k
```
