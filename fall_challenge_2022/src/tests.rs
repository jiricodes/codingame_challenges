use crate::*;

#[test]
#[ignore]
fn basics() {}

#[test]
fn get_neighbours_of_index() {
    let grid = Grid::new(3, 4);
    let i = 0;
    let expected: [Option<usize>; 4] = [None, Some(1), None, Some(3)];
    let res = grid.get_neighbours(i);
    assert_eq!(res, expected, "failed at index={}", i);
    let i = 1;
    let expected: [Option<usize>; 4] = [Some(0), Some(2), None, Some(4)];
    let res = grid.get_neighbours(i);
    assert_eq!(res, expected, "failed at index={}", i);
    let i = 2;
    let expected: [Option<usize>; 4] = [Some(1), None, None, Some(5)];
    let res = grid.get_neighbours(i);
    assert_eq!(res, expected, "failed at index={}", i);
    let i = 3;
    let expected: [Option<usize>; 4] = [None, Some(4), Some(0), Some(6)];
    let res = grid.get_neighbours(i);
    assert_eq!(res, expected, "failed at index={}", i);
    let i = 4;
    let expected: [Option<usize>; 4] = [Some(3), Some(5), Some(1), Some(7)];
    let res = grid.get_neighbours(i);
    assert_eq!(res, expected, "failed at index={}", i);
    let i = 5;
    let expected: [Option<usize>; 4] = [Some(4), None, Some(2), Some(8)];
    let res = grid.get_neighbours(i);
    assert_eq!(res, expected, "failed at index={}", i);
    let i = 9;
    let expected: [Option<usize>; 4] = [None, Some(10), Some(6), None];
    let res = grid.get_neighbours(i);
    assert_eq!(res, expected, "failed at index={}", i);
    let i = 10;
    let expected: [Option<usize>; 4] = [Some(9), Some(11), Some(7), None];
    let res = grid.get_neighbours(i);
    assert_eq!(res, expected, "failed at index={}", i);
    let i = 11;
    let expected: [Option<usize>; 4] = [Some(10), None, Some(8), None];
    let res = grid.get_neighbours(i);
    assert_eq!(res, expected, "failed at index={}", i);
}

#[test]
fn dist_sq() {
    let f = (1, 2);
    let t = (2, 3);
    assert_eq!(dist_squared(f, t), 2);
}
