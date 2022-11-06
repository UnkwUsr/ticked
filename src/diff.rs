use crate::{mytask::MyTask, Mode};

pub fn get_diff(mode: &Mode, old_tasks: Vec<MyTask>, new_tasks: Vec<MyTask>) -> Vec<MyTask> {
    match mode {
        Mode::Delete => {
            let mut new_titles: Vec<String> = new_tasks.into_iter().map(|x| x.title).collect();
            let diff_tasks = old_tasks
                .into_iter()
                .filter(|old| {
                    let mut found = false;
                    let mut i = 0;
                    for x in &new_titles {
                        if x == &old.title {
                            found = true;
                            break;
                        }
                        i += 1;
                    }
                    if found {
                        new_titles.remove(i);
                    }
                    !found
                })
                .map(|mut x| {
                    x.status = 2;
                    x
                })
                .collect::<Vec<MyTask>>();

            diff_tasks
        }
        Mode::Edit => {
            let diff_tasks = std::iter::zip(old_tasks, new_tasks)
                .into_iter()
                .filter(|(x, y)| !eq_tasks(x, y))
                .map(|(old, new)| apply_task_changes(old, new))
                .collect();

            diff_tasks
        }
    }
}

fn eq_tasks(a: &MyTask, b: &MyTask) -> bool {
    a.title == b.title && a.content == b.content
}

fn apply_task_changes(mut old: MyTask, new: MyTask) -> MyTask {
    old.title = new.title;
    old.content = new.content;

    old
}
