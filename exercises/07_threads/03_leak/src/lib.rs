// TODO: Given a vector of integers, leak its heap allocation.
//  Then split the resulting static slice into two halves and
//  sum each half in a separate thread.
//  Hint: check out `Vec::leak`.
//use std::thread;
use std::thread;

pub fn sum(v: Vec<i32>) -> i32 {
     let v= v.leak();
     let handle1 = std::thread::spawn(|| {
        let midpoint = v.len() / 2;
        let first = &v[..midpoint];
        println!("Here's the first half of v: {first:?}");
        first.iter().sum::<i32>()
    });
    let handle2 = std::thread::spawn(|| {
        let midpoint = v.len() / 2;
        let second = &v[midpoint..];
        println!("Here's the second half of v: {second:?}");
        second.iter().sum::<i32>()
    });
    handle1.join().unwrap() + handle2.join().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(sum(vec![]), 0);
    }

    #[test]
    fn one() {
        assert_eq!(sum(vec![1]), 1);
    }

    #[test]
    fn five() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5]), 15);
    }

    #[test]
    fn nine() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]), 45);
    }

    #[test]
    fn ten() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 55);
    }
}
