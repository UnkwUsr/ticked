mod diff;
mod mode;
mod mytask;
mod parse;
mod req;
mod send;
mod ticktick_root_structs;

use crate::diff::get_diff;
use crate::mode::Mode;
use crate::mytask::MyTask;
use crate::parse::parse_file;
use crate::send::send;
use std::error::Error;
use std::io::Write;

const DEFAULT_MODE: Mode = Mode::Delete;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Receiving data...");
    let root = req::_req_get_root()?;
    // temp for developing
    // let root: ticktick_root_structs::Root;
    // {
    //     let a = std::fs::read("last.json")?;
    //     let asd = std::str::from_utf8(&a)?;
    //     root = serde_json::from_str(asd)?;
    // }

    let tmpfile = tempfile::Builder::new()
        .prefix("ticked-")
        .suffix(".md")
        .tempfile()
        .unwrap();

    let target_project_id = root.inbox_id;
    let tasks = root.sync_task_bean.update;

    let my_tasks: Vec<MyTask> = tasks
        .into_iter()
        .filter(|x| x.project_id == target_project_id)
        .map(|x| x.into())
        .collect();

    write_file(tmpfile.reopen().unwrap(), DEFAULT_MODE, my_tasks.iter());
    run_editor(tmpfile.path().to_str().unwrap());

    let (mode, new_my_tasks) = parse_file(tmpfile.reopen().unwrap(), DEFAULT_MODE);

    let diff_tasks = get_diff(&mode, my_tasks, new_my_tasks);
    if diff_tasks.is_empty() {
        println!("No diffs");
        return Ok(());
    }

    prompt_user(mode, &diff_tasks);

    // TODO: feat: ability to change mode after user see that he forget to change mode

    send(diff_tasks).unwrap();

    Ok(())
}

fn write_file(mut file: impl Write, mode: Mode, my_tasks: std::slice::Iter<MyTask>) {
    file.write(format!("# Mode: {}\n\n", mode).as_bytes())
        .unwrap();

    let res_file_content = my_tasks
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join("\n");

    file.write_all(res_file_content.as_bytes()).unwrap();
}

fn run_editor(path: &str) {
    std::process::Command::new(std::env::var("EDITOR").expect("EDITOR is not set"))
        .arg(path)
        .spawn()
        .expect("Error: Failed to run editor")
        .wait()
        .expect("Error: Editor returned a non-zero status");
}

fn prompt_user(mode: Mode, diff_tasks: &Vec<MyTask>) {
    println!(
        "\n{}",
        match mode {
            Mode::Delete => "To delete:",
            Mode::Edit => "Changed:",
        }
    );
    let asd: Vec<&str> = diff_tasks.iter().map(|x| x.title.as_ref()).collect();
    println!("* {}", asd.join("\n* "));
    println!("\nPress enter to apply");
    let mut asd = String::new();
    std::io::stdin().read_line(&mut asd).unwrap();
}
