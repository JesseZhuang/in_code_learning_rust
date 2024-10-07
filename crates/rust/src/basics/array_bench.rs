// #![feature(test)]
//
// extern crate test;
//
// fn main() {}
//
// /// modify run config in IDE or use `rustup default nightly`.
// ///
// /// https://stackoverflow.com/questions/53691012/is-there-any-way-to-allocate-a-standard-rust-array-directly-on-the-heap-skippin
// // running 3 tests
// // test tests::array_heap  ... bench:   9,613,370 ns/iter (+/- 4,174,693)
// // test tests::array_stack ... bench:   8,796,671 ns/iter (+/- 1,006,284)
// // test tests::vec_heap    ... bench:   8,183,537 ns/iter (+/- 696,343)
// // running 3 tests, again
// // test tests::array_heap  ... bench:   8,256,834 ns/iter (+/- 813,194)
// // test tests::array_stack ... bench:   8,735,491 ns/iter (+/- 814,914)
// // test tests::vec_heap    ... bench:   8,121,683 ns/iter (+/- 450,031)
// #[cfg(test)]
// mod tests {
//     use test::Bencher;
//
//     #[bench]
//     fn array_stack(b: &mut Bencher) {
//         const LENGTH: usize = 10_000;
//
//         b.iter(|| {
//             let mut a: [i32; LENGTH] = [0; LENGTH];
//             for j in 0..LENGTH {
//                 for i in 0..LENGTH {
//                     a[i] = j as i32;
//                 }
//             }
//
//             a
//         });
//     }
//
//     #[bench]
//     fn array_heap(b: &mut Bencher) {
//         const LENGTH: usize = 10_000;
//
//         b.iter(|| {
//             let mut a: Box<[i32; LENGTH]> = Box::new([0; LENGTH]);
//             for j in 0..LENGTH {
//                 for i in 0..LENGTH {
//                     a[i] = j as i32;
//                 }
//             }
//
//             a
//         });
//     }
//
//
//     #[bench]
//     fn vec_heap(b: &mut Bencher) {
//         const LENGTH: usize = 10_000;
//
//         b.iter(|| {
//             let mut a: Vec<i32> = vec![0; LENGTH];
//             for j in 0..LENGTH {
//                 for i in 0..LENGTH {
//                     a[i] = j as i32;
//                 }
//             }
//
//             a
//         });
//     }
// }
