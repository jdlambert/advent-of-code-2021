use std::fmt;

#[derive(Clone, Debug)]
enum FishNum {
    Value(u32),
    Pair(Box<FishNum>, Box<FishNum>),
}

struct FishParser {
    i: usize,
    s: String,
}

impl FishParser {
    fn new(s: &str) -> Self {
        Self {
            i: 0,
            s: s.to_owned(),
        }
    }

    fn parse(&mut self) -> FishNum {
        if self.s.chars().nth(self.i).unwrap() == '[' {
            self.i += 1;
            let left = self.parse();
            self.i += 1;
            let right = self.parse();
            self.i += 1; // ']'
            FishNum::Pair(Box::new(left), Box::new(right))
        } else {
            let digits = self
                .s
                .chars()
                .skip(self.i)
                .take_while(|c| c.is_digit(10))
                .collect::<String>();
            self.i += digits.len();
            FishNum::Value(digits.parse().unwrap())
        }
    }
}

impl FishNum {
    fn from_str(s: &str) -> Self {
        let mut parser = FishParser::new(s);
        parser.parse()
    }

    fn magnitude(&self) -> u32 {
        match self {
            FishNum::Value(v) => *v,
            FishNum::Pair(l, r) => 3 * l.magnitude() + 2 * r.magnitude(),
        }
    }

    fn add_left(&self, value: u32) -> Self {
        match self {
            FishNum::Value(v) => FishNum::Value(v + value),
            FishNum::Pair(a, b) => FishNum::Pair(Box::new(a.add_left(value)), b.clone()),
        }
    }

    fn add_right(&self, value: u32) -> Self {
        match self {
            FishNum::Value(v) => FishNum::Value(v + value),
            FishNum::Pair(a, b) => FishNum::Pair(a.clone(), Box::new(b.add_right(value))),
        }
    }

    fn explode_helper(&self, depth: usize) -> Option<(Option<u32>, Self, Option<u32>)> {
        if let FishNum::Pair(a, b) = self {
            if depth > 3 {
                let (a, b) = (a.clone(), b.clone());
                if let (FishNum::Value(a), FishNum::Value(b)) = (*a, *b) {
                    Some((Some(a), FishNum::Value(0), Some(b)))
                } else {
                    None
                }
            } else if let Some((l, new_a, r)) = a.explode_helper(depth + 1) {
                if let Some(v) = r {
                    Some((
                        l,
                        FishNum::Pair(Box::new(new_a), Box::new(b.add_left(v))),
                        None,
                    ))
                } else {
                    Some((l, FishNum::Pair(Box::new(new_a), b.clone()), r))
                }
            } else if let Some((l, new_b, r)) = b.explode_helper(depth + 1) {
                if let Some(v) = l {
                    Some((
                        None,
                        FishNum::Pair(Box::new(a.add_right(v)), Box::new(new_b)),
                        r,
                    ))
                } else {
                    Some((l, FishNum::Pair(a.clone(), Box::new(new_b)), r))
                }
            } else {
                None
            }
        } else {
            None
        }
    }

    fn explode(&self) -> Option<Self> {
        if let Some((_, n, _)) = self.explode_helper(0) {
            Some(n)
        } else {
            None
        }
    }

    fn split(&self) -> Option<Self> {
        match self {
            FishNum::Value(a) if *a > 9 => Some(FishNum::Pair(
                Box::new(FishNum::Value(a / 2)),
                Box::new(FishNum::Value(a - a / 2)),
            )),
            FishNum::Value(_) => None,
            FishNum::Pair(a, b) => {
                if let Some(a) = a.split() {
                    Some(FishNum::Pair(Box::new(a), b.clone()))
                } else if let Some(b) = b.split() {
                    Some(FishNum::Pair(a.clone(), Box::new(b)))
                } else {
                    None
                }
            }
        }
    }

    fn reduce(mut self) -> Self {
        loop {
            if let Some(f) = self.explode() {
                self = f;
            } else if let Some(f) = self.split() {
                self = f;
            } else {
                return self;
            }
        }
    }

    fn add(&self, that: &Self) -> Self {
        Self::Pair(Box::new(self.clone()), Box::new(that.clone())).reduce()
    }

    fn sum(nums: &[Self]) -> Self {
        let mut sum = nums[0].clone();
        nums.iter().for_each(|num| sum = sum.add(num));
        sum
    }
}

impl fmt::Display for FishNum {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FishNum::Value(a) => write!(f, "{}", a),
            FishNum::Pair(a, b) => {
                write!(f, "[")?;
                write!(f, "{}", a)?;
                write!(f, ",")?;
                write!(f, "{}", b)?;
                write!(f, "]")
            }
        }
    }
}

fn part1(nums: &[FishNum]) -> u32 {
    FishNum::sum(nums).magnitude()
}

fn part2(nums: &[FishNum]) -> u32 {
    let mut max = u32::MIN;
    for i in 0..nums.len() {
        for j in 0..nums.len() {
            if i != j {
                max = max.max(nums[i].add(&nums[j]).magnitude());
            }
        }
    }
    max
}

