fn main() {
    let mut my_fav_fruits = vec!["banana", "custard apple", "avocado", "peach", "raspberry"];

    let mut my_iterable_fav_fruits = my_fav_fruits.iter();   // Step 1: Use `iter_mut` to create a mutable iterator.

    assert_eq!(my_iterable_fav_fruits.next(), Some(& "banana"));    // Step 2: Use `next()` to retrieve the next element in the iterator.
    assert_eq!(my_iterable_fav_fruits.next(), Some(& "custard apple"));
    assert_eq!(my_iterable_fav_fruits.next(), Some(& "avocado"));
    assert_eq!(my_iterable_fav_fruits.next(), Some(& "peach"));
   // let new_element = "sss";
    //my_fav_fruits.push(new_element);
 assert_eq!(my_iterable_fav_fruits.next(), Some(& "raspberry"));
assert_eq!(my_iterable_fav_fruits.next(), None);

}
