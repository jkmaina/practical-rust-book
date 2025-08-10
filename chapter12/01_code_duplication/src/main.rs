
fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];
    
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    
    largest
}
fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];
    
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    
    largest
}
fn main() {

    let numbers = vec![34, 50, 25, 100, 65];
    let result = largest_i32(&numbers);
    println!("The largest number is {}", result);
    
    let chars = vec!['y', 'm', 'a', 'q'];
    let result = largest_char(&chars);
    println!("The largest character is {}", result);

}
