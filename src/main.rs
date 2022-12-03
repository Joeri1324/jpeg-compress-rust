use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;

struct HuffmanNode {
    left: Option<Box<HuffmanNode>>,
    right: Option<Box<HuffmanNode>>,
    value: Option<char>,
    frequency: i32,
}

impl PartialEq for HuffmanNode {
    fn eq(&self, other: &Self) -> bool {
        self.frequency == other.frequency
    }
}

impl Ord for HuffmanNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.frequency.cmp(&other.frequency)
    }
}

impl Eq for HuffmanNode {}

impl PartialOrd for HuffmanNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.frequency.partial_cmp(&other.frequency)
    }
}

struct HuffmanTree {
    table: HashMap<char, Vec<bool>>,
    root: Box<HuffmanNode>,
}

fn build_table(node: &HuffmanNode, code: Vec<bool>, table: &mut HashMap<char, Vec<bool>>) {
    match node.value {
        Some(value) => {
            display_code(&code);
            table.insert(value, code);
            return;
        }
        None => {}
    }

    match &node.left {
        Some(child) => {
            let mut child_code = code.clone();
            child_code.push(false);
            build_table(&*child, child_code, table);
        }
        None => panic!("Something strange happening :/"),
    }

    match &node.right {
        Some(child) => {
            let mut child_code = code.clone();
            child_code.push(true);
            build_table(&*child, child_code, table);
        }
        None => panic!("Something strange happening :/"),
    }
}

fn build_huffman_from_frequencies(frequencies: &HashMap<char, i32>) -> HuffmanTree {
    let mut unique_chars: Vec<&char> = frequencies.keys().collect();
    unique_chars.sort_by(|a, b| {
        frequencies
            .get(*a)
            .unwrap()
            .cmp(&frequencies.get(*b).unwrap())
    });
    let mut heap = BinaryHeap::new();

    for char in &unique_chars {
        let node = HuffmanNode {
            left: None,
            right: None,
            value: Some(**char),
            frequency: *frequencies.get(*char).unwrap(),
        };
        heap.push(Reverse(node));
    }

    while heap.len() > 1 {
        if let Some(Reverse(left)) = heap.pop() {
            if let Some(Reverse(right)) = heap.pop() {
                let new_frequency = left.frequency + right.frequency;
                let new_node = HuffmanNode {
                    left: Some(Box::new(left)),
                    right: Some(Box::new(right)),
                    frequency: new_frequency,
                    value: None,
                };
                heap.push(Reverse(new_node));
            };
        };
    }

    if let Some(Reverse(root)) = heap.pop() {
        let mut table = HashMap::new();
        build_table(&root, Vec::new(), &mut table);

        return HuffmanTree { table: table, root: Box::new(root) };
    } else {
        panic!("Something strange going on :/")
    }
}

impl HuffmanTree {
    fn new(chars: &[char]) -> HuffmanTree {
        let frequencies = get_frequencies(&chars);
        return build_huffman_from_frequencies(&frequencies);
    }

    fn get_code(&self, char: char) -> Option<&Vec<bool>> {
        self.table.get(&char)
    }

    fn encode(&self, chars: &[char]) -> Vec<bool> {
        let mut result = Vec::new();
        for c in chars {
            let code_option = self.get_code(*c);
            match code_option {
                Some(code) => {
                    result.extend(code);
                }
                None => {
                    panic!("'{c}' was not found in huffman tree :: Failed to encode")
                }
            }
        }
        return result
    }

    fn decode(&self, code: &Vec<bool>) -> Vec<char> {
        let mut result = Vec::new();

        let mut current_node = &*self.root;
        for bit in code {
            println!("{bit}");
            match &bit {
                true => {
                    if let Some(new_current_node) = &current_node.right {
                        current_node = new_current_node;
                    }
                }
                false => {
                    if let Some(new_current_node) = &current_node.left {
                        current_node = new_current_node;
                    }
                }
            }
            match current_node.value {
                Some(value) => {
                    result.push(value);
                    current_node = &*self.root;
                }
                None => {

                }
            }
        }

        return result;
    }
}

fn get_frequencies(chars: &[char]) -> HashMap<char, i32> {
    let mut frequencies = HashMap::new();

    for c in chars {
        frequencies.entry(*c).and_modify(|x| *x += 1).or_insert(1);
    }

    return frequencies;
}

fn code_to_string(code: &Vec<bool>) -> String {
    let code_chars: Vec<&str> = code.iter().map(|x| if *x { "1" } else { "0" }).collect();
    let code_string = code_chars.join("");
    return code_string
}

fn display_code(code: &Vec<bool>) {
    let code_string = code_to_string(code);
    println!("Code: {code_string}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_huffman_from_frequencies() {
        let frequencies = HashMap::from([
            ('f', 5),
            ('e', 9),
            ('c', 12),
            ('b', 13),
            ('d', 16),
            ('a', 45),
        ]);
        let huffman_tree = build_huffman_from_frequencies(&frequencies);

        let f_code = huffman_tree.get_code('f').unwrap();
        let e_code = huffman_tree.get_code('e').unwrap();
        let c_code = huffman_tree.get_code('c').unwrap();
        let b_code = huffman_tree.get_code('b').unwrap();
        let d_code = huffman_tree.get_code('d').unwrap();
        let a_code = huffman_tree.get_code('a').unwrap();

        assert_eq!(f_code, &[true, true, false, false]);
        assert_eq!(e_code, &[true, true, false, true]);
        assert_eq!(c_code, &[true, false, false]);
        assert_eq!(b_code, &[true, false, true]);
        assert_eq!(d_code, &[true, true, true]);
        assert_eq!(a_code, &[false]);
    }
}

fn char_vec_to_string(chars: &Vec<char>) -> String {
    let strings: Vec<String> = chars.iter().map(|x| x.to_string()).collect();
    return strings.join("");
}


fn char_slice_to_string(chars: &[char]) -> String {
    let strings: Vec<String> = chars.iter().map(|x| x.to_string()).collect();
    return strings.join("");
}

fn main() {
    let chars = ['a', 'a', 'b', 'c', 'd', 'd', 'd', 'e', 'e', 'd'];
    let huffman_tree = HuffmanTree::new(&chars);
    let encoded = huffman_tree.encode(&chars);
    let decoded = huffman_tree.decode(&encoded);

    println!("Input: \t\t{}", char_slice_to_string(&chars));
    println!("Encoded: \t{}", code_to_string(&encoded));
    println!("Decoded: \t{}", char_vec_to_string(&decoded));
}
