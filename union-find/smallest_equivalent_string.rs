impl Solution {
    pub fn smallest_equivalent_string(s1: String, s2: String, base_str: String) -> String {
        let mut parent = (0..26).collect::<Vec<_>>();

        fn find(x: usize, parent: &mut Vec<usize>) -> usize {
            if parent[x] != x {
                parent[x] = find(parent[x], parent);
            }
            parent[x]
        }

        fn union(x: usize, y: usize, parent: &mut Vec<usize>) {
            let px = find(x, parent);
            let py = find(y, parent);
            if px != py {
                // Always link the larger root to the smaller one (to maintain lexicographic order)
                if px < py {
                    parent[py] = px;
                } else {
                    parent[px] = py;
                }
            }
        }

        for (a, b) in s1.bytes().zip(s2.bytes()) {
            union((a - b'a') as usize, (b - b'a') as usize, &mut parent);
        }

        base_str
            .bytes()
            .map(|c| {
                let root = find((c - b'a') as usize, &mut parent);
                (b'a' + root as u8) as char
            })
            .collect()
    }
}
