use crate::ticktick_root_structs::Root;
use curl::easy::Easy;
use std::env;
use std::error::Error;

const GET_URL: &str = "https://api.ticktick.com/api/v2/batch/check/0";

pub fn _req_get_root() -> Result<Root, Box<dyn Error>> {
    let mut data = Vec::new();

    {
        let mut handle = Easy::new();
        handle.url(GET_URL)?;
        handle.cookie(&format!(
            "t={}",
            env::var("TICKTICK_COOKIE").expect("TICKTICK_COOKIE is not set")
        ))?;
        handle.get(true)?;

        let mut transfer = handle.transfer();
        transfer.write_function(|chunk| {
            data.extend_from_slice(chunk);
            Ok(chunk.len())
        })?;
        transfer.perform()?;
    }

    let root: Root = serde_json::from_str(std::str::from_utf8(&data)?)?;

    Ok(root)
}
