use smart_pointers_1::*;

fn main() {
    let b: Box<i32> = Box::new(5);
    let linked_list: Node<String> = Node::new(String::from("Hello, world!"));

    println!("The value is {}", b);
    println!(
        "The value of the node in the linked list is {}",
        linked_list.value
    );
} // end main()
