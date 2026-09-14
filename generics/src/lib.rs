// TODO: Define a generic struct with a single field `item: T`.
pub struct ItemContainer<T> {
    // Add the field here
    pub item: T,
}

// "For any type T, here's an implementation for ItemContainer<T>."
impl<T> ItemContainer<T> {
    // TODO: Implement the `get_item` method to return a reference to the item.
    //A type parameter is specified as generic by the use of angle brackets and upper camel case: <Aaa, Bbb, ...>.
    // “Generic type parameters” are typically represented as <T>.
    pub fn get_item(&self) -> &T {
        &self.item
    }
}

// Example usage
pub fn main() {
    let item_1 = ItemContainer { item: 42 };
    assert_eq!(*item_1.get_item(), 42);

    let item_2 = ItemContainer {
        item: String::from("Hello"),
    };

    assert_eq!(item_2.get_item(), "Hello");
}
