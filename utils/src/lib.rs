use std::ops::{Index, IndexMut};
use std::str::FromStr;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }

    pub fn neighbors(&self) -> Vec<Point> {
        vec![
            Point::new(self.x - 1, self.y),
            Point::new(self.x + 1, self.y),
            Point::new(self.x, self.y - 1),
            Point::new(self.x, self.y + 1),
        ]
    }

    pub fn neighbors_with_diagonals(&self) -> Vec<Point> {
        vec![
            Point::new(self.x - 1, self.y),
            Point::new(self.x + 1, self.y),
            Point::new(self.x, self.y - 1),
            Point::new(self.x, self.y + 1),
            Point::new(self.x - 1, self.y - 1),
            Point::new(self.x + 1, self.y - 1),
            Point::new(self.x - 1, self.y + 1),
            Point::new(self.x + 1, self.y + 1),
        ]
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Grid<T> {
    pub width: i32,
    pub height: i32,
    pub bytes: Vec<T>,
}

impl Grid<u8> {
    pub fn parse(input: &str) -> Self {
        let raw: Vec<_> = input.lines().map(str::as_bytes).collect();
        let width = raw[0].len() as i32;
        let height = raw.len() as i32;
        let mut bytes = Vec::with_capacity((width * height) as usize);
        raw.iter().for_each(|slice| bytes.extend_from_slice(slice));
        Grid {
            width,
            height,
            bytes,
        }
    }
}

impl<T> Grid<T> {
    pub fn contains(&self, point: Point) -> bool {
        point.x >= 0 && point.x < self.width && point.y >= 0 && point.y < self.height
    }
}

impl<T: Copy + PartialEq> Grid<T> {
    #[inline]
    pub fn find(&self, needle: T) -> Option<Point> {
        let to_point = |index| {
            let x = (index as i32) % self.width;
            let y = (index as i32) / self.width;
            Point::new(x, y)
        };
        self.bytes.iter().position(|&h| h == needle).map(to_point)
    }

    #[inline]
    pub fn find_all(&self, needle: T) -> Vec<Point> {
        let to_point = |index| {
            let x = (index as i32) % self.width;
            let y = (index as i32) / self.width;
            Point::new(x, y)
        };

        self.bytes
            .iter()
            .enumerate() // Get (index, value) pairs
            .filter_map(|(index, &h)| if h == needle { Some(index) } else { None }) // Keep indices where the value matches
            .map(to_point) // Convert indices to Points
            .collect() // Collect all Points into a Vec
    }
}

impl<T> Index<Point> for Grid<T> {
    type Output = T;

    fn index(&self, index: Point) -> &Self::Output {
        &self.bytes[(self.width * index.y + index.x) as usize]
    }
}

impl<T> IndexMut<Point> for Grid<T> {
    fn index_mut(&mut self, index: Point) -> &mut Self::Output {
        &mut self.bytes[(self.width * index.y + index.x) as usize]
    }
}

/// Splits input into lines as a vector of strings.
pub fn split_lines(input: &str) -> Vec<String> {
    input.lines().map(String::from).collect()
}

/// Parses lines into a vector of T where T implements `FromStr`.
pub fn parse_lines<T: FromStr>(input: &str) -> Vec<T> {
    input.lines().filter_map(|line| line.parse().ok()).collect()
}

/// Splits a single line of comma-separated values into a vector of T.
pub fn parse_csv<T: FromStr>(input: &str) -> Vec<T> {
    input
        .split(',')
        .filter_map(|value| value.trim().parse().ok())
        .collect()
}
