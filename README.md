`cargo bench`

```
test borrow_collect_join    ... bench:       2,919 ns/iter (+/- 141)
test borrow_itertools_join  ... bench:       9,138 ns/iter (+/- 1,286)
test consume_collect_join   ... bench:      17,472 ns/iter (+/- 1,690)
test consume_itertools_join ... bench:      24,025 ns/iter (+/- 1,160)
```

- https://stackoverflow.com/questions/28311868/what-is-the-equivalent-of-the-join-operator-over-a-vector-of-strings#comment102529891_51191466
- https://gist.github.com/green-s/fbd0d374b290781ac9b3f8ff03e3245d
- https://stackoverflow.com/questions/36941851/whats-an-idiomatic-way-to-print-an-iterator-separated-by-spaces-in-rust
