
pub mod vptree
{
    use std::{cmp::Ordering, fmt::Debug};
    use min_max_heap::MinMaxHeap;
    use rand::{Rng, prelude::ThreadRng};

    // static mut COUNTER: i32 = 0;

    type Link = Option<Box<Node>>;

    #[derive(Debug)]
    pub struct Node
    {
        pub index: usize,
        pub radius: f64,
        pub left: Link,
        pub right: Link
    }

    impl Node
    {
        fn new(value: usize) -> Self
        {
            Node {
                index: value,
                radius: 0.,
                left: None,
                right: None
            }
        }

        fn new_empty(value: usize) -> Link
        {
            Some(Box::new(Node::new(value)))
        }
    }

    pub struct VPTree<T> where
    {
        pub root: Link,
        pub items: Vec<T>,
        distance: Box<dyn Fn(&T, &T) -> f64>
    }

    impl<'a, T: Copy + Clone + Eq + PartialEq + 'a> VPTree<T> where
    {
        pub fn new(items: &Vec<T>, distance: &'static dyn Fn(&T, &T) -> f64) -> Self
        {
            let rng = &mut rand::thread_rng();
            let items = &mut items.clone();
            let root = VPTree::construct_node(items, distance, rng, 0, items.len());
            VPTree { root, items: items.to_vec(), distance: Box::new(distance) }
        }

        fn compare_distance(distance: &'static dyn Fn(&T, &T) -> f64, r: T) -> impl Fn(&T, &T) -> Ordering
        {
            move |a: &T, b: &T|
                distance(&r, a).partial_cmp(&distance(&r, b)).unwrap_or(Ordering::Equal)
        }

        fn construct_node(items: &mut Vec<T>, distance: &'static dyn Fn(&T, &T) -> f64, rng: &mut ThreadRng, start: usize, end: usize) -> Link
        {
            if start == end
            {
                None
            }
            else if end - start > 1
            {
                let i = rng.gen_range(start..end);
                items.swap(start, i);
                
                let median = (start + end) / 2;
                let f = VPTree::compare_distance(distance, items[start]);
                (&mut items[start + 1..end]).select_nth_unstable_by((end - start) / 2 - 1, f);

                let mut node = Node::new(start);
                node.radius = distance(&items[median], &items[start]);
                node.left = VPTree::construct_node(items, distance, rng, start + 1, median);
                node.right = VPTree::construct_node(items, distance, rng,median, end);
                Some(Box::new(node))
            }
            else
            {
                Node::new_empty(start)
            }
        }

        pub fn search(&self, target: &T, k: usize) -> Vec<(T, f64)>
        {
            let mut heap = MinMaxHeap::<HeapElement>::with_capacity(k);
            self._search(&self.root, target, k, &mut heap);
            heap.into_vec_asc().iter().map(|e| (self.items[e.0], e.1)).collect()
        }

        fn _search(&self, node: &Link, target: &T, k: usize, heap: &mut MinMaxHeap<HeapElement>)
        {
            let node = match node {
                Some(n) => n.as_ref(),
                None => return
            };
            
            // unsafe
            // {
            //     COUNTER += 1;
            //     println!("iter {}", COUNTER);
            // }

            // how about you mutably borrow these nuts
            macro_rules! tau {() => { if heap.len() == k { heap.peek_max().and_then(|e| Some(e.1)).unwrap() } else { f64::INFINITY } }}
            macro_rules! search {($n: expr) => { self._search($n, target, k, heap) }}

            let e = &self.items[node.index];
            
            let dist = (self.distance)(e, target);
            if dist <= tau!() && e != target
            {
                let h = HeapElement(node.index, dist);
                if heap.len() == k
                {
                    heap.push_pop_max(h);
                }
                else
                {
                    heap.push(h);
                }
            }

            if node.left.is_none() && node.right.is_none()
            {
                return;
            }

            if dist <= node.radius
            {
                if dist - tau!() <= node.radius { search!(&node.left); }
                if dist + tau!() >= node.radius { search!(&node.right); }
            }
            else
            {
                if dist + tau!() >= node.radius { search!(&node.right); }
                if dist - tau!() <= node.radius { search!(&node.left); }
            }
        }

    }

    #[derive(PartialEq, Debug)]
    pub struct HeapElement(pub usize, pub f64);

    impl Eq for HeapElement {}

    impl PartialOrd for HeapElement
    {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering>
        {
            self.1.partial_cmp(&other.1)
        }
    }

    impl Ord for HeapElement
    {
        fn cmp(&self, other: &HeapElement) -> Ordering
        {
            self.partial_cmp(other).unwrap()
        }
    }
}

