# Colon
A lodash inspired library for Rust.

Colon makes Rust easier by taking the hassle out of working with arrays,
numbers, etc. 

## Implementation Status

### Array
- [x] chunk
- [x] compact
- [x] concat
- [x] difference
- [ ] difference_by
- [ ] difference_with
- [x] drop
- [ ] drop_right
- [ ] drop_right_while
- [ ] drop_while
- [x] fill
- [x] find_index
- [x] find_last_index
- [x] first
- [x] flatten
- [x] from_pairs
- [x] head
- [x] index_of
- [x] initial
- [x] intersection
- [ ] intersection_by
- [ ] intersection_with
- [x] join
- [x] last
- [x] last_index_of
- [x] nth
- [x] pull
- [ ] pull_all
- [ ] pull_all_by
- [ ] pull_all_with
- [x] pull_at
- [x] remove
- [x] reverse
- [x] slice
- [x] sorted_index
- [ ] sorted_index_by
- [ ] sorted_index_of
- [x] sorted_last_index
- [ ] sorted_last_index_by
- [ ] sorted_last_index_of
- [x] sorted_uniq
- [ ] sorted_uniq_by
- [x] tail
- [x] take
- [ ] take_while
- [x] take_right
- [ ] take_right_while
- [x] union
- [ ] union_by
- [ ] union_with
- [x] unzip
- [ ] unzip_with
- [x] without
- [x] xor
- [ ] xor_by
- [ ] xor_with
- [x] zip
- [ ] zip_object
- [ ] zip_object_deep
- [ ] zip_with

### Maths
- [x] add
- [x] ceil
- [x] divide
- [x] floor
- [x] max
- [ ] max_by
- [x] mean
- [ ] mean_by
- [x] min
- [x] multiply
- [x] round
- [x] subtract
- [x] sum
- [ ] sum_by

### Cargo Bench Result on Macbook Pro 2016
```
test add::tests::bench_add                             ... bench:           0 ns/iter (+/- 0)
test ceil::tests::bench_ceil                           ... bench:           0 ns/iter (+/- 0)
test chunk::tests::bench_chunk                         ... bench:         850 ns/iter (+/- 132)
test compact::tests::bench_compat                      ... bench:          90 ns/iter (+/- 12)
test concat::tests::bench_concat                       ... bench:         391 ns/iter (+/- 269)
test difference::tests::bench_difference               ... bench:         486 ns/iter (+/- 88)
test divide::tests::bench_divide                       ... bench:           0 ns/iter (+/- 0)
test drop::tests::bench_drop                           ... bench:         251 ns/iter (+/- 88)
test fill::tests::bench_fill                           ... bench:         207 ns/iter (+/- 127)
test find_index::tests::bench_find_index               ... bench:          87 ns/iter (+/- 237)
test find_last_index::tests::bench_find_last_index     ... bench:          87 ns/iter (+/- 86)
test first::tests::bench_first                         ... bench:          77 ns/iter (+/- 13)
test flatten::tests::bench_flatten                     ... bench:       1,853 ns/iter (+/- 177)
test floor::tests::bench_floor                         ... bench:           0 ns/iter (+/- 0)
test from_pairs::tests::bench_from_pairs               ... bench:         512 ns/iter (+/- 136)
test initial::tests::bench_initial                     ... bench:          77 ns/iter (+/- 123)
test intersection::tests::bench_intersection           ... bench:         875 ns/iter (+/- 63)
test join::tests::bench_join                           ... bench:       2,053 ns/iter (+/- 1,251)
test last::tests::bench_last                           ... bench:          82 ns/iter (+/- 15)
test max::tests::bench_max                             ... bench:          80 ns/iter (+/- 51)
test mean::tests::bench_mean                           ... bench:         233 ns/iter (+/- 87)
test min::tests::bench_min                             ... bench:          77 ns/iter (+/- 11)
test multiply::tests::bench_multiply                   ... bench:           0 ns/iter (+/- 0)
test nth::tests::bench_nth                             ... bench:          78 ns/iter (+/- 12)
test pull::tests::bench_pull                           ... bench:         242 ns/iter (+/- 61)
test pull_at::tests::bench_pull_at                     ... bench:         366 ns/iter (+/- 45)
test remove::tests::bench_remove                       ... bench:       1,206 ns/iter (+/- 158)
test reverse::tests::bench_first                       ... bench:           4 ns/iter (+/- 0)
test round::tests::bench_round                         ... bench:           0 ns/iter (+/- 0)
test slice::tests::bench_slice                         ... bench:         203 ns/iter (+/- 51)
test sorted_index::tests::bench_sorted_index           ... bench:           0 ns/iter (+/- 0)
test sorted_last_index::tests::bench_sorted_last_index ... bench:           0 ns/iter (+/- 0)
test sorted_uniq::tests::bench_sorted_uniq             ... bench:         223 ns/iter (+/- 18)
test subtract::tests::bench_subtract                   ... bench:           0 ns/iter (+/- 0)
test sum::tests::bench_sum                             ... bench:         215 ns/iter (+/- 19)
test tail::tests::bench_tail                           ... bench:         196 ns/iter (+/- 48)
test take::tests::bench_take                           ... bench:         208 ns/iter (+/- 38)
test take_right::tests::bench_take_right               ... bench:         202 ns/iter (+/- 14)
test union::tests::bench_union                         ... bench:       1,099 ns/iter (+/- 114)
test unzip::tests::bench_unzip                         ... bench:       1,210 ns/iter (+/- 69)
test without::tests::bench_without                     ... bench:         353 ns/iter (+/- 26)
test xor::tests::bench_xor                             ... bench:       1,520 ns/iter (+/- 82)
test zip::tests::bench_zip                             ... bench:       1,209 ns/iter (+/- 54)
```