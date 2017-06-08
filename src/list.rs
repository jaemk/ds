pub struct List<T> {
    size: usize,
    head: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

pub struct Node<T> {
    elem: T,
    link: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> List<T> {
        List { size: 0, head: None }
    }

    pub fn from_vec(items: Vec<T>) -> List<T> {
        let mut list = Self::new();
        for item in items.into_iter() {
            list.push(item);
        }
        list
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn push(&mut self, elem: T) {
        let new_head = Node { elem: elem, link: self.head.take() };
        self.head = Some(Box::new(new_head));
        self.size += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|mut head| {
            let next = head.link.take();
            self.head = next;
            self.size -= 1;
            head.elem
        })
    }
    pub fn into_iter(self) -> IntoIterList<T> {
        IntoIterList::new(self.head)
    }
}

pub struct IntoIterList<T> {
    head: Link<T>,
}
impl<T> IntoIterList<T> {
    pub fn new(link: Link<T>) -> IntoIterList<T> {
        IntoIterList { head: link }
    }
}
impl<T> Iterator for IntoIterList<T> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        self.head.take().map(|head| {
            let head = *head;
            self.head = head.link;
            head.elem
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_create() {
        let list: List<i32> = List::new();
        assert!(list.len() == 0);
    }

    #[test]
    fn test_push() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        assert_eq!(vec![2, 1], list.into_iter().collect::<Vec<_>>());

        let mut list = List::from_vec(vec!["a", "b", "c"]);
        list.push("james");
        assert_eq!(vec!["james", "c", "b", "a"], list.into_iter().collect::<Vec<_>>());
    }

    #[test]
    fn test_pop() {
        let mut list = List::from_vec(vec!["a", "b", "c"]);
        let c = list.pop().unwrap();
        assert_eq!("c", c);
        assert_eq!(vec!["b", "a"], list.into_iter().collect::<Vec<_>>());
    }
}
