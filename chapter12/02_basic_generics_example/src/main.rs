// A function that finds the largest element in a list using generics
// The type T must implement the PartialOrd trait to allow comparison
// The function returns a reference to the largest element in the list

fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    
    largest
}
fn main() {
    let numbers = vec![34, 50, 25, 100, 65];
    let result = largest(&numbers);
    println!("The largest number is {}", result);
    
    let chars = vec!['y', 'm', 'a', 'q'];
    let result = largest(&chars);
    println!("The largest character is {}", result);
}
