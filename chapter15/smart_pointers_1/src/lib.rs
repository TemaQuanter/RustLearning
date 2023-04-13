/// This file contains an implementation of
/// singly linked lists.

/// This structure is a single node of a singly
/// linked list.
pub struct Node<T> {
    pub value: T,
    pub left: Option<Box<Node<T>>>,
    pub right: Option<Box<Node<T>>>,
} // end Node

impl<T> Node<T> {
    /// This function initializes the node in a linked list.
    ///
    /// # Example
    ///
    /// ```
    /// use smart_pointers_1::Node;
    ///
    /// fn main() {
    ///     let linked_list: Node<String> = Node::new("Hello, world".to_string());
    ///
    ///     assert_eq!(linked_list.value, "Hello, world".to_string());
    /// }
    /// ```
    pub fn new(value: T) -> Node<T> {
        Node {
            value,
            left: None,
            right: None,
        } // end new_node
    } // end new()

    /// This function adds a new value to the end
    /// of the linked list.
    ///
    pub fn push(&mut self, value: T) {

    } // end push()
} // end impl Node

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initialization_works() {
        let linked_list: Node<i32> = Node::new(5);

        assert_eq!(linked_list.value, 5);

        match linked_list.left {
            Some(_) => panic!("Left node is initialized incorreclty"),
            None => (),
        } // end match

        match linked_list.right {
            Some(_) => panic!("Right node is initialized incorreclty"),
            None => (),
        } // end match
    } // end initailization_works()

    #[test]
    fn linked_list_generates_correctly() {

    } // end linked_list_generates_correclty()
} // end mod tests
