
pub mod sample
{
    use vptree::vptree::HeapElement;
    use min_max_heap::MinMaxHeap;
    use rand::Rng;

    #[derive(Debug, Clone, Copy, Eq)]
    pub struct Point
    {
        pub id: i32,
        pub x: i32,
        pub y: i32
    }
    
    impl Point
    {
        // squared distance will not work with vp trees since they dont satisfy triangle inequality
        // https://en.wikipedia.org/wiki/Euclidean_distance#Squared_Euclidean_distance
        pub fn distance(a: &Point, b: &Point) -> f64
        {
            (((a.x - b.x).pow(2) + (a.y - b.y).pow(2)) as f64).sqrt()
        }
    }
    
    impl PartialEq for Point
    {
        fn eq(&self, other: &Self) -> bool
        {
            self.id == other.id
        }
    }

    pub fn generate_data(n: i32) -> Vec<Point>
    {
        let rng = &mut rand::thread_rng();
        (0..n).map(|id| Point { id, x: rng.gen_range(0..1000), y: rng.gen_range(0..1000) }).collect::<Vec<Point>>()
    }

    // bruteforce knn search O(n)
    pub fn bf_knn(d: &Vec<Point>, p: &Point, k: usize) -> Vec<(Point, f64)>
    {
        let mut h = MinMaxHeap::<HeapElement>::new();
        for (i, v) in d.iter().enumerate()
        {
            if p == v { continue; } 
            h.push(HeapElement(i, Point::distance(p, v)));
            if h.len() > k { h.pop_max(); }
        }

        return h.into_vec_asc().iter().map(|e| (d[e.0], e.1)).collect()
    }
}