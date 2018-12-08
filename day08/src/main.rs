use std::collections::HashMap;

fn main() {
    let input: Vec<usize> = include_str!("input.txt")
        .trim()
        .split(" ")
        .map(|x| x.parse().unwrap())
        .collect();

    let mut nodes: Vec<Node> = vec![];
    let mut edges: HashMap<usize, Vec<usize>> = HashMap::new();
    {
        let mut stack: Vec<Frame> = vec![];
        stack.push(Frame {
            id: 0,
            children_left: 1,
            num_metadata: 0,
        });
        let mut id = 1usize;
        let mut index = 0;
        loop {
            let frame = stack.last_mut().expect("wat");
            if frame.children_left > 0 {
                let num_children = input[index];
                let num_metadata = input[index + 1];
                index += 2;

                let node_id = id;
                id += 1;

                frame.children_left -= 1;
                stack.push(Frame {
                    id: node_id,
                    children_left: num_children,
                    num_metadata,
                });
            } else {
                let mut metadata = vec![];
                for _ in 0..frame.num_metadata {
                    metadata.push(input[index]);
                    index += 1;
                }
                nodes.push(Node {
                    id: frame.id,
                    metadata,
                });
                stack.pop().unwrap();
                if stack.len() < 1 {
                    break;
                }
            }
        }
    }
    println!(
        "Part 1: {}",
        nodes.iter().flat_map(|x| &x.metadata).sum::<usize>()
    );
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Frame {
    id: usize,
    children_left: usize,
    num_metadata: usize,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Node {
    id: usize,
    metadata: Vec<usize>,
}
