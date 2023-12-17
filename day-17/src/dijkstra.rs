// Copied from https://github.com/evenfurther/pathfinding and slightly modified

//! Compute a shortest path using the [Dijkstra search
//! algorithm](https://en.wikipedia.org/wiki/Dijkstra's_algorithm).

use indexmap::map::Entry::{Occupied, Vacant};
use num_traits::Zero;
use rustc_hash::{FxHashMap, FxHashSet};
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::hash::Hash;
use std::usize;

use indexmap::IndexMap;
use rustc_hash::FxHasher;
use std::hash::BuildHasherDefault;

pub type FxIndexMap<K, V> = IndexMap<K, V, BuildHasherDefault<FxHasher>>;

/// Compute a shortest path using the [Dijkstra search
/// algorithm](https://en.wikipedia.org/wiki/Dijkstra's_algorithm).
///
/// The shortest path starting from `start` up to a node for which `success` returns `true` is
/// computed and returned along with its total cost, in a `Some`. If no path can be found, `None`
/// is returned instead.
///
/// - `start` is the starting node.
/// - `successors` returns a list of successors for a given node, along with the cost for moving
/// from the node to the successor. This cost must be non-negative.
/// - `success` checks whether the goal has been reached. It is not a node as some problems require
/// a dynamic solution instead of a fixed node.
///
/// A node will never be included twice in the path as determined by the `Eq` relationship.
///
/// The returned path comprises both the start and end node.
///
/// # Example
///
/// We will search the shortest path on a chess board to go from (1, 1) to (4, 6) doing only knight
/// moves.
///
/// The first version uses an explicit type `Pos` on which the required traits are derived.
///
/// ```
/// use pathfinding::prelude::dijkstra;
///
/// #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
/// struct Pos(i32, i32);
///
/// impl Pos {
///   fn successors(&self) -> Vec<(Pos, usize)> {
///     let &Pos(x, y) = self;
///     vec![Pos(x+1,y+2), Pos(x+1,y-2), Pos(x-1,y+2), Pos(x-1,y-2),
///          Pos(x+2,y+1), Pos(x+2,y-1), Pos(x-2,y+1), Pos(x-2,y-1)]
///          .into_iter().map(|p| (p, 1)).collect()
///   }
/// }
///
/// static GOAL: Pos = Pos(4, 6);
/// let result = dijkstra(&Pos(1, 1), |p| p.successors(), |p| *p == GOAL);
/// assert_eq!(result.expect("no path found").1, 4);
/// ```
///
/// The second version does not declare a `Pos` type, makes use of more closures,
/// and is thus shorter.
///
/// ```
/// use pathfinding::prelude::dijkstra;
///
/// static GOAL: (i32, i32) = (4, 6);
/// let result = dijkstra(&(1, 1),
///                       |&(x, y)| vec![(x+1,y+2), (x+1,y-2), (x-1,y+2), (x-1,y-2),
///                                      (x+2,y+1), (x+2,y-1), (x-2,y+1), (x-2,y-1)]
///                                  .into_iter().map(|p| (p, 1)),
///                       |&p| p == GOAL);
/// assert_eq!(result.expect("no path found").1, 4);
/// ```
pub fn dijkstra<N, C, FN, IN, FS>(
    start: &N,
    mut successors: FN,
    mut success: FS,
) -> Option<(Vec<N>, C)>
where
    N: Eq + Hash + Clone,
    C: Zero + Ord + Copy,
    FN: FnMut(&N, &FxIndexMap<N, (usize, C)>, usize) -> IN,
    IN: IntoIterator<Item = (N, C)>,
    FS: FnMut(&N) -> bool,
{
    dijkstra_internal(start, &mut successors, &mut success)
}

pub(crate) fn dijkstra_internal<N, C, FN, IN, FS>(
    start: &N,
    successors: &mut FN,
    success: &mut FS,
) -> Option<(Vec<N>, C)>
where
    N: Eq + Hash + Clone,
    C: Zero + Ord + Copy,
    FN: FnMut(&N, &FxIndexMap<N, (usize, C)>, usize) -> IN,
    IN: IntoIterator<Item = (N, C)>,
    FS: FnMut(&N) -> bool,
{
    let (parents, reached) = run_dijkstra(start, successors, success);
    reached.map(|target| {
        (
            reverse_path(&parents, |&(p, _)| p, target),
            parents.get_index(target).unwrap().1 .1,
        )
    })
}

