#![feature(test)]

mod lib;
use std::{fmt::Debug};
use rand::{Rng};
use crate::{lib::vptree::VPTree, sample::sample::Point};
mod sample;

#[cfg(test)]
mod tests
{
    use std::collections::HashSet;

    use vptree::vptree::VPTree;
    use crate::sample::sample::{Point, generate_data, bf_knn};

    #[test]
    fn validate_correct()
    {
        let k = 5;
        let data = generate_data(100);
        let tree = VPTree::new(&data, &Point::distance);
        assert_eq!(tree.items.len(), data.len());

        for p in &data
        {
            let test = tree.search(p, k);
            let validation = bf_knn(&data, p, k);
            if test.len() != k
            {
                println!("{} {:#?} {:#?}", p.id, test, validation);
            }
            assert_eq!(test.len(), validation.len());

            let s1 = test.iter().map(|a| a.0.id).collect::<HashSet<_>>();
            let s2 = validation.iter().map(|a| a.0.id).collect::<HashSet<_>>();
            let d = s1.symmetric_difference(&s2).collect::<HashSet<_>>();
            if d.len() != 0
            {
                println!("{} {:#?} {:#?}", p.id, test, validation);
            }
            assert_eq!(d.len(), 0);
        }
    }

}

fn main()
{
    let rng = &mut rand::thread_rng();
    let data = &mut (0..1000000).map(|id| Point { id, x: rng.gen_range(0..1000), y: rng.gen_range(0..1000) }).collect::<Vec<Point>>();

    let t = VPTree::new(data, &Point::distance);
    // println!("{:#?}", &t.root);

    // println!("{:#?}", &data[0]);
    for (k, i) in data.iter().enumerate()
    {
        let res = t.search(i, 5);
    }
    print!("");
    // let res = t.search(&data[0], 5);
    // println!("{:#?}", res);
}
