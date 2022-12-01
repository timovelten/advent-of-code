use std::cmp::{max, min};

const INPUT: &str = include_str!("../input/day22");

fn interval_intersection((a, b): (i32, i32), (u, v): (i32, i32)) -> (i32, i32) {
    (max(a, u), min(b, v))
}

fn interval_difference((a, b): (i32, i32), (u, v): (i32, i32)) -> [(i32, i32); 2] {
    [(a, min(b, u - 1)), (max(a, v + 1), b)]
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Rectangle<const N: usize> {
    ranges: [(i32, i32); N],
}

impl<const N: usize> Rectangle<N> {
    fn is_empty(&self) -> bool {
        self.ranges.iter().any(|(a, b)| a > b)
    }

    fn count(&self) -> usize {
        if self.is_empty() {
            return 0;
        }

        let mut res = 1;

        for i in 0..N {
            let (a, b) = self.ranges[i];
            res *= (b - a + 1) as usize;
        }

        res
    }

    fn intersect(&self, other: &Rectangle<N>) -> Rectangle<N> {
        let mut ret = Rectangle {
            ranges: [(0, 0); N],
        };

        for i in 0..N {
            ret.ranges[i] = interval_intersection(self.ranges[i], other.ranges[i]);
        }

        ret
    }
}

/// Return the difference of two rectangles as the disjoint union of other rectangles.
/// Note that some of the 6 returned rectangles might be empty.
fn rectangle_difference_3(p: Rectangle<3>, q: Rectangle<3>) -> [Rectangle<3>; 6] {
    // This is how you could prove this somewhat mysterious formula:
    /*
    Assume that $P = S_1 \times S_2 \times S_3$ and $Q = T_1 \times T_2 \times T_3$
    are two rectangles. Then
    \begin{align*} (S_1 \times S_2) \setminus (T_1 \times T_2)
    = (S_1 \setminus T_1) \times S_2 \; \cup \; (S_1 \cap T_1) \times (S_2 \setminus T_2)
    \end{align*}
    and both sets on the right are disjoint. So
    \begin{align*}
        P \setminus Q ={} & (S_1 \times S_2 \times S_3) \setminus (T_1 \times T_2 \times T_3) \\
        ={} & ((S_1 \times S_2) \times S_3) \setminus ((T_1 \times T_2) \times T_3) \\
        ={} & (S_1 \times S_2) \setminus (T_1 \times T_2) \times S_3
            \; \cup \; ((S_1 \times S_2) \cap (T_1 \times T_2)) \times (S_3 \setminus T_3) \\
        ={} & (S_1 \setminus T_1) \times S_2 \times S_3 \; \cup \; (S_1 \cap T_1) \times (S_2 \setminus T_2) \times S_3
            \; \cup \;
            (S_1 \cap T_1) \times (S_2 \cap T_2) \times (S_3 \setminus T_3).
    \end{align*}
    Finally, note that
    $$ [a, b] \setminus [u, v] = [a, b] \cap (-\infty, u) \cup [a, b] \cap (v, \infty) $$
    from which we conclude that
    $$ [a, b] \cap \mathbb{Z} \setminus [u, v] = [a, \min(b, u - 1)] \cup [\max(a, v + 1), b].$$
    */
    let [s1, s2, s3] = p.ranges;
    let [t1, t2, t3] = q.ranges;

    let s1_i_t1 = interval_intersection(s1, t1);
    let s2_i_t2 = interval_intersection(s2, t2);

    let [s1_wo_t1_1, s1_wo_t1_2] = interval_difference(s1, t1);
    let [s2_wo_t2_1, s2_wo_t2_2] = interval_difference(s2, t2);
    let [s3_wo_t3_1, s3_wo_t3_2] = interval_difference(s3, t3);

    [
        // (S1 \ T1) x S2 x S3
        Rectangle {
            ranges: [s1_wo_t1_1, s2, s3],
        },
        Rectangle {
            ranges: [s1_wo_t1_2, s2, s3],
        },
        // (S1 \cap T1) x (S2 \ T2) x S3
        Rectangle {
            ranges: [s1_i_t1, s2_wo_t2_1, s3],
        },
        Rectangle {
            ranges: [s1_i_t1, s2_wo_t2_2, s3],
        },
        // (S1 \cap T1) x (S2 \cap T2) x (S3 \ T3)
        Rectangle {
            ranges: [s1_i_t1, s2_i_t2, s3_wo_t3_1],
        },
        Rectangle {
            ranges: [s1_i_t1, s2_i_t2, s3_wo_t3_2],
        },
    ]
}

#[derive(Debug, Clone, Copy)]
struct Instruction {
    rect: Rectangle<3>,
    on: bool,
}

#[derive(Debug, Clone, PartialEq)]
struct Node {
    rect: Rectangle<3>,
    value: NodeValue,
}

impl Node {
    fn count(&self, val: bool) -> usize {
        match &self.value {
            NodeValue::Leaf(b) if *b == val => self.rect.count(),
            NodeValue::Children(children) => {
                let mut ret = 0;

                for child in children {
                    ret += child.count(val);
                }

                ret
            }
            _ => 0,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
enum NodeValue {
    Leaf(bool),
    Children(Vec<Node>),
}

fn set(root: &mut Node, target: Rectangle<3>, val: bool) {
    fn set_inner(node: &mut Node, target: Rectangle<3>, val: bool) {
        let intersection = node.rect.intersect(&target);

        if intersection.is_empty() {
            // Rectangles are disjoint, we don't have to change anything
            return;
        }

        if intersection == node.rect {
            // node.rect is a subset of target
            node.value = NodeValue::Leaf(val);
            return;
        }

        match &mut node.value {
            NodeValue::Leaf(b) => {
                // This node is too broad to acurately represent the change we want to make,
                // so it has to be subdivided further, which is actually quite tricky

                let mut children = Vec::new();

                // These are the rectangles in node.rect without target, i.e. they can all keep the
                // old value
                for rect in rectangle_difference_3(node.rect, target) {
                    if rect.is_empty() {
                        continue;
                    }

                    children.push(Node {
                        rect,
                        value: NodeValue::Leaf(*b),
                    })
                }

                // Set the new value on the intersection
                children.push(Node {
                    rect: intersection,
                    value: NodeValue::Leaf(val),
                });

                node.value = NodeValue::Children(children);
            }
            NodeValue::Children(nodes) => {
                // Try and change the value in all child nodes. If they are disjoint, this
                // returns immediately
                let mut all_true = true;
                let mut all_false = true;

                for child_node in nodes {
                    set_inner(child_node, target, val);

                    if child_node.value != NodeValue::Leaf(true) {
                        all_true = false;
                    }

                    if child_node.value != NodeValue::Leaf(false) {
                        all_false = false;
                    }
                }

                // Simplification
                if all_true {
                    node.value = NodeValue::Leaf(true);
                }

                if all_false {
                    node.value = NodeValue::Leaf(false);
                }
            }
        }
    }

    set_inner(root, target, val)
}

fn parse_range(s: &str) -> Option<(i32, i32)> {
    let mut split = s.split("..");

    let a = split.next()?.parse::<i32>().ok()?;
    let b = split.next()?.parse::<i32>().ok()?;

    Some((a, b))
}

fn parse_instruction(mut s: &str) -> Option<Instruction> {
    let on = if s.starts_with("on ") {
        s = &s[3..];
        true
    } else if s.starts_with("off ") {
        s = &s[4..];
        false
    } else {
        return None;
    };

    let mut ranges = s.split(',');

    let xs = parse_range(&ranges.next()?[2..])?;
    let ys = parse_range(&ranges.next()?[2..])?;
    let zs = parse_range(&ranges.next()?[2..])?;

    let rect = Rectangle {
        ranges: [xs, ys, zs],
    };

    Some(Instruction { rect, on })
}

pub fn solve() -> crate::Result<()> {
    let instructions = INPUT.lines()
        .map(|line| parse_instruction(&line))
        .collect::<Option<Vec<Instruction>>>()
        .ok_or(crate::Error::InvalidInput)?;

    let init_rect = Rectangle {
        ranges: [(-50, 50); 3],
    };

    let mut root1 = Node {
        rect: init_rect.clone(),
        value: NodeValue::Leaf(false),
    };

    let mut root2 = Node {
        rect: Rectangle {
            ranges: [(-100_000, 100_000); 3],
        },
        value: NodeValue::Leaf(false),
    };

    for Instruction { rect, on } in instructions {
        set(&mut root1, rect, on);
        set(&mut root2, rect, on);
    }

    println!("Problem 1: {}", root1.count(true));
    println!("Problem 2: {}", root2.count(true));

    Ok(())
}
