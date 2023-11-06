fn iter_vs_into_iter() {
    let vs = vec![1, 2, 3];
    for v in vs.iter(){
        // borrows vs, borrowed v
    }
    for v in &vs{
        // same as iter
        // borrows vs, borrowed v
    }
    for v in vs{
        // consumes vs, owned v
    }
    let vs = vec![1, 2, 3];
    for v in vs.into_iter(){
        // same as for v in vs
        // consumes vs, owned v
    }
}

pub struct Flatten<O> where O: Iterator, O::Item: IntoIterator{
    outer :O,
    inner: Option<<O::Item as IntoIterator>::IntoIter>,
}

pub fn flatten<I>(iter: I) -> Flatten<I> where I:Iterator, I::Item: IntoIterator{
    Flatten::new(iter)
}

impl <O> Flatten<O> where O: Iterator, O::Item: IntoIterator{
    fn new(iter: O) -> Self{
        Flatten{outer: iter, inner: None}
    }
}

impl <O> Iterator for Flatten<O> where O: Iterator, O::Item: IntoIterator{
    type Item = <O::Item as IntoIterator>::Item;
    fn next(&mut self) -> Option<Self::Item>{
        loop {

        if let Some(ref mut inner_iter) = self.inner{
            if let Some(i) = inner_iter.next(){
                return Some(i);
            }
            self.inner = None;
        }

        let next_inner_item = self.outer.next()?.into_iter();
        self.inner = Some(next_inner_item);
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn empty() {
        assert_eq!(flatten(std::iter::empty::<Vec<()>>()).count(), 0);
    }
    #[test]
    fn one() {
        assert_eq!(flatten(std::iter::once(vec![1])).count(), 1);
    }
    #[test]
    fn two() {
        assert_eq!(flatten(std::iter::once(vec![1, 2])).count(), 2);
    }
    #[test]
    fn two_wide() {
        assert_eq!(flatten(vec![vec![1], vec![2]].into_iter()).count(), 2);
    }
}
