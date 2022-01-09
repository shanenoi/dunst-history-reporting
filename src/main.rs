use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::process::{Command, Stdio};

#[derive(Serialize, Deserialize, Debug)]
struct ValueString {
    r#type: String,
    data: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ValueNum {
    r#type: String,
    data: i64,
}

#[derive(Serialize, Deserialize, Debug)]
struct Item {
    body: ValueString,
    message: ValueString,
    summary: ValueString,
    appname: ValueString,
    category: ValueString,
    default_action_name: ValueString,
    icon_path: ValueString,
    id: ValueNum,
    timestamp: ValueNum,
    timeout: ValueNum,
    progress: ValueNum,
}

#[derive(Serialize, Deserialize, Debug)]
struct History {
    r#type: String,
    data: Vec<Vec<Item>>,
}

fn main() {
    let output = Command::new("dunstctl")
        .arg("history")
        .stdout(Stdio::piped())
        .output()
        .unwrap();

    let stdout = String::from_utf8(output.stdout).unwrap();

    let history_struct: Result<History> = serde_json::from_str(&stdout[..]);
    match history_struct {
        Ok(value) => {
            for item in &value.data {
                for mess in item {
                    println!("{:?}", mess.message.data);
                }
            }
        },
        Err(err) => println!("something weird is going on: {:?}", err)
    }
}
