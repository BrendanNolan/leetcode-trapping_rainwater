mod histogram {
    pub trait Histogram {
        fn width(&self) -> usize;
        fn height_at(&self, horizontal_position: usize) -> i32;
    }
}

mod rainwater {
    use crate::histogram::Histogram;
    use std::cmp;

    fn calculate_trapped_rainwater<H: Histogram>(histogram: &H) -> Vec<i32> {
        if histogram.width() == 0 {
            return Vec::new();
        }
        let mut left_position = 0;
        let mut highest_bar_left = 0;
        let mut right_position = histogram.width() - 1;
        let mut highest_bar_right = 0;
        let mut water_heights = vec![0; histogram.width()];
        while left_position <= right_position {
            if highest_bar_left <= highest_bar_right {
                water_heights[left_position] =
                    cmp::max(0, highest_bar_left - histogram.height_at(left_position));
                highest_bar_left = cmp::max(highest_bar_left, histogram.height_at(left_position));
                left_position += 1;
            } else {
                water_heights[right_position] =
                    cmp::max(0, highest_bar_right - histogram.height_at(right_position));
                highest_bar_right =
                    cmp::max(highest_bar_right, histogram.height_at(right_position));
                right_position -= 1;
            }
        }
        water_heights
    }

    pub fn calculate_total_trapped_rainwater<H: Histogram>(histogram: &H) -> i32 {
        calculate_trapped_rainwater(histogram).iter().copied().sum()
    }
}

mod terrain {
    use crate::histogram::Histogram;

    pub struct Terrain {
        peaks: Vec<i32>,
    }

    impl Terrain {
        pub fn new(peaks: Vec<i32>) -> Self {
            Terrain { peaks }
        }
    }

    impl Histogram for Terrain {
        fn width(&self) -> usize {
            self.peaks.len()
        }

        fn height_at(&self, horizontal_position: usize) -> i32 {
            self.peaks[horizontal_position]
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::rainwater;
    use crate::terrain::Terrain;

    #[test]
    fn test_0() {
        let terrain = Terrain::new(vec![4, 2, 0, 3, 2, 5]);
        assert_eq!(rainwater::calculate_total_trapped_rainwater(&terrain), 9);
    }

    #[test]
    fn test_1() {
        let terrain = Terrain::new(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]);
        assert_eq!(rainwater::calculate_total_trapped_rainwater(&terrain), 6);
    }

    #[test]
    fn test_2() {
        let terrain = Terrain::new(vec![1, 1, 2, 3, 2, 1, 2]);
        assert_eq!(rainwater::calculate_total_trapped_rainwater(&terrain), 1);
    }

    #[test]
    fn test_3() {
        let terrain = Terrain::new(vec![1, 1, 2, 3, 2, 2, 2]);
        assert_eq!(rainwater::calculate_total_trapped_rainwater(&terrain), 0);
    }

    #[test]
    fn test_4() {
        let terrain = Terrain::new(vec![3, 2, 2, 2, 1, 0, 1, 2, 3]);
        assert_eq!(rainwater::calculate_total_trapped_rainwater(&terrain), 11);
    }

    #[test]
    fn test_5() {
        let terrain = Terrain::new(vec![4, 2, 3]);
        assert_eq!(rainwater::calculate_total_trapped_rainwater(&terrain), 1);
    }

    #[test]
    fn test_6() {
        let terrain = Terrain::new(vec![5, 5, 1, 7, 1, 1, 5, 2, 7, 6]);
        assert_eq!(rainwater::calculate_total_trapped_rainwater(&terrain), 23);
    }
}

use terrain::Terrain;
fn main() {
    let terrain = Terrain::new(Vec::new());
    assert_eq!(rainwater::calculate_total_trapped_rainwater(&terrain), 0);
    println!("Hello, world!");
}
