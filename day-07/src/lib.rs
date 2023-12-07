mod part_1;
mod part_2;

pub use part_1::process_part_1;
pub use part_2::process_part_2;

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    #[test]
    fn part_1_works() {
        let result = process_part_1(INPUT);
        assert_eq!(result, "6440");
    }

    #[test]
    fn part_2_works() {
        let result = process_part_2(INPUT);
        assert_eq!(result, "5905");
    }
}
