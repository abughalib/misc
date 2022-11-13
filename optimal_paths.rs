/* Question.

**Optimal paths**

You are given an undirected weighted graph Of N nodes and M edges. There are Q queries.
each query has two nodes U and V.

Task
For each query, you have to find the least integer K such that there is a path between and
V using only edges of weight at most K If U and Vare not reachable at all, print -1
Note: I-based indexing is followed.

Example
Assumptions
• N = 7
• M = 7
• Q = 6
• Edges  [(1 4 4), (1 2 3), (1 5 7), (1 3 6), (2 4 2), (2 5 12), (3 6 9)]
• Query: [(1, 4), (2, 6), (4, 5), (2, 5), (3, 4), (1, 7)]

Approach
For 1st query:
  • There is a path 1-24 which has all edges of weight 3
  • No other path results in a smaller value of 3

Therefore the answer is 3

For 6th query:
• Here. 1 and 7 are not reachable at all.

Therefore the answer is -1

Function description
Complete the function solve provided in the editor This function takes the following 4
parameters and returns the maximum required sum:

• N: Represents an integer denoting the number of nodes in the graph
• M: Represents an integer denoting the number of edges in the graph
• Q: Represents an integer denoting the size of the query array
• edges: Represents a 2D array denoting the edges in the given graph
• query: Represents a 2D array denoting the queries

Input format

Note: This is the input format you must use to provide custom input (available above the Compile and Test button).

• The first line contains three integers N, M, and Q.
• The next M lines contain three integers U, V, and W representing an undirected edge between U and V of weight W
• The next Q lines contain two integers (J and V for the respective queries. 

Output format

For each query in a new line, print the answer.

Constraints
1 <= N, Q <= 10e5
1 <= M <= min(2 * 10e3, N * (N - 1)/2)
1 <= w <= 10e9

Sample Input
9 7 7
4 6 5
7 1 6
9 2 6
5 3 8
2 5 6
2 3 8
4 9 4
5 1
2 4
2 9
2 7
6 9
6 4
6 7

Sample Output
-1
6
6
-1
5
5
-1
*/

use std::cmp::max;
use std::io::{self, prelude::*, BufRead, BufWriter};
use std::str::*;

struct Scanner<R> {
    reader: R,
    buf_str: Vec<u8>,
    buf_iter: SplitWhitespace<'static>,
}

impl<R: BufRead> Scanner<R> {
    fn new(reader: R) -> Self {
        Self {
            reader,
            buf_str: vec![],
            buf_iter: "".split_whitespace(),
        }
    }
    fn cin<T: FromStr>(&mut self) -> T {
        loop {
            if let Some(t) = self.buf_iter.next() {
                return t.parse().ok().expect("Failed to Parse!");
            }
            self.buf_str.clear();
            self.reader
                .read_until(b'\n', &mut self.buf_str)
                .ok()
                .expect("Failed to Read!");
            self.buf_iter = unsafe {
                let slice = from_utf8_unchecked(&self.buf_str);
                std::mem::transmute(slice.split_whitespace())
            }
        }
    }
}

#[derive(Clone)]
struct MinHeap {
    value: Vec<usize>,
    node: Vec<usize>,
}

impl MinHeap {
    fn new() -> Self {
        Self {
            value: vec![],
            node: vec![],
        }
    }

