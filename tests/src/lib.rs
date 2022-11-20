#[cfg(test)]
use std::{fmt::Debug, str::FromStr};

#[cfg(test)]
fn convert_and_compare<T>(a: impl ToString, b: T)
where
    T: FromStr + PartialEq + Debug,
    <T as FromStr>::Err: std::fmt::Debug,
{
    let a = a.to_string();
    let a = a.parse::<T>().unwrap();
    assert_eq!(a, b)
}

#[cfg(test)]
mod aoc_2015_1 {
    use super::convert_and_compare;
    use aoc_2015_1::*;

    #[test]
    fn p1_test() {
        convert_and_compare(p1("(((").unwrap(), 3);
        convert_and_compare(p1("((()").unwrap(), 2);
        convert_and_compare(p1("()()()").unwrap(), 0);
        convert_and_compare(p1("()()())").unwrap(), -1);
    }

    #[test]
    fn p2_test() {
        convert_and_compare(p2(")").unwrap(), 1);
        convert_and_compare(p2("()())").unwrap(), 5);
    }
}

#[cfg(test)]
mod aoc_2015_2 {
    use super::convert_and_compare;
    use aoc_2015_2::*;

    #[test]
    fn p1_test() {
        convert_and_compare(p1("2x3x4").unwrap(), 58);
        convert_and_compare(p1("1x1x10").unwrap(), 43);
    }

    #[test]
    fn p2_test() {
        convert_and_compare(p2("2x3x4").unwrap(), 34);
        convert_and_compare(p2("1x1x10").unwrap(), 14);
    }
}

#[cfg(test)]
mod aoc_2015_3 {
    use super::convert_and_compare;
    use aoc_2015_3::*;

    #[test]
    fn p1_test() {
        convert_and_compare(p1(">").unwrap(), 2);
        convert_and_compare(p1(">>").unwrap(), 3);
        convert_and_compare(p1(">><").unwrap(), 3);
        convert_and_compare(p1("").unwrap(), 1);
        convert_and_compare(p1("^").unwrap(), 2);
        convert_and_compare(p1("^>v<").unwrap(), 4);
        convert_and_compare(p1("^v^v^v^v^v^").unwrap(), 2);
    }

    #[test]
    fn p2_test() {
        convert_and_compare(p2("^v").unwrap(), 3);
        convert_and_compare(p2("^>v<").unwrap(), 3);
        convert_and_compare(p2("^v^v^v^v^v").unwrap(), 11);
    }
}

#[cfg(test)]
mod aoc_2015_4 {
    use super::convert_and_compare;
    use aoc_2015_4::*;

    #[test]
    #[ignore = "slow"]
    fn p1_test() {
        let key = "abcdef";
        convert_and_compare(p1(key).unwrap(), 609043);

        let key = "pqrstuv";
        convert_and_compare(p1(key).unwrap(), 1048970);
    }
}
