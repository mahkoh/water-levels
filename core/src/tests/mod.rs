mod gen;

use crate::Terrain;

const EPSILON: f64 = 0.00001;

#[test]
fn empty() {
    assert!(Terrain::new(&[]).is_err());
}

#[test]
fn invalid_level() {
    assert!(Terrain::new(&[-1.0]).is_err());
}

#[test]
fn invalid_hours() {
    assert!(Terrain::new(&[1.0]).unwrap().rain(-1.0).is_err());
}

fn test_(before: &[f64], hours: f64, expected: &[f64]) {
    assert_eq!(before.len(), expected.len());
    let after = Terrain::new(before).unwrap().rain(hours).unwrap();
    assert_eq!(after.len(), expected.len());
    for (pos, (e, a)) in expected
        .iter()
        .copied()
        .zip(after.iter().copied())
        .enumerate()
    {
        assert!(
            (e - a).abs() <= EPSILON,
            "difference at position {},\nexpected {:?},\nactual   {:?}",
            pos,
            expected,
            after,
        );
    }
    let expected_volume = hours * before.len() as f64;
    let mut actual_volume = 0.0;
    for (pos, (b, a)) in before
        .iter()
        .copied()
        .zip(after.iter().copied())
        .enumerate()
    {
        assert!(
            a >= b,
            "after value at position {} is smaller than before value, before {}, after {}",
            pos,
            b,
            a,
        );
        actual_volume += a - b;
    }
    assert!(
        (expected_volume - actual_volume).abs() <= EPSILON,
        "expected water volume differs from actual volume, expected {}, actual {}",
        expected_volume,
        actual_volume,
    );
}

fn test(before: &[f64], hours: f64, expected: &[f64]) {
    test_(before, hours, expected);
    let mut before = before.to_vec();
    let mut expected = expected.to_vec();
    before.reverse();
    expected.reverse();
    test_(&before, hours, &expected);
}

#[test]
fn test1() {
    test(&[0.0], 0.0, &[0.0]);
}

#[test]
fn test2() {
    test(&[0.0], 1.0, &[1.0]);
}

#[test]
fn test3() {
    test(&[1.0], 1.0, &[2.0]);
}

#[test]
fn test4() {
    test(&[1.0], 1.0, &[2.0]);
}

#[test]
fn test5() {
    test(&[2.0, 0.0], 1.0, &[2.0, 2.0]);
}

#[test]
fn test6() {
    test(&[2.0, 0.0], 2.0, &[3.0, 3.0]);
}

#[test]
fn test7() {
    test(&[2.0, 0.0], 0.5, &[2.0, 1.0]);
}

#[test]
fn test8() {
    test(
        &[10.0, 0.0, 2.0, 1.0, 10.0, 1.0],
        0.5,
        &[10.0, 1.25, 2.0, 2.0, 10.0, 1.75],
    );
}

#[test]
fn test9() {
    test(
        &[10.0, 0.0, 2.0, 1.0, 10.0, 1.0],
        0.25,
        &[10.0, 0.625, 2.0, 1.5, 10.0, 1.375],
    );
}

#[test]
fn test10() {
    test(
        &[10.0, 0.0, 2.0, 1.0, 10.0, 1.0],
        1.0,
        &[10.0, 2.5, 2.5, 2.5, 10.0, 2.5],
    );
}

#[test]
fn test11() {
    test(
        &[10.0, 0.0, 2.0, 1.0, 2.0, 0.0],
        1.0,
        &[10.0, 2.2, 2.2, 2.2, 2.2, 2.2],
    );
}

#[test]
fn test12() {
    test(
        &[10.0, 0.0, 2.0, 1.0, 2.0, 0.0],
        1.0,
        &[10.0, 2.2, 2.2, 2.2, 2.2, 2.2],
    );
}

#[test]
fn test13() {
    test(
        &[0.0, 10.0, 10.0, 4.0, 11.0, 10.0],
        1.0,
        &[2.0, 10.0, 10.0, 7.0, 11.0, 11.0],
    );
}

#[test]
fn test14() {
    test(&[0.0, 1.0, 2.0], 1.0, &[2.0, 2.0, 2.0]);
}

#[test]
fn test15() {
    test(&[0.0, 1.0, 3.0], 1.0, &[2.0, 2.0, 3.0]);
}

#[test]
fn test16() {
    test(&[1.0, 0.0, 3.0], 1.0, &[2.0, 2.0, 3.0]);
}

#[test]
fn test17() {
    test(
        &[1.0, 0.0, 1.0, 2.0],
        0.5,
        &[1.333333, 1.3333333, 1.3333333, 2.0],
    );
}
