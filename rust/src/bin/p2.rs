use std::{str::FromStr};

use anyhow::{anyhow, Result};

fn get_input() -> &'static str {
    return "0,9 -> 5,9
8,0 -> 0,8
5,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Line {
    p1: Point,
    p2: Point,
}

impl FromStr for Point {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self> {
        let result = s.split_once(",");
        if result.is_none() {
            return Err(anyhow!("Expect a point that contain a comma"));
        }

        let (x, y) = result.unwrap();
        let x: i32 = str::parse(x)?;
        let y: i32 = str::parse(y)?;

        return Ok(Point { x, y });
    }
}

impl FromStr for Line {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self> {
        let result = s.split_once(" -> ");
        if result.is_none() {
            return Err(anyhow!("Expect a point that contain a -> "));
        }

        let (x, y) = result.unwrap();
        let p1 = str::parse(x)?;
        let p2 = str::parse(y)?;

        return Ok(Line { p1, p2 });
    }
}

impl Line {
    fn is_horv(&self) -> bool {
        return self.p1.x == self.p2.x || self.p1.y == self.p2.y;
    }
}

fn main() {
    let lines = get_input()
        .lines()
        .flat_map(|x| str::parse(x))
        .filter(|x: &Line| x.is_horv())
        .collect::<Vec<Line>>();
    println!("{:?}", lines)
}
