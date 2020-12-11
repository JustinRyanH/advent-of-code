#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_two_values_equal_2020() {
        // pub fn find_sum_2020(values: &[u16]) -> (u16, 16)
        let values = [
            1721,
            979,
            366,
            299,
            675,
            1456,
        ];
        assert_eq!(find_sum_2020(&values), (1721, 299))
    }
}
