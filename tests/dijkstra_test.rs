extern crate subway;

use std::cmp::Ordering;
use std::fmt;
use std::hash::Hash;
use subway::dijkstra::*;

#[derive(Clone)]
struct SimpleWeight {
    weight: usize,
    is_infinity: bool,
}

impl PartialEq for SimpleWeight {
    fn eq(&self, other: &SimpleWeight) -> bool {
        (self.is_infinity && other.is_infinity)
            || (self.is_infinity == other.is_infinity && self.weight == other.weight)
    }
}

impl Eq for SimpleWeight {}

impl PartialOrd for SimpleWeight {
    fn partial_cmp(&self, other: &SimpleWeight) -> Option<Ordering> {
        Some(Ord::cmp(self, other))
    }
}

impl Ord for SimpleWeight {
    fn cmp(&self, other: &SimpleWeight) -> Ordering {
        match (self.is_infinity, other.is_infinity) {
            (true, true) => Ordering::Equal,
            (true, false) => Ordering::Greater,
            (false, true) => Ordering::Less,
            (false, false) => self.weight.cmp(&other.weight),
        }
    }
}

impl Weight for SimpleWeight {
    fn add(&self, other: &Self) -> Self {
        SimpleWeight {
            weight: self.weight + other.weight,
            is_infinity: self.is_infinity || other.is_infinity,
        }
    }

    fn is_infinity(&self) -> bool {
        self.is_infinity
    }

    fn zero() -> Self {
        SimpleWeight {
            weight: 0,
            is_infinity: false,
        }
    }

    fn infinity() -> Self {
        SimpleWeight {
            weight: 0,
            is_infinity: true,
        }
    }
}

impl fmt::Debug for SimpleWeight {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            if self.is_infinity {
                "Inf".to_owned()
            } else {
                format!("{}", self.weight)
            }
        )
    }
}

struct SimpleEdge<'a> {
    to: &'a SimpleVertex<'a>,
    weight: SimpleWeight,
}

impl<'a> Edge<'a, SimpleVertex<'a>, SimpleWeight> for SimpleEdge<'a> {
    fn get_to(&'a self) -> &'a SimpleVertex {
        &self.to
    }

    fn get_weight(&self) -> &SimpleWeight {
        &self.weight
    }
}

impl<'a> fmt::Debug for SimpleEdge<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "-{:?}> {}", self.weight, self.to.name)
    }
}

struct SimpleVertex<'a> {
    name: String,
    edges: Vec<SimpleEdge<'a>>,
}

impl<'a> Hash for SimpleVertex<'a> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl<'a> PartialEq for SimpleVertex<'a> {
    fn eq(&self, other: &SimpleVertex) -> bool {
        self.name.eq(&other.name)
    }
}

impl<'a> Eq for SimpleVertex<'a> {}

impl<'a> Vertex<'a, SimpleEdge<'a>, SimpleWeight> for SimpleVertex<'a> {
    type Edges = std::slice::Iter<'a, SimpleEdge<'a>>;
    fn edges(&'a self) -> Self::Edges {
        self.edges.iter()
    }
}

impl<'a> fmt::Debug for SimpleVertex<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {:?}", self.name, self.edges)
    }
}

impl<'a> SimpleVertex<'a> {
    fn new(name: String) -> Self {
        SimpleVertex {
            name,
            edges: vec![],
        }
    }

    fn add_edge(&mut self, to: &'a SimpleVertex, weight: usize) {
        self.edges.push(SimpleEdge {
            to,
            weight: SimpleWeight {
                weight,
                is_infinity: false,
            },
        });
    }
}

#[test]
fn dijkstra_basic_test() {
    let d = SimpleVertex::new("D".to_owned());
    let mut c = SimpleVertex::new("C".to_owned());
    let b = SimpleVertex::new("B".to_owned());
    let mut s = SimpleVertex::new("S".to_owned());

    c.add_edge(&d, 12);
    s.add_edge(&b, 24);
    s.add_edge(&c, 3);
    s.add_edge(&d, 20);

    let list = vec![&s, &b, &c, &d];

    let dijkstra = Dijkstra::new(list);

    let to_b = dijkstra.find_shorted_path(vec![&s], vec![&b]);
    assert_eq!(to_b.0, vec![&s, &b]);
    assert_eq!(to_b.1.weight, 24);

    let to_c = dijkstra.find_shorted_path(vec![&s], vec![&c]);
    assert_eq!(to_c.0, vec![&s, &c]);
    assert_eq!(to_c.1.weight, 3);

    let to_d = dijkstra.find_shorted_path(vec![&s], vec![&d]);
    assert_eq!(to_d.0, vec![&s, &c, &d]);
    assert_eq!(to_d.1.weight, 15);
}
