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
test chunk::tests::bench_chunk                         ... bench:         853 ns/iter (+/- 93)
test compact::tests::bench_compact                     ... bench:          85 ns/iter (+/- 10)
test concat::tests::bench_concat                       ... bench:         478 ns/iter (+/- 71)
test difference::tests::bench_difference               ... bench:         503 ns/iter (+/- 31)
test divide::tests::bench_divide                       ... bench:           0 ns/iter (+/- 0)
test drop::tests::bench_drop                           ... bench:         193 ns/iter (+/- 11)
test fill::tests::bench_fill                           ... bench:         199 ns/iter (+/- 23)
test find_index::tests::bench_find_index               ... bench:          82 ns/iter (+/- 20)
test find_last_index::tests::bench_find_last_index     ... bench:         223 ns/iter (+/- 49)
test first::tests::bench_first                         ... bench:          81 ns/iter (+/- 8)
test flatten::tests::bench_flatten                     ... bench:       1,197 ns/iter (+/- 140)
test floor::tests::bench_floor                         ... bench:           0 ns/iter (+/- 0)
test from_pairs::tests::bench_from_pairs               ... bench:         487 ns/iter (+/- 70)
test initial::tests::bench_initial                     ... bench:          79 ns/iter (+/- 9)
test intersection::tests::bench_intersection           ... bench:         474 ns/iter (+/- 34)
test join::tests::bench_join                           ... bench:       1,866 ns/iter (+/- 131)
test last::tests::bench_last                           ... bench:          81 ns/iter (+/- 13)
test max::tests::bench_max                             ... bench:          82 ns/iter (+/- 8)
test mean::tests::bench_mean                           ... bench:         221 ns/iter (+/- 40)
test min::tests::bench_min                             ... bench:          83 ns/iter (+/- 13)
test multiply::tests::bench_multiply                   ... bench:           0 ns/iter (+/- 0)
test nth::tests::bench_nth                             ... bench:          80 ns/iter (+/- 9)
test pull::tests::bench_pull                           ... bench:         204 ns/iter (+/- 24)
test pull_at::tests::bench_pull_at                     ... bench:         302 ns/iter (+/- 58)
test remove::tests::bench_remove                       ... bench:       1,219 ns/iter (+/- 140)
test reverse::tests::bench_first                       ... bench:         205 ns/iter (+/- 23)
test round::tests::bench_round                         ... bench:           0 ns/iter (+/- 0)
test slice::tests::bench_slice                         ... bench:         192 ns/iter (+/- 41)
test sorted_index::tests::bench_sorted_index           ... bench:          82 ns/iter (+/- 8)
test sorted_last_index::tests::bench_sorted_last_index ... bench:          83 ns/iter (+/- 12)
test sorted_uniq::tests::bench_sorted_uniq             ... bench:         224 ns/iter (+/- 390)
test subtract::tests::bench_subtract                   ... bench:           0 ns/iter (+/- 0)
test sum::tests::bench_sum                             ... bench:         218 ns/iter (+/- 23)
test tail::tests::bench_tail                           ... bench:         201 ns/iter (+/- 10)
test take::tests::bench_take                           ... bench:         208 ns/iter (+/- 17)
test take_right::tests::bench_take_right               ... bench:         190 ns/iter (+/- 30)
test union::tests::bench_union                         ... bench:         488 ns/iter (+/- 57)
test unzip::tests::bench_unzip                         ... bench:       1,224 ns/iter (+/- 150)
test without::tests::bench_without                     ... bench:         375 ns/iter (+/- 63)
test xor::tests::bench_xor                             ... bench:       1,131 ns/iter (+/- 629)
test zip::tests::bench_zip                             ... bench:       1,209 ns/iter (+/- 40)
```