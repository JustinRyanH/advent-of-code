use std::cmp::Ordering;

pub fn find_sum_2020(values: &[u32]) -> Option<(u32, u32)> {
    let mut values = values.to_vec();
    values.sort_by(|a, b| b.partial_cmp(a).unwrap_or(Ordering::Equal));
    for value in values.clone() {
        if let Some(high) = values.iter().rev().find(|v| *v + value == 2020) {
            return Some((value, *high))
        }
    }
    None

}

pub fn part1(values: &[u32]) -> Option<u32> {
    find_sum_2020(values).map(|(low, high)| low * high)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_two_values_equal_2020() {
        let values = [
            1721,
            979,
            366,
            299,
            675,
            1456,
        ];
        // assert_eq!(find_sum_2020(&values), Some((1721, 299)));
        assert_eq!(part1(&values), Some(514579));
    }
}
