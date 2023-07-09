#[derive(Debug)]
pub struct ToyVec<T> {
    elements: Box<[T]>,
    len: usize,
}

impl<T: Default> ToyVec<T> {
    pub fn new() -> Self {
        Self::with_capacity(0)
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            elements: Self::allocate_in_heap(capacity),
            len: 0,
        }
    }

    fn allocate_in_heap(size: usize) -> Box<[T]> {
        std::iter::repeat_with(Default::default)
            .take(size)
            .collect::<Vec<_>>()
            .into_boxed_slice()
    }

    pub fn len(&self) -> usize {
        // usize は copy traitがあるのでコピーされる
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn capacity(&self) -> usize {
        self.elements.len()
    }

    pub fn push(&mut self, element: T) {
        if self.len == self.capacity() {
            self.grow()
        }
        self.elements[self.len] = element;
        self.len += 1
    }

    fn grow(&mut self) {
        if self.capacity() == 0 {
            self.elements = Self::allocate_in_heap(1);
        } else {
            let new_elements = Self::allocate_in_heap(self.capacity() * 2);
            let old_elements = std::mem::replace(&mut self.elements, new_elements);
            for (i, elem) in old_elements.into_vec().into_iter().enumerate() {
                self.elements[i] = elem;
            }
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            None
        } else {
            self.len -= 1;
            let elem = std::mem::take(&mut self.elements[self.len]);
            Some(elem)
        }
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        if index < self.len() {
            Some(&self.elements[index])
        } else {
            None
        }
    }

    pub fn get_or<'a, 'b>(&'a self, index: usize, default: &'b T) -> &'a T
    where
        'b: 'a,
    {
        self.get(index).unwrap_or(default)
    }

    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            elements: &self.elements,
            len: self.len,
            pos: 0,
        }
    }
}

impl<T: Default> Default for ToyVec<T> {
    fn default() -> Self {
        Self::new()
    }
}

pub struct Iter<'vec, T> {
    elements: &'vec [T],
    len: usize,
    pos: usize,
}

impl<'vec, T> Iterator for Iter<'vec, T> {
    type Item = &'vec T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= self.len {
            None
        } else {
            let res = Some(&self.elements[self.pos]);
            self.pos += 1;
            res
        }
    }
}

impl<'vec, T: Default> IntoIterator for &'vec ToyVec<T> {
    type Item = &'vec T;
    type IntoIter = Iter<'vec, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_toy_vec_new() {
        let mut toy_vec = ToyVec::<String>::new();

        toy_vec.push("Java".to_string());
        toy_vec.push("Python".to_string());
        toy_vec.push("Rust".to_string());

        let ele = toy_vec.get(1);
        assert_eq!(ele, Some(&"Python".to_string()));
        assert_eq!(toy_vec.capacity(), 4)
    }

    #[test]
    fn test_toy_vec_with_capacity() {
        let mut toy_vec: ToyVec<String> = ToyVec::with_capacity(2);

        toy_vec.push("Java".to_string());
        toy_vec.push("Python".to_string());

        let ele = toy_vec.get(1);
        assert_eq!(ele, Some(&"Python".to_string()))
    }

    #[test]
    fn test_toy_vec_iter() {
        let mut toy_vec: ToyVec<String> = ToyVec::with_capacity(2);

        toy_vec.push("Java".to_string());
        toy_vec.push("Python".to_string());

        let mut iter = toy_vec.iter();

        assert_eq!(iter.next(), Some(&"Java".to_string()));
        assert_eq!(iter.next(), Some(&"Python".to_string()));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_toy_vec_into_iter() {
        let mut toy_vec: ToyVec<String> = ToyVec::with_capacity(2);

        toy_vec.push("Java".to_string());
        toy_vec.push("Python".to_string());

        for value in toy_vec.into_iter() {
            println!("{}", value);
        }
    }

    #[test]
    fn test_toy_vec_debug_display() {
        let mut toy_vec: ToyVec<String> = ToyVec::with_capacity(2);

        toy_vec.push("Java".to_string());
        toy_vec.push("Python".to_string());

        println!("{:?}", toy_vec);
    }
}
