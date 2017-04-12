extern crate cons;

use cons::first;


pub fn main() {
    let mut list: first::List<i32> = first::List::new()
        .conj(1).conj(2).conj(3).conj(4);
    println!("list: {}", list);
    println!("pop: {:?}", list.pop());
    println!("pop: {:?}", list.pop());
    println!("pop: {:?}", list.pop());
    println!("pop: {:?}", list.pop());
    println!("list[debug]: {:?}", list);
    println!("list[displ]: {}", list);

    println!();
    let list = first::List::new()
        .conj(0).conj(1).conj(2).conj(3).conj(4);
    println!("reversed: {}", list);
    let list = list.reverse();
    println!("reversed: {}", list);

    println!("list.nth(2) -> {} == 2", list.nth(2).unwrap());
    println!("list.nth(20) -> {:?} == None", list.nth(20));
}
