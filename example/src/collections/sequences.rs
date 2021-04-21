pub fn it_works() {
    vec::it_works();
    deque::it_works();
    linkedlist::it_works();
}

mod vec {

    pub fn it_works() {
        marco_vec();
        index_vec();
        slice_vec();
    }
    fn marco_vec() {
        let mut vec = Vec::new();
        vec.push(1);
        println!("{:?}", vec);
        let mut vec = vec![1];
        vec.push(2);
        println!("{:?}", vec);

        let mut vec = vec![1, 2];
        vec.extend([3, 4, 5].iter().copied());
        println!("{:?}", vec);

        let mut vec = Vec::with_capacity(5);
        vec.resize(5, "ooo");
        println!("{:?}", &vec);
        while let Some(top) = vec.pop() {
            println!("{}", top);
        }
    }

    fn index_vec() {
        //capacity 2^n  default 4
        let mut vec: Vec<i32> = Vec::new();
        vec.append(&mut vec![0i32, 2, 4]);
        println!(
            "vec = {:?}; vec[2] = {}, capacity = {}, len = {}",
            &vec,
            vec[2],
            vec.capacity(),
            vec.len()
        );

        let mut vec: Vec<i32> = Vec::with_capacity(5);
        vec.append(&mut vec![44i32]);
        println!(
            "vec = {:?}, capacity = {}, len = {}",
            &vec,
            vec.capacity(),
            vec.len()
        );

        let vec = vec![1, 2];
        println!(
            "vec {:?},capacity = {}, len = {}",
            vec,
            vec.capacity(),
            vec.len()
        );
        let mut _vec = vec![1, 2];
        println!(
            "mut vec {:?}, capacity = {}, len = {}",
            _vec,
            _vec.capacity(),
            _vec.len()
        );
    }

    fn slice_vec() {
        let mut v = vec![];
        v.resize_with(6, || "ooo");
        println!(
            "&v = {:?};\n&v[0..2] = {:?};\n&v[0..=2] = {:?};\n&v[..] = {:?}",
            &v,
            &v[0..2],
            &v[0..=2],
            &v[..]
        );
    }
}

use std::collections::VecDeque;
mod deque {
    use super::VecDeque;
    pub fn it_works() {
        // capacity 2^n - 1  default 7
        // let mut vector: VecDeque<i32> = VecDeque::with_capacity(8);
        let mut vector: VecDeque<i32> = VecDeque::new();
        println!(
            "vector capacity = {}, len = {}",
            vector.capacity(),
            vector.len()
        );
        vector.extend(vec![1, 2, 3]);
        println!(
            "vector capacity = {}, len = {}",
            vector.capacity(),
            vector.len()
        );
        println!("vector = {:?}", &vector);
        vector.push_front(0);
        vector.push_back(4);
        println!("vector = {:?}", &vector);
        if let Some(e) = vector.get_mut(0) {
            *e = -1;
            println!("vector = {:?}", &vector);
        }
        vector.swap(0, 4);
        println!("vector = {:?}", &vector);
        vector.swap_remove_back(0);
        println!("vector = {:?}", &vector);
        vector.swap_remove_front(3);
        println!("vector = {:?}", &vector);
        println!(
            "vector capacity = {}, len = {}",
            vector.capacity(),
            vector.len()
        );
        vector.insert(0, 998);
        println!("afetr vector.insert(0, 998), vector = {:?}", &vector);

        let mut deque: VecDeque<i32> = VecDeque::new();
        deque.extend([1, 2, 3, 4, 5, 6, 7].iter());
        println!("deque = {:?}", &deque);
        deque.rotate_left(3);
        println!("afetr deque.rotate_left(3), deque = {:?}", &deque);
        deque.rotate_right(3);
        println!("afetr deque.rotate_right(3), deque = {:?}", &deque);
    }
}

mod linkedlist {
    use std::collections::LinkedList;
    pub fn it_works() {
        let ll: LinkedList<i32> = LinkedList::default();
        println!("{:?}, len = {}", &ll, ll.len());
    }
}
