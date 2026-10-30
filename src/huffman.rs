use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub fn count_frequencies(data: &[u8]) -> [u64; 256] {
    let mut freq = [0u64; 256];
    for &b in data {
        freq[b as usize] += 1;
    }
    freq
}

#[derive(Debug)]
pub enum Node {
    Leaf(u8),
    Internal(usize, usize),
}

// https://www.geeksforgeeks.org/dsa/huffman-coding-greedy-algo-3/
pub fn build_tree(freq: &[u64; 256]) -> Option<(Vec<Node>, usize)> {
    let mut nodes: Vec<Node> = Vec::new();

    let mut heap = BinaryHeap::new();

    for (byte, &f) in freq.iter().enumerate() {
        if f > 0 {
            nodes.push(Node::Leaf(byte as u8));
            heap.push(Reverse((f, nodes.len() - 1)));
        }
    }

    while heap.len() > 1 {
        let Reverse((w1, i1)) = heap.pop().unwrap();
        let Reverse((w2, i2)) = heap.pop().unwrap();
        nodes.push(Node::Internal(i1, i2));
        heap.push(Reverse((w1 + w2, nodes.len() - 1)));
    }

    match heap.pop() {
        Some(Reverse((_, root))) => Some((nodes, root)),
        None => None,
    }
}

fn walk(nodes: &[Node], node_index: usize, bits_so_far: u32, depth: u8, codes: &mut [(u32, u8); 256]) {
    match nodes[node_index] {
        Node::Leaf(l) => {
            if nodes.len() == 1 {
                codes[l as usize] = (0,1);
            } else {
                codes[l as usize] = (bits_so_far, depth);
            }
        }, 
        Node::Internal(i1,i2) => {
            walk(nodes, i1, bits_so_far << 1, depth+1, codes);
            walk(nodes, i2, (bits_so_far << 1) | 1, depth+1, codes);
        }
    }
}

// Returns, for each byte value, its (code bits, code length).
// Bytes that never appear get (0, 0).
pub fn assign_codes(nodes: &[Node], root: usize) -> [(u32, u8); 256] {
    let mut codes = [(0u32, 0u8); 256];
    walk(nodes, root, 0, 0, &mut codes);
    codes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn frequencies() {
        let f = count_frequencies(b"aaaabbc");
        assert_eq!(f[b'a' as usize], 4);
        assert_eq!(f[b'b' as usize], 2);
        assert_eq!(f[b'c' as usize], 1);
        assert_eq!(f[b'z' as usize], 0);
    }

    #[test]
    fn code_lengths() {
        let f = count_frequencies(b"aaaabbc");
        let (nodes, root) = build_tree(&f).unwrap();
        let codes = assign_codes(&nodes, root);
        assert_eq!(codes[b'a' as usize].1, 1);
        assert_eq!(codes[b'b' as usize].1, 2);
        assert_eq!(codes[b'c' as usize].1, 2);
    }

    #[test]
    fn no_code_is_a_prefix_of_another() {
        let f = count_frequencies(b"aabbbccccdddddd");
        let (nodes, root) = build_tree(&f).unwrap();
        let codes = assign_codes(&nodes, root);
        let used: Vec<(u32, u8)> = codes.iter().copied().filter(|c| c.1 > 0).collect();
        for (i, &(bits_a, len_a)) in used.iter().enumerate() {
            for (j, &(bits_b, len_b)) in used.iter().enumerate() {
                if i != j && len_a <= len_b {
                    // top len_a bits of b equal a means a is a prefix of b
                    assert_ne!(bits_b >> (len_b - len_a), bits_a);
                }
            }
        }
    }

    #[test]
    fn single_symbol_and_empty() {
        let f = count_frequencies(b"aaaa");
        let (nodes, root) = build_tree(&f).unwrap();
        assert_eq!(assign_codes(&nodes, root)[b'a' as usize].1, 1);

        assert!(build_tree(&count_frequencies(b"")).is_none());
    }
}
