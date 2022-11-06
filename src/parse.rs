use crate::{mytask::MyTask, Mode};
use pulldown_cmark::{Event, HeadingLevel, Tag};
use std::io::Read;

// TODO: drop pulldown library. My syntax is simple enough to parse it manually

pub fn parse_file(mut file: impl Read, default_mode: Mode) -> (Mode, Vec<MyTask>) {
    let mut buf = String::new();
    file.read_to_string(&mut buf).unwrap();

    let parser = pulldown_cmark::Parser::new(buf.as_str());

    let mut tasks = Vec::new();

    let mut mode = default_mode;
    let mut task: Option<MyTask> = None;

    let mut state_parsing_mode = false;
    let mut state_parsing_sublist = false;

    parser.for_each(|event| match event {
        Event::Start(Tag::Heading(HeadingLevel::H1, _, _)) => {
            state_parsing_mode = true;
        }
        Event::End(Tag::Heading(HeadingLevel::H1, _, _)) => {
            state_parsing_mode = false;
        }
        Event::Start(Tag::List(None)) => {
            if let Some(asd) = &mut task {
                if !state_parsing_sublist {
                    state_parsing_sublist = true;
                    asd.content = Some("".to_string());
                } else {
                    panic!("nested list is not supported");
                }
            }
        }
        Event::End(Tag::List(None)) => {
            if task.is_some() {
                state_parsing_sublist = false;
            }
        }
        Event::Start(Tag::Item) => {
            if state_parsing_sublist {
                return;
            }
            task = Some(MyTask::default());
        }
        Event::End(Tag::Item) => {
            if state_parsing_sublist {
                return;
            }
            let t = std::mem::replace(&mut task, Some(MyTask::default()));
            tasks.push(t.unwrap());
        }
        Event::Start(Tag::Paragraph) => {}
        Event::End(Tag::Paragraph) => {
            if state_parsing_sublist {
                // unwrap's guaranteed
                task.as_mut()
                    .unwrap()
                    .content
                    .as_mut()
                    .unwrap()
                    .push_str("\n\n");
            }
        }
        Event::Start(x) => todo!("start: {:?}", x),
        Event::End(x) => todo!("end: {:?}", x),

        Event::Text(text) => {
            if state_parsing_mode {
                match text.split_whitespace().last().unwrap() {
                    "d" => mode = Mode::Delete,
                    "e" => mode = Mode::Edit,
                    _ => {}
                };
                return;
            }

            if let Some(asd) = &mut task {
                if state_parsing_sublist {
                    if let Some(content) = &mut asd.content {
                        content.push_str(&text);
                    }
                    return;
                }

                if asd.title.is_empty() {
                    asd.title = text.to_string()
                }
            } else {
                // TODO: handle
            }
        }
        Event::SoftBreak => {
            if state_parsing_sublist {
                // unwrap's guaranteed
                task.as_mut().unwrap().content.as_mut().unwrap().push('\n');
            }
        }
        Event::Rule => {}

        Event::Code(_) => todo!(),
        Event::Html(_) => todo!(),
        Event::FootnoteReference(_) => todo!(),
        Event::HardBreak => todo!("hard"),
        Event::TaskListMarker(_) => todo!(),
    });

    (mode, tasks)
}
