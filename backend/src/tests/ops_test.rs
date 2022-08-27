
#[cfg(test)]
mod tests {
    use crate::database::models::*;
    use crate::ops::object_ops::*;
    use crate::ops::statistic_ops::*;
    use crate::ops::user_ops::*;
    #[test]
    fn create_show_del_user() {
        let test1: NewUser = NewUser { username: "John" };
        let res = create_user(test1);
        let user = show_by_user_id(res.unwrap()).unwrap();
        delete_user(user.id); //if it doesnt panic, it works
    }
}
