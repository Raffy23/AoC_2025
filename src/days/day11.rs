use std::{
    collections::{HashMap, VecDeque},
    ops::{Index, IndexMut},
};
use winnow::{
    ascii::newline,
    combinator::{iterator, separated, seq, terminated},
    error::{EmptyError, ErrMode},
    token::take,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Node(u16);

type Graph = Vec<Vec<Node>>;

pub fn solve1(input: &str) -> usize {
    let (nodes, graph) = parse_input(input);

    let start = nodes["you"];
    let end = nodes["out"];

    let mut paths = 0;

    let mut queue = VecDeque::with_capacity(500);
    queue.push_back(start);

    while let Some(node) = queue.pop_back() {
        if node == end {
            paths += 1;
        } else {
            for &neighbor in &graph[node] {
                queue.push_back(neighbor);
            }
        }
    }

    paths
}

pub fn solve2(input: &str) -> usize {
    let (nodes, graph) = parse_input(input);

    fn find_paths(node: Node, end: Node, graph: &Graph, cache: &mut Vec<Option<usize>>) -> usize {
        if node == end {
            cache[node] = Some(1);
            return 1;
        }

        if let Some(cached_counts) = cache[node] {
            return cached_counts;
        }

        let mut paths = 0;
        for &neighbor in &graph[node] {
            paths += find_paths(neighbor, end, &graph, cache);
        }

        cache[node] = Some(paths);

        paths
    }

    let start = nodes["svr"];
    let end = nodes["out"];

    let dac = nodes["dac"];
    let fft = nodes["fft"];

    [[start, dac, fft, end], [start, fft, dac, end]]
        .map(|sub_path| {
            let mut length = 1;

            for pair in sub_path.windows(2) {
                length *= find_paths(pair[0], pair[1], &graph, &mut vec![None; nodes.len()]);
                if length == 0 {
                    return 0;
                }
            }

            length
        })
        .into_iter()
        .sum()
}

fn parse_input<'s>(input: &'s str) -> (HashMap<&'s str, Node>, Graph) {
    let mut input = iterator::<_, (&str, Vec<&str>), ErrMode<EmptyError>, _>(
        input,
        terminated(
            seq!(
                take(3u32),
                _: ':',
                _: ' ',
                separated(1.., take(3u32), ' '),
            ),
            newline,
        ),
    );

    let mut node_labels = HashMap::new();

    fn get_node_id<'s>(
        label_name: &'s str,
        node_labels: &mut HashMap<&'s str, Node>,
        label_count: &mut u16,
    ) -> Node {
        match node_labels.get(label_name) {
            Some(&label) => label,
            None => {
                let new_label = Node(*label_count);
                node_labels.insert(label_name, new_label);
                *label_count += 1;

                new_label
            }
        }
    }

    let mut graph = Vec::new();
    let mut label_count = 0;
    input.for_each(|(node_label, neighbors)| {
        let node = get_node_id(node_label, &mut node_labels, &mut label_count);

        while graph.len() <= node.into() {
            graph.push(Vec::new());
        }

        for neighbor in neighbors {
            let neighbor = get_node_id(neighbor, &mut node_labels, &mut label_count);
            graph[node].push(neighbor);
        }
    });

    // need to fill up the matrix so all rows are populated
    while graph.len() <= label_count as usize {
        graph.push(Vec::new());
    }

    (node_labels, graph)
}

impl<T> Index<Node> for Vec<T> {
    type Output = T;

    fn index(&self, index: Node) -> &Self::Output {
        &self[index.0 as usize]
    }
}

impl<T> IndexMut<Node> for Vec<T> {
    fn index_mut(&mut self, index: Node) -> &mut Self::Output {
        &mut self[index.0 as usize]
    }
}

impl Into<usize> for Node {
    fn into(self) -> usize {
        self.0 as usize
    }
}

#[allow(const_item_mutation)]
#[cfg(test)]
mod tests {
    use crate::day11::{solve1, solve2};

    const EXAMPLE_INPUT_1: &'static str = r#"aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out
"#;

    #[test]
    fn part1() {
        assert_eq!(solve1(EXAMPLE_INPUT_1), 5)
    }

    const EXAMPLE_INPUT_2: &'static str = r#"svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out
"#;

    #[test]
    fn part2() {
        assert_eq!(solve2(EXAMPLE_INPUT_2), 2)
    }
}
