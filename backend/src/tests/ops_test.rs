#[cfg(test)]
mod tests {
    use diesel::sql_query;
    use diesel::RunQueryDsl;
    use serial_test::serial;

    use crate::api::statistic_controller::get_all_statistics;
    use crate::database::db::establish_connection;
    use crate::database::models::*;
    use crate::ops::object_ops::*;
    use crate::ops::statistic_ops::*;
    use crate::ops::user_ops::*;
    use diesel::sql_types::Integer;

    #[test]
    #[serial]
    fn create_show_del_user() {
        let test1: NewUser = NewUser { username: "Peter" };
        let res = create_user(test1);
        let user = show_by_user_id(res.unwrap()).unwrap();
        delete_user(user.id); //if it doesnt panic, it works
    }
    #[test]
    #[serial]
    fn statistic_ops() {
        let connection = establish_connection();
        let last_stat: Vec<Statistic> = sql_query(format!(
            "SELECT * FROM statistics ORDER BY id DESC LIMIT 1;"
        ))
        .load(&connection)
        .expect("SQL query error");
        let last_id = last_stat[0].id;
        let test2: NewStatistic = NewStatistic {
            name: ("kitchen"),
            user_id: (15),
        };
        create_statistic(test2);
        let mut res = show_statistic_by_id(last_id + 1);
        res.name = String::from("shoes");
        update_statistic(res);
        let res2 = show_statistic_by_id(last_id + 1);
        assert_eq!(res2.name, "shoes");
        delete_statistic(last_id + 1);
    }
    #[test]
    #[serial]
    fn object_ops() {
        let connection = establish_connection();
        let last_obj: Vec<Object> =
            sql_query(format!("SELECT * FROM objects ORDER BY id DESC LIMIT 1;"))
                .load(&connection)
                .expect("SQL query error");
        let last_id = last_obj[0].id;
        let test3: NewObject = NewObject {
            name: "red book",
            statistic_id: 15,
            counter: 4,
        };
        create_object(test3);
        let res = show_object_by_id(last_id + 1);
        assert_eq!(res.name, "red book");
        // delete_object(2);
        update_counter(last_id + 1, 24);
        let res = show_object_by_id(last_id + 1);
        assert_eq!(res.counter, 24);
        update_object(Object {
            id: last_id + 1,
            name: String::from("yellow book"),
            statistic_id: 15,
            counter: 54,
        });
        let res = show_object_by_id(last_id + 1);
        assert_eq!(res.name, "yellow book");
        assert_eq!(delete_object(last_id + 1), true);
    }
}