fn main() {
    let data: Vec<_> = include_str!("../input.txt")
        .lines()
        .map(FishNum::from_str)
        .collect();
    println!("Part 1: {}", part1(&data));
    println!("Part 2: {}", part2(&data));
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_magnitudes() {
        for (s, m) in [
            ("[[1,2],[[3,4],5]]", 143),
            (
                "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]",
                3488,
            ),
        ] {
            assert_eq!(FishNum::from_str(s).magnitude(), m);
        }
    }

    #[test]
    fn test_sum() {
        for (nums, sum) in [(
            vec![
                "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]",
                "[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]",
                "[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]",
                "[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]",
                "[7,[5,[[3,8],[1,4]]]]",
                "[[2,[2,2]],[8,[8,1]]]",
                "[2,9]",
                "[1,[[[9,3],9],[[9,0],[0,7]]]]",
                "[[[5,[7,4]],7],1]",
                "[[[[4,2],2],6],[8,7]]",
            ],
            "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]",
        )] {
            let nums: Vec<_> = nums.iter().map(|s| FishNum::from_str(s)).collect();
            assert_eq!(format!("{}", FishNum::sum(&nums)), sum);
        }
    }

    #[test]
    fn test_add() {
        for (a, b, s) in [
            ("[1,2]", "[[3,4],5]", "[[1,2],[[3,4],5]]"),
            (
                "[[[[4,3],4],4],[7,[[8,4],9]]]",
                "[1,1]",
                "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]",
            ),
            (
                "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]",
                "[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]",
                "[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]",
            ),
            (
                "[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]",
                "[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]",
                "[[[[6,7],[6,7]],[[7,7],[0,7]]],[[[8,7],[7,7]],[[8,8],[8,0]]]]",
            ),
            (
                "[[[[6,7],[6,7]],[[7,7],[0,7]]],[[[8,7],[7,7]],[[8,8],[8,0]]]]",
                "[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]",
                "[[[[7,0],[7,7]],[[7,7],[7,8]]],[[[7,7],[8,8]],[[7,7],[8,7]]]]",
            ),
            (
                "[[[[7,0],[7,7]],[[7,7],[7,8]]],[[[7,7],[8,8]],[[7,7],[8,7]]]]",
                "[7,[5,[[3,8],[1,4]]]]",
                "[[[[7,7],[7,8]],[[9,5],[8,7]]],[[[6,8],[0,8]],[[9,9],[9,0]]]]",
            ),
            (
                "[[[[7,7],[7,8]],[[9,5],[8,7]]],[[[6,8],[0,8]],[[9,9],[9,0]]]]",
                "[[2,[2,2]],[8,[8,1]]]",
                "[[[[6,6],[6,6]],[[6,0],[6,7]]],[[[7,7],[8,9]],[8,[8,1]]]]",
            ),
            (
                "[[[[6,6],[6,6]],[[6,0],[6,7]]],[[[7,7],[8,9]],[8,[8,1]]]]",
                "[2,9]",
                "[[[[6,6],[7,7]],[[0,7],[7,7]]],[[[5,5],[5,6]],9]]",
            ),
            (
                "[[[[6,6],[7,7]],[[0,7],[7,7]]],[[[5,5],[5,6]],9]]",
                "[1,[[[9,3],9],[[9,0],[0,7]]]]",
                "[[[[7,8],[6,7]],[[6,8],[0,8]]],[[[7,7],[5,0]],[[5,5],[5,6]]]]",
            ),
            (
                "[[[[7,8],[6,7]],[[6,8],[0,8]]],[[[7,7],[5,0]],[[5,5],[5,6]]]]",
                "[[[5,[7,4]],7],1]",
                "[[[[7,7],[7,7]],[[8,7],[8,7]]],[[[7,0],[7,7]],9]]",
            ),
            (
                "[[[[7,7],[7,7]],[[8,7],[8,7]]],[[[7,0],[7,7]],9]]",
                "[[[[4,2],2],6],[8,7]]",
                "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]",
            ),
        ] {
            let sum = FishNum::from_str(a).add(&FishNum::from_str(b));
            assert_eq!(format!("{}", sum), s);
        }
    }

    #[test]
    fn test_explode() {
        for (s, e) in [("[[[[[9,8],1],2],3],4]", "[[[[0,9],2],3],4]")] {
            let exploded = FishNum::from_str(s).explode().unwrap();
            assert_eq!(format!("{}", exploded), e);
        }
    }

    #[test]
    fn test_split() {
        for (s, e) in [(
            "[[[[0,7],4],[15,[0,13]]],[1,1]]",
            "[[[[0,7],4],[[7,8],[0,13]]],[1,1]]",
        )] {
            let split = FishNum::from_str(s).split().unwrap();
            assert_eq!(format!("{}", split), e);
        }
    }
}
