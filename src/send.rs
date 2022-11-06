use crate::mytask::MyTask;

use curl::easy::{Easy, List};
use serde::Serialize;
use serde_with::skip_serializing_none;
use std::env;
use std::error::Error;
use std::io::Read;

const SEND_URL: &str = "https://api.ticktick.com/api/v2/batch/task";

#[skip_serializing_none]
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct RequestBody {
    // add: Option<Vec<MyTask>>,
    update: Option<Vec<MyTask>>,
    // delete: Option<Vec<MyTask>>,
    // add_attachments: Option<Vec<PreparedTask>>,
    // update_attachments: Option<Vec<PreparedTask>>,
    // delete_attachments: Option<Vec<PreparedTask>>,
}

impl From<Vec<MyTask>> for RequestBody {
    fn from(v: Vec<MyTask>) -> Self {
        RequestBody { update: Some(v) }
    }
}

pub fn send(tasks: Vec<MyTask>) -> Result<(), Box<dyn Error>> {
    let mut response_data = Vec::new();

    let request_body = &RequestBody::from(tasks);
    // println!(
    //     "request_body: {}",
    //     serde_json::to_string_pretty(request_body)?
    // );

    // this is to suppress error "temporary value dropped while borrowed"
    let request_body = serde_json::to_string(request_body)?;
    let mut request_body = request_body.as_bytes();

    {
        let mut list = List::new();
        list.append("content-type: application/json;charset=UTF-8")
            .unwrap();

        let mut handle = Easy::new();
        handle.url(SEND_URL)?;
        handle.cookie(&format!(
            "t={}",
            env::var("TICKTICK_COOKIE").expect("TICKTICK_COOKIE is not set")
        ))?;
        handle.http_headers(list).unwrap();
        handle.post(true)?;

        {
            let mut transfer = handle.transfer();
            transfer.write_function(|chunk| {
                response_data.extend_from_slice(chunk);
                Ok(chunk.len())
            })?;

            transfer.read_function(|into| Ok(request_body.read(into).unwrap()))?;

            transfer.perform()?;
        }

        println!("code {:?}", handle.response_code()?);
    }

    println!("{:?}", std::str::from_utf8(&response_data)?);

    Ok(())
}
