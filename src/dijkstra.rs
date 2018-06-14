use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::hash::Hash;
use std::iter::{self, FromIterator};
use std::marker::{PhantomData, Sized};

pub trait Weight
where
    Self: Clone + Ord,
{
    fn add(&self, other: &Self) -> Self;
    fn zero() -> Self;
    fn infinity() -> Self;
    fn is_infinity(&self) -> bool;
}

pub trait Edge<'a, V, W>
where
    Self: Sized + 'a,
    V: Vertex<'a, Self, W> + 'a,
    W: Weight,
{
    fn get_to(&'a self) -> &'a V;
    fn get_weight(&self) -> &W;
}

pub trait Vertex<'a, E, W>
where
    Self: Sized + Eq + Hash + 'a,
    E: Edge<'a, Self, W> + 'a,
    W: Weight,
{
    type Edges: IntoIterator<Item = &'a E>;
    fn edges(&'a self) -> Self::Edges;
}

#[derive(PartialEq, Eq)]
struct UnvisitedVertex<W>
where
    W: Weight,
{
    index: usize,
    weight: W,
}

impl<W> PartialOrd for UnvisitedVertex<W>
where
    W: Weight,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(other.weight.cmp(&self.weight))
    }
}

impl<W> Ord for UnvisitedVertex<W>
where
    W: Weight,
{
    fn cmp(&self, other: &Self) -> Ordering {
        other.weight.cmp(&self.weight)
    }
}

pub struct Dijkstra<'a, V, E, W>
where
    V: Vertex<'a, E, W> + 'a,
    E: Edge<'a, V, W> + 'a,
    W: Weight,
{
    graph: Vec<&'a V>,
    v_to_index_map: HashMap<&'a V, usize>,
    _marker: PhantomData<(E, W)>,
}

impl<'a, V, E, W> Dijkstra<'a, V, E, W>
where
    V: Vertex<'a, E, W> + 'a,
    E: Edge<'a, V, W> + 'a,
    W: Weight,
{
    pub fn new(list: impl IntoIterator<Item = &'a V>) -> Self {
        let graph: Vec<_> = list.into_iter().collect();
        let v_to_index_map = graph.iter().enumerate().map(|(i, &v)| (v, i)).collect();
        Dijkstra {
            graph,
            v_to_index_map,
            _marker: PhantomData,
        }
    }

    pub fn find_shorted_path(
        &self,
        starts: impl IntoIterator<Item = &'a V>,
        ends: impl IntoIterator<Item = &'a V>,
    ) -> (Vec<&'a V>, W) {
        let mut weights: Vec<_> = iter::repeat_with(W::infinity)
            .take(self.graph.len())
            .collect();
        let mut unvisiteds = BinaryHeap::<UnvisitedVertex<W>>::new();

        let start_set = HashSet::<usize>::from_iter(
            starts
                .into_iter()
                .filter_map(|v| self.v_to_index_map.get(v))
                .map(|&i| i),
        );
        let end_set = HashSet::<usize>::from_iter(
            ends.into_iter()
                .filter_map(|v| self.v_to_index_map.get(v))
                .map(|&i| i),
        );

        for &i in start_set.iter() {
            weights[i] = W::zero();
            unvisiteds.push(UnvisitedVertex {
                index: i,
                weight: weights[i].clone(),
            });
        }

        let mut backtracker: Vec<_> = iter::repeat(0).take(self.graph.len()).collect();
        let mut visiteds: Vec<_> = iter::repeat(false).take(self.graph.len()).collect();

        let start_pair = unvisiteds.pop().unwrap();

        let mut now = start_pair.index;
        let mut weight_sum = start_pair.weight;
        while !end_set.contains(&now) {
            let now_vertex = self.graph[now];

            for edge in now_vertex.edges() {
                let to_vertex = edge.get_to();
                let to = *self.v_to_index_map.get(&to_vertex).unwrap();

                if visiteds[to] {
                    continue;
                }

                let weight = edge.get_weight();

                let added_weight = weight_sum.add(weight);

                if weights[to] > added_weight {
                    weights[to] = added_weight.clone();
                    backtracker[to] = now;
                }

                unvisiteds.push(UnvisitedVertex {
                    index: to,
                    weight: weights[to].clone(),
                })
            }
            visiteds[now] = true;

            let mut next_index = now;
            while visiteds[next_index] {
                let pair = unvisiteds.pop().unwrap();
                next_index = pair.index;
                weight_sum = pair.weight;
            }
            now = next_index;
        }

        let mut route = vec![];

        while !start_set.contains(&now) {
            route.insert(0, self.graph[now]);
            now = backtracker[now];
        }
        route.insert(0, self.graph[now]);

        (route, weight_sum)
    }
}