    fn swap(&mut self, i: usize, j: usize) {
        self.value[i] = self.value[j];
        self.node[i] = self.node[j];
    }
    fn bubble_up(&mut self, mut idx: usize) {
        while idx > 0 && self.value[(idx - 1) / 2] > self.value[idx] {
            self.swap(idx, (idx - 1) / 2);
            idx = (idx - 1) / 2;
        }
    }
    fn bubble_down(&mut self, mut idx: usize) {
        let n = self.value.len();
        while idx <= n - 1 {
            let (l, r) = (idx * 2 + 1, idx * 2 + 2);
            if l > n - 1 {
                break;
            }
            let child = if r <= n - 1 && self.value[r] < self.value[l] {
                r
            } else {
                l
            };
            if self.value[child] < self.value[idx] {
                self.swap(idx, child);
                idx = child;
            } else {
                break;
            }
        }
    }
    fn extract_min(self) -> Option<(usize, usize)> {
        if self.value.is_empty() {
            return None;
        }
        return Some((self.value[0], self.node[0]));
    }
    fn insert(&mut self, v: usize, node: usize) {
        let idx = self.value.len();
        self.value.push(v);
        self.node.push(node);
        self.bubble_up(idx);
    }
    fn delete(&mut self, pos: usize) {
        let idx = self.value.len() - 1;
        self.swap(pos, idx);
        self.value.pop();
        self.node.pop();
        if self.value.is_empty() || pos > self.value.len() - 1 {
            return;
        }
        // Bubble up, Bubble Down
        if pos > 0 && self.value[pos] < self.value[(pos - 1) / 2] {
            self.bubble_up(pos);
        } else {
            self.bubble_down(pos);
        }
    }
}

trait Indexing {
    fn find_index(&self, value: usize) -> Option<usize>;
}

impl Indexing for Vec<usize> {
    fn find_index(&self, value: usize) -> Option<usize> {
        for i in 0..self.len() {
            if value == self[i] {
                return Some(i);
            }
        }
        return None;
    }
}

fn dijkstra(adj_list: &Vec<Vec<usize>>, weights: &Vec<Vec<usize>>, src: usize) -> Vec<usize> {
    let n = adj_list.len();
    let mut dist_min_bottleneck = vec![std::usize::MAX; n];
    dist_min_bottleneck[src] = 0;
    let mut heap: MinHeap = MinHeap::new();

    let mut explored: Vec<bool> = vec![false; n];
    explored[src] = true;

    for (&nex, w) in adj_list[src].iter().zip(weights[src].clone()) {
        if !heap.node.contains(&nex) {
            heap.insert(w, nex);
        } else if w < heap.value[heap.node.find_index(nex).unwrap()].clone() as usize {
            heap.delete(heap.node.find_index(nex).unwrap());
            heap.insert(w, nex);
        }
    }

    for _ in 0..n - 1 {
        if heap.value.is_empty() {
            break;
        }
        let (mut score, mut curr) = (0usize, 0usize);
        let extract_min = heap.clone().extract_min();
        if let Some(value) = extract_min {
            (score, curr) = value;
        }
        heap.delete(0);
        dist_min_bottleneck[curr] = score;
        explored[curr] = true;

        for (&nex, w) in adj_list[curr].iter().zip(weights[curr].clone()) {
            if explored[nex] {
                continue;
            }
            if !heap.node.contains(&nex) {
                heap.insert(max(dist_min_bottleneck[curr], w), nex);
            } else {
                let idx = heap.node.find_index(nex).unwrap();
                if max(dist_min_bottleneck[curr], w) < heap.value[idx] {
                    heap.delete(idx);
                    heap.insert(max(dist_min_bottleneck[curr], w), nex);
                }
            }
        }
    }
    return dist_min_bottleneck;
}

fn main() -> std::io::Result<()> {
    let stdin = io::stdin();
    let stdout = io::stdout();

    let mut s = Scanner::new(stdin.lock());
    let mut out = BufWriter::new(stdout.lock());

    let (n, m, q) = (s.cin::<usize>(), s.cin::<usize>(), s.cin::<usize>());

    let mut adj_lists: Vec<Vec<usize>> = vec![vec![]; n];
    let mut weights: Vec<Vec<usize>> = vec![vec![]; n];

    for _ in 0..m {
        let (u, v, w) = (s.cin::<usize>(), s.cin::<usize>(), s.cin::<usize>());
        adj_lists[u - 1].push(v - 1);
        adj_lists[v - 1].push(u - 1);
        weights[u - 1].push(w);
        weights[v - 1].push(w);
    }

    for _ in 0..q {
        let (u, v) = (s.cin::<usize>(), s.cin::<usize>());
        let ans: Vec<usize> = dijkstra(&adj_lists, &weights, u - 1);
        if ans[v - 1] == std::usize::MAX {
            writeln!(out, "{}", -1)?;
        } else {
            writeln!(out, "{:?}", ans[v - 1])?
        };
    }

    Ok(())
}
