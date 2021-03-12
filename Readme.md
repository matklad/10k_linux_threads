Creating 10_000 threads on Linux

```
λ rustc main.rs -C opt-level=3 && ./t ./main
real 10.35s
user 5.14s
sys  16.04s
rss  94628k

λ go build main.go && ./t ./main
real 10.92s
user 13.00s
sys  0.50s
rss  34612k

λ cargo build -q --release && ./t ./target/release/main --pin-to-core
real 10.35s
user 3.00s
sys  8.94s
rss  94916k
```

See [this post](https://matklad.github.io/2021/03/12/goroutines-are-not-significantly-lighter-than-threads.html) for discussion.
