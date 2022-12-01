use std::{
    cmp::Ordering,
    collections::BinaryHeap,
    hash::{Hash, Hasher},
};

use ahash::AHashMap;

#[derive(Debug, Clone, Copy)]
pub struct Edge<V> {
    pub vertex: V,
    pub cost: u64
}

pub trait Graph<V: Copy + Hash + Eq> {
    fn neighbors(&self, v: &V) -> Vec<Edge<V>>;

    fn dijsktra(
        &self,
        start: V,
        end: Option<V>,
        keep_previous: bool,
    ) -> (AHashMap<V, u64>, Option<AHashMap<V, V>>) {
        const INF: u64 = std::u64::MAX;

        // The following code is an only slightly modified version of
        // the implementation found at
        // https://doc.rust-lang.org/std/collections/binary_heap/index.html

        #[derive(Copy, Clone, Eq, PartialEq)]
        struct State<V> {
            cost: u64,
            vertex: V,
        }

        // Explicitly implement the trait so the queue becomes a min-heap instead of a max-heap.
        impl<V: Hash + Eq> Ord for State<V> {
            fn cmp(&self, other: &Self) -> Ordering {
                // Notice that the we flip the ordering on costs.
                // In case of a tie we compare positions - this step is necessary
                // to make implementations of `PartialEq` and `Ord` consistent.
                other.cost.cmp(&self.cost).then_with(|| {
                    // This may be unconventional, but it works (...to guarantee
                    // duality, which is the hard thing to do here)
                    let mut self_hasher: ahash::AHasher = Default::default();
                    self.vertex.hash(&mut self_hasher);

                    let mut other_hasher: ahash::AHasher = Default::default();
                    other.vertex.hash(&mut other_hasher);

                    self_hasher.finish().cmp(&other_hasher.finish())
                })
            }
        }

        // `PartialOrd` needs to be implemented as well.
        impl<V: Hash + Eq> PartialOrd for State<V> {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                Some(self.cmp(other))
            }
        }

        let mut dist: AHashMap<V, u64> = Default::default();
        dist.insert(start, 0);

        let mut heap = BinaryHeap::new();
        heap.push(State {
            cost: 0,
            vertex: start,
        });

        let mut prev: Option<AHashMap<V, V>> = if keep_previous {
            Some(Default::default())
        } else {
            None
        };

        while let Some(State { cost, vertex }) = heap.pop() {
            if Some(vertex) == end {
                break;
            }

            // Important as we may have already found a better way
            if cost > dist[&vertex] {
                continue;
            }

            // For each node we can reach, see if we can find a way with
            // a lower cost going through this node
            for Edge { vertex: nv, cost: c }in self.neighbors(&vertex) {
                let next = State {
                    cost: cost + c,
                    vertex: nv,
                };

                if next.cost < *dist.get(&next.vertex).unwrap_or(&INF) {
                    heap.push(next);
                    // Relaxation, we have now found a better way
                    dist.insert(next.vertex, next.cost);

                    if let Some(prev) = &mut prev {
                        prev.insert(next.vertex, vertex);
                    }
                }
            }
        }

        (dist, prev)
    }
}
