use winnow::ascii::{dec_uint, newline};
use winnow::combinator::{iterator, seq, terminated};
use winnow::error::{EmptyError, ErrMode};
use winnow::token::take;

type Point2D = (u32, u32);
type Edge = (Point2D, Point2D);

#[derive(Debug, Clone, Copy)]
struct Rect {
    x1: u32,
    y1: u32,
    x2: u32,
    y2: u32,
}

pub fn solve1(input: &str) -> u64 {
    let points = parse_input(input);

    let mut max = 0;
    for i in 0..points.len() {
        for j in i + 1..points.len() {
            let a = area(&points[i], &points[j]);
            if max < a {
                max = a;
            }
        }
    }

    max
}

pub fn solve2(input: &str) -> u64 {
    let points = parse_input(input);

    let mut max = 0;
    for i in 0..points.len() {
        for j in i + 1..points.len() {
            let a = area(&points[i], &points[j]);
            if max < a {
                if Rect::from(&points[i], &points[j]).is_inside_polygon(&points) {
                    max = a;
                }
            }
        }
    }

    max
}

fn area(p1: &Point2D, p2: &Point2D) -> u64 {
    (p1.0.abs_diff(p2.0) + 1) as u64 * (p1.1.abs_diff(p2.1) + 1) as u64
}

impl Rect {
    fn from(p1: &Point2D, p2: &Point2D) -> Self {
        Self {
            x1: p1.0.min(p2.0),
            x2: p1.0.max(p2.0),
            y1: p1.1.min(p2.1),
            y2: p1.1.max(p2.1),
        }
    }

    fn is_inside_polygon(&self, polygon: &[Point2D]) -> bool {
        let len = polygon.len();

        let mut j = len - 1;
        for i in 0..len {
            let e1 = polygon[i];
            let e2 = polygon[j];

            // Note: This might not be fully correct, but since polygons are axis aligned
            //       it should be sufficient to just check for any edges that reach into
            //       the interior of the rectangle
            if self.intersect_edge((e1, e2)) {
                return false;
            }

            j = i;
        }

        true
    }

    fn intersect_edge(&self, (u, v): Edge) -> bool {
        #[inline]
        fn is_between(value: u32, start: u32, end: u32) -> bool {
            value > start && value < end
        }

        #[inline]
        fn overlaps(edge_min: u32, edge_max: u32, start: u32, end: u32) -> bool {
            let start = start.max(edge_min);
            let end = end.min(edge_max);
            start < end
        }

        if u.0 == v.0 {
            let edge_y_min = u.1.min(v.1);
            let edge_y_max = u.1.max(v.1);

            if is_between(u.0, self.x1, self.x2)
                && overlaps(edge_y_min, edge_y_max, self.y1, self.y2)
            {
                return true;
            }
        } else {
            let edge_x_min = u.0.min(v.0);
            let edge_x_max = u.0.max(v.0);

            if is_between(u.1, self.y1, self.y2)
                && overlaps(edge_x_min, edge_x_max, self.x1, self.x2)
            {
                return true;
            }
        }

        false
    }
}

fn parse_input(input: &str) -> Vec<Point2D> {
    iterator::<_, _, ErrMode<EmptyError>, _>(
        input,
        terminated(
            seq!(
                dec_uint,
                _: take(1u16),
                dec_uint,
            ),
            newline,
        ),
    )
    .fold(Vec::with_capacity(500), |mut vec, point| {
        vec.push(point);
        vec
    })
}

#[allow(const_item_mutation)]
#[cfg(test)]
mod tests {
    use crate::day09::{area, solve1, solve2};

    const EXAMPLE_INPUT: &'static str = r#"7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
"#;

    #[test]
    fn area_example() {
        assert_eq!(area(&(9, 7), &(2, 5)), 24);
        assert_eq!(area(&(2, 5), &(9, 7)), 24);
    }

    #[test]
    fn part1() {
        assert_eq!(solve1(EXAMPLE_INPUT), 50)
    }

    #[test]
    fn part2() {
        assert_eq!(solve2(EXAMPLE_INPUT), 24)
    }
}
