use serde::{Serialize, Deserialize};
use serde_json::{json, Value};

#[derive(Debug, Serialize, Deserialize)]
pub enum Types {
    Plain,
    Img,
    Face,
    Record,
    Video,
    At,
    Share,
    Reply,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MsgSeg {
    pub r#type: Types,
    pub msg: String,
}

pub struct QSegmentConstructor;

impl QSegmentConstructor {
    pub fn create(type_: Types, msg: &str) -> MsgSeg {
        MsgSeg {
            r#type: type_,
            msg: msg.to_string(),
        }
    }

    pub fn factory(msgs: Vec<MsgSeg>) -> Value {
        let mut msg_arr = Vec::new();

        for msg in msgs {
            let msg_obj = Self::create_msg_obj(&msg.r#type);
            let data_obj = Self::create_data_obj(&msg.r#type, &msg.msg);

            let mut msg_obj = msg_obj.as_object().unwrap().clone();
            msg_obj.insert("data".to_string(), data_obj);

            msg_arr.push(Value::Object(msg_obj));
        }

        Value::Array(msg_arr)
    }

    fn create_msg_obj(type_: &Types) -> Value {
        let type_str = match type_ {
            Types::Plain => "text",
            Types::Img => "image",
            Types::Face => "face",
            Types::Record => "record",
            Types::Video => "video",
            Types::At => "at",
            Types::Share => "share",
            Types::Reply => "reply",
        };

        json!({
            "type": type_str
        })
    }

    fn create_data_obj(type_: &Types, msg: &str) -> Value {
        let data_field_name = match type_ {
            Types::Plain => "text",
            Types::Img => "file",
            Types::Face => "id",
            Types::Record => "file",
            Types::Video => "file",
            Types::At => "qq",
            Types::Share => "url",
            Types::Reply => "id",
        };

        json!({
            data_field_name: msg
        })
    }
}
