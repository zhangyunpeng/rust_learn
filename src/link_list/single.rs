type Link<T> = Option<Box<Node<T>>>;

pub struct Node<T> {
    pub elem: T,
    pub next: Link<T>,
}

pub struct LinkedList<T> {
    pub head: Link<T>,
}

pub struct IterMut<'a, T:'a>(Option<&'a mut Node<T>>); 

impl<T> LinkedList<T> {
    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut(
            self.head.as_mut().map(|node|&mut **node)
        )
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.take().map(|node|{
            self.0 = node.next.as_mut().map(|node|&mut **node);
            &node.elem
        })
    }
}
