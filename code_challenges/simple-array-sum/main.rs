
fn challenge(ar : &Vec<i32>) -> i32 {
    // given an array find his sum
    return ar.iter().sum()
}

fn main() {
    // base 
    let base_test_case = vec![1, 2, 3];
    assert!(challenge(&base_test_case) == 6, "actual = {}, expected = {}", result, 6);
}