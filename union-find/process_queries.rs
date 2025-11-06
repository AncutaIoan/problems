/*
3607. Power Grid Maintenance

You are given an integer c representing c power stations, each with a unique identifier id from 1 to c (1‑based indexing).

These stations are interconnected via n bidirectional cables, represented by a 2D array connections, where each element connections[i] = [ui, vi] indicates a connection between station ui and station vi. Stations that are directly or indirectly connected form a power grid.

Initially, all stations are online (operational).

You are also given a 2D array queries, where each query is one of the following two types:

[1, x]: A maintenance check is requested for station x. If station x is online, it resolves the check by itself. If station x is offline, the check is resolved by the operational station with the smallest id in the same power grid as x. If no operational station exists in that grid, return -1.

[2, x]: Station x goes offline (i.e., it becomes non-operational).

Return an array of integers representing the results of each query of type [1, x] in the order they appear.

Note: The power grid preserves its structure; an offline (non‑operational) node remains part of its grid and taking it offline does not alter connectivity.

 

*/
use std::collections::{BTreeSet, HashMap};

struct DSU {
    parent: Vec<usize>,
}

impl DSU {
    fn new(n: usize) -> Self {
        Self { parent: (0..=n).collect() }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, a: usize, b: usize) {
        let pa = self.find(a);
        let pb = self.find(b);
        if pa != pb {
            self.parent[pb] = pa;
        }
    }
}

impl Solution {
    pub fn process_queries(c: i32, connections: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let n = c as usize;
        let mut dsu = DSU::new(n);

        // Build connected components
        for edge in connections {
            let (u, v) = (edge[0] as usize, edge[1] as usize);
            dsu.union(u, v);
        }

        // Initialize BTreeSets for each component
        let mut sets: HashMap<usize, BTreeSet<usize>> = HashMap::new();
        for i in 1..=n {
            let root = dsu.find(i);
            sets.entry(root).or_default().insert(i);
        }

        let mut online = vec![true; n + 1];
        let mut res = Vec::new();

        // Process queries
        for q in queries {
            let (t, x) = (q[0], q[1] as usize);
            let root = dsu.find(x);
            match t {
                1 => {
                    if online[x] {
                        res.push(x as i32);
                    } else {
                        let set = sets.get(&root);
                        if let Some(s) = set {
                            if let Some(&v) = s.iter().next() {
                                res.push(v as i32);
                            } else {
                                res.push(-1);
                            }
                        } else {
                            res.push(-1);
                        }
                    }
                }
                2 => {
                    if online[x] {
                        online[x] = false;
                        if let Some(s) = sets.get_mut(&root) {
                            s.remove(&x);
                        }
                    }
                }
                _ => {}
            }
        }

        res
    }
}
