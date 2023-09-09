mod list {
    pub struct Tasks {
        pub item: String,
    }
}

mod things_todo;
use crate::things_todo::add_activity;
use things_todo::items_completed;

fn lets_add_Task() {
    let task = list::Tasks {
        item: String::from("tasks"),
    };
    add_activity();
    items_completed::remove_task();
}
