
/// This function checks if the subscription string of
/// an user already contains id of user to subscribe.
/// It returns this as boolean value
pub fn check_duplicates(subscriptions: &String, new: i32) -> bool {
    let arr: Vec<&str> = subscriptions.split(" ").collect();
    for el in arr {
        if el.parse::<i32>().is_err() {
            return false;
        }
        if el.parse::<i32>().unwrap() == new {
            return true;
        }
    }
    return false;
}