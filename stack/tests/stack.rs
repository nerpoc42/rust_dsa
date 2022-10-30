use stack::*;

#[test]
fn push_pop() {
    let mut stack = Stack::new();

    stack.push(10);
    stack.push(15);
    stack.push(11);

    assert_eq!(stack.pop(), Some(11));
    assert_eq!(stack.pop(), Some(15));
    assert_eq!(stack.pop(), Some(10));
    assert_eq!(stack.pop(), None);
    assert_eq!(stack.pop(), None);
}

#[test]
fn peek() {
    let mut stack = Stack::new();

    assert_eq!(stack.peek(), None);
    assert_eq!(stack.peek_mut(), None);

    stack.push(10);
    assert_eq!(stack.peek(), Some(&10));
    assert_eq!(stack.peek_mut(), Some(&mut 10));

    stack.push(15);
    assert_eq!(stack.peek(), Some(&15));
    assert_eq!(stack.peek_mut(), Some(&mut 15));

    stack.push(11);
    assert_eq!(stack.peek(), Some(&11));
    assert_eq!(stack.peek_mut(), Some(&mut 11));
}

#[test]
fn nth() {
    let mut stack = Stack::new();

    assert_eq!(stack.nth(0), None);
    assert_eq!(stack.nth_mut(0), None);

    stack.push(10);
    stack.push(15);
    stack.push(11);

    assert_eq!(stack.nth(0), Some(&11));
    assert_eq!(stack.nth(1), Some(&15));
    assert_eq!(stack.nth(2), Some(&10));
    assert_eq!(stack.nth_mut(0), Some(&mut 11));
    assert_eq!(stack.nth_mut(1), Some(&mut 15));
    assert_eq!(stack.nth_mut(2), Some(&mut 10));

    assert_eq!(stack.nth(3), None);
    assert_eq!(stack.nth_mut(4), None);
}

#[test]
fn remove() {
    let mut stack = Stack::new();

    assert_eq!(stack.remove(0), None);

    stack.push(10);
    stack.push(15);
    stack.push(11);

    assert_eq!(stack.remove(1), Some(15));
    assert_eq!(stack.remove(1), Some(10));
    assert_eq!(stack.remove(1), None);
    assert_eq!(stack.remove(0), Some(11));
    assert_eq!(stack.remove(0), None);

    assert_eq!(stack.peek(), None);
}

#[test]
fn contains() {
    let mut stack = Stack::new();

    assert!(!stack.contains(&0));

    stack.push(10);

    assert!(!stack.contains(&0));
    assert!(stack.contains(&10));

    stack.pop();

    assert!(!stack.contains(&10));

    stack.push(15);
    stack.push(11);

    assert!(!stack.contains(&10));
    assert!(stack.contains(&15));
    assert!(stack.contains(&11));

    stack.pop();
    stack.pop();

    assert!(!stack.contains(&15));
    assert!(!stack.contains(&11));
}
