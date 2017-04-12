/*!
Linked lists - first

*/
use std::mem;
use std::fmt;


#[derive(Debug)]
/// Cons linked list
pub enum List<T> {
    Cons(T, Box<List<T>>),
    Nil,
}
impl<T: fmt::Display> List<T> {
    pub fn new() -> List<T> {
        List::Nil
    }
    pub fn conj(self, head: T) -> List<T> {
        List::Cons(head, Box::new(self))
    }
    pub fn pop(&mut self) -> Option<T> {
        match mem::replace(self, List::Nil) {
            List::Nil => return None,
            List::Cons(v, more) => {
                *self = *more;
                Some(v)
            }
        }
    }
    pub fn reverse(mut self) -> List<T> {
        let mut list = List::Nil;
        while let Some(head) = self.pop() {
            list = list.conj(head);
        }
        list
    }
    pub fn nth(&self, n: usize) -> Option<&T> {
        match self {
            &List::Nil => None,
            &List::Cons(ref v, ref more) => {
                if n == 0 {
                    Some(v)
                } else {
                    more.nth(n-1)
                }
            }
        }
    }
    pub fn len(&self) -> usize {
        match self {
            &List::Nil => 0,
            &List::Cons(_, ref more) => {
                1 + more.len()
            }
        }
    }
    fn to_string_fmt(&self) -> String {
        match self {
            &List::Nil => "".to_string(),
            &List::Cons(ref v, ref more) => {
                let rem = more.to_string_fmt();
                if rem.is_empty() {
                    format!("{}", v)
                } else {
                    format!("{}, {}", v, rem)
                }
            }
        }
    }
    pub fn to_string(&self) -> String {
        let content = self.to_string_fmt();
        format!("[{}]", content)
    }
}


impl<T: fmt::Display> fmt::Display for List<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}


#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn constructs() {
        let list: List<i32> = List::new();
        assert_eq!(0, list.len());
        let list = list.conj(0).conj(1);
        assert_eq!(2, list.len());
    }

    #[test]
    fn formats() {
        let list = List::new()
            .conj(0).conj(1).conj(2).conj(3);
        assert_eq!("[3, 2, 1, 0]", list.to_string());
        assert_eq!(4, list.len());
    }

    #[test]
    fn pops() {
        let mut list = List::new()
            .conj("james1").conj("james2").conj("james3");
        let head = list.pop();
        assert_eq!("james3", head.unwrap());
        assert_eq!(2, list.len());
        list.pop();
        list.pop();
        let v = list.pop();
        assert!(v.is_none());
    }

    #[test]
    fn get_nth() {
        let list = List::new()
            .conj("james1").conj("james2").conj("james3");
        assert_eq!(Some(&"james2"), list.nth(1));
        assert_eq!(None, list.nth(100));
    }

    #[test]
    fn reversed() {
        let list = List::new()
            .conj(0).conj(1).conj(2).conj(3);
        let list = list.reverse();
        assert_eq!(Some(&2), list.nth(2));
        assert_eq!(4, list.len());
        assert_eq!("[0, 1, 2, 3]", list.to_string());
    }
}
