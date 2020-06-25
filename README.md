# wordsquare
Solve wordsquares

## Benchmarks

```
running 5 tests
test tests::bench_solve_square_naive                          ... bench:   2,346,800 ns/iter (+/- 305,054)
test tests::bench_solve_square_naive_hash_first_letter        ... bench:   1,171,656 ns/iter (+/- 277,578)
test tests::bench_solve_square_reverse                        ... bench:   1,303,564 ns/iter (+/- 184,705)
test tests::bench_solve_square_reverse_hash_first_letter      ... bench:     933,068 ns/iter (+/- 228,583)
test tests::bench_solve_square_reverse_hash_first_two_letters ... bench:   1,342,177 ns/iter (+/- 319,067)

test result: ok. 0 passed; 0 failed; 0 ignored; 5 measured; 0 filtered out
```