fn run_dijkstra<N, C, FN, IN, FS>(
    start: &N,
    successors: &mut FN,
    stop: &mut FS,
) -> (FxIndexMap<N, (usize, C)>, Option<usize>)
where
    N: Eq + Hash + Clone,
    C: Zero + Ord + Copy,
    FN: FnMut(&N, &FxIndexMap<N, (usize, C)>, usize) -> IN,
    IN: IntoIterator<Item = (N, C)>,
    FS: FnMut(&N) -> bool,
{
    let mut to_see = BinaryHeap::new();
    to_see.push(SmallestHolder {
        cost: Zero::zero(),
        index: 0,
    });
    let mut parents: FxIndexMap<N, (usize, C)> = FxIndexMap::default();
    parents.insert(start.clone(), (usize::max_value(), Zero::zero()));
    let mut target_reached = None;
    while let Some(SmallestHolder { cost, index }) = to_see.pop() {
        let successors = {
            let (node, _) = parents.get_index(index).unwrap();
            if stop(node) {
                target_reached = Some(index);
                break;
            }
            successors(node, &parents, index)
        };
        for (successor, move_cost) in successors {
            let new_cost = cost + move_cost;
            let n;
            match parents.entry(successor) {
                Vacant(e) => {
                    n = e.index();
                    e.insert((index, new_cost));
                }
                Occupied(mut e) => {
                    if e.get().1 > new_cost {
                        n = e.index();
                        e.insert((index, new_cost));
                    } else {
                        continue;
                    }
                }
            }

            to_see.push(SmallestHolder {
                cost: new_cost,
                index: n,
            });
        }
    }
    (parents, target_reached)
}

struct SmallestHolder<K> {
    cost: K,
    index: usize,
}

impl<K: PartialEq> PartialEq for SmallestHolder<K> {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}

impl<K: PartialEq> Eq for SmallestHolder<K> {}

impl<K: Ord> PartialOrd for SmallestHolder<K> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<K: Ord> Ord for SmallestHolder<K> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

/// Struct returned by [`dijkstra_reach`].
pub struct DijkstraReachable<N, C, FN> {
    to_see: BinaryHeap<SmallestHolder<C>>,
    seen: FxHashSet<usize>,
    parents: FxIndexMap<N, (usize, C)>,
    total_costs: FxHashMap<N, C>,
    successors: FN,
}

/// Information about a node reached by [`dijkstra_reach`].
#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct DijkstraReachableItem<N, C> {
    /// The node that was reached by [`dijkstra_reach`].
    pub node: N,
    /// The previous node that the current node came from.
    /// If the node is the first node, there will be no parent.
    pub parent: Option<N>,
    /// The total cost from the starting node.
    pub total_cost: C,
}

impl<N, C, FN, IN> Iterator for DijkstraReachable<N, C, FN>
where
    N: Eq + Hash + Clone,
    C: Zero + Ord + Copy + Hash,
    FN: FnMut(&N, C) -> IN,
    IN: IntoIterator<Item = (N, C)>,
{
    type Item = DijkstraReachableItem<N, C>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(SmallestHolder { cost, index }) = self.to_see.pop() {
            if !self.seen.insert(index) {
                continue;
            }
            let item;
            let successors = {
                let (node, (parent_index, _)) = self.parents.get_index(index).unwrap();
                let total_cost = self.total_costs[node];
                item = Some(DijkstraReachableItem {
                    node: node.clone(),
                    parent: self.parents.get_index(*parent_index).map(|x| x.0.clone()),
                    total_cost,
                });
                (self.successors)(node, total_cost)
            };
            for (successor, move_cost) in successors {
                let new_cost = cost + move_cost;
                let n;
                match self.parents.entry(successor.clone()) {
                    Vacant(e) => {
                        n = e.index();
                        e.insert((index, new_cost));
                        self.total_costs.insert(successor.clone(), new_cost);
                    }
                    Occupied(mut e) => {
                        if e.get().1 > new_cost {
                            n = e.index();
                            e.insert((index, new_cost));
                            self.total_costs.insert(successor.clone(), new_cost);
                        } else {
                            continue;
                        }
                    }
                }

                self.to_see.push(SmallestHolder {
                    cost: new_cost,
                    index: n,
                });
            }
            return item;
        }

        None
    }
}

#[allow(clippy::needless_collect)]
fn reverse_path<N, V, F>(parents: &FxIndexMap<N, V>, mut parent: F, start: usize) -> Vec<N>
where
    N: Eq + Hash + Clone,
    F: FnMut(&V) -> usize,
{
    let mut i = start;
    let path = std::iter::from_fn(|| {
        parents.get_index(i).map(|(node, value)| {
            i = parent(value);
            node
        })
    })
    .collect::<Vec<&N>>();
    // Collecting the going through the vector is needed to revert the path because the
    // unfold iterator is not double-ended due to its iterative nature.
    path.into_iter().rev().cloned().collect()
}

pub fn path_n<N, V, F>(
    parents: &FxIndexMap<N, V>,
    mut parent: F,
    start: usize,
    skip: usize,
    len: usize,
) -> Vec<N>
where
    N: Eq + Hash + Clone,
    F: FnMut(&V) -> usize,
{
    let mut i = start;
    std::iter::from_fn(|| {
        parents.get_index(i).map(|(node, value)| {
            i = parent(value);
            node
        })
    })
    .skip(skip)
    .take(len)
    .cloned()
    .collect::<Vec<N>>()
}
