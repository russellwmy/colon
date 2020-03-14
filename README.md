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

# Cargo Bench Result on Macbook Pro 2016
```
test add::tests::bench_add                             ... bench:           0 ns/iter (+/- 0)
test ceil::tests::bench_ceil                           ... bench:           0 ns/iter (+/- 0)
test chunk::tests::bench_chunk                         ... bench:         798 ns/iter (+/- 101)
test compact::tests::bench_compat                      ... bench:          81 ns/iter (+/- 6)
test concat::tests::bench_concat                       ... bench:         351 ns/iter (+/- 49)
test difference::tests::bench_difference               ... bench:       1,198 ns/iter (+/- 70)
test divide::tests::bench_divide                       ... bench:           0 ns/iter (+/- 0)
test drop::tests::bench_drop                           ... bench:         201 ns/iter (+/- 35)
test fill::tests::bench_fill                           ... bench:         201 ns/iter (+/- 26)
test find_index::tests::bench_find_index               ... bench:          75 ns/iter (+/- 9)
test find_last_index::tests::bench_find_last_index     ... bench:          82 ns/iter (+/- 4)
test first::tests::bench_first                         ... bench:          73 ns/iter (+/- 7)
test flatten::tests::bench_flatten                     ... bench:       1,750 ns/iter (+/- 119)
test floor::tests::bench_floor                         ... bench:           0 ns/iter (+/- 0)
test from_pairs::tests::bench_from_pairs               ... bench:         470 ns/iter (+/- 27)
test initial::tests::bench_initial                     ... bench:          72 ns/iter (+/- 10)
test intersection::tests::bench_intersection           ... bench:         873 ns/iter (+/- 62)
test join::tests::bench_join                           ... bench:       1,855 ns/iter (+/- 233)
test last::tests::bench_last                           ... bench:          73 ns/iter (+/- 6)
test max::tests::bench_max                             ... bench:          74 ns/iter (+/- 5)
test mean::tests::bench_mean                           ... bench:         194 ns/iter (+/- 25)
test min::tests::bench_min                             ... bench:          74 ns/iter (+/- 6)
test multiply::tests::bench_multiply                   ... bench:           0 ns/iter (+/- 0)
test nth::tests::bench_nth                             ... bench:          72 ns/iter (+/- 7)
test pull::tests::bench_pull                           ... bench:         225 ns/iter (+/- 37)
test pull_at::tests::bench_pull_at                     ... bench:         324 ns/iter (+/- 44)
test remove::tests::bench_remove                       ... bench:       1,995 ns/iter (+/- 264)
test reverse::tests::bench_first                       ... bench:           4 ns/iter (+/- 0)
test round::tests::bench_round                         ... bench:           0 ns/iter (+/- 0)
test slice::tests::bench_slice                         ... bench:         202 ns/iter (+/- 31)
test sorted_index::tests::bench_sorted_index           ... bench:           0 ns/iter (+/- 0)
test sorted_last_index::tests::bench_sorted_last_index ... bench:           0 ns/iter (+/- 0)
test sorted_uniq::tests::bench_sorted_uniq             ... bench:         197 ns/iter (+/- 27)
test subtract::tests::bench_subtract                   ... bench:           0 ns/iter (+/- 0)
test sum::tests::bench_sum                             ... bench:         213 ns/iter (+/- 35)
test tail::tests::bench_tail                           ... bench:         196 ns/iter (+/- 18)
test take::tests::bench_take                           ... bench:         183 ns/iter (+/- 28)
test take_right::tests::bench_take_right               ... bench:         181 ns/iter (+/- 7)
test union::tests::bench_union                         ... bench:       1,083 ns/iter (+/- 129)
test unzip::tests::bench_unzip                         ... bench:       1,167 ns/iter (+/- 176)
test without::tests::bench_without                     ... bench:         341 ns/iter (+/- 20)
test xor::tests::bench_xor                             ... bench:       1,453 ns/iter (+/- 57)
test zip::tests::bench_zip                             ... bench:       1,136 ns/iter (+/- 90)
```