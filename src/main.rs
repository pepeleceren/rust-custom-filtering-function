fn main() {
    let list = vec![5, 6, 10, 15, 20];
    let condition = FilterCondition { val: 10 };

    let filtered_list = custom_filter(list, &condition);
    println!("{:?}", filtered_list);
}

struct FilterCondition {
    val: i32,
}

impl FilterCondition {
    fn is_matched(&self, item: i32) -> bool {
        item > self.val
    }
}

fn custom_filter(collection: Vec<i32>, filtercondition: &FilterCondition) -> Vec<i32> {
    let filtered_collection = collection
        .iter()
        .filter(|&n| filtercondition.is_matched(*n))
        .cloned()
        .collect::<Vec<i32>>();
    filtered_collection
}
