use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::error::Error;
use actix_web::HttpResponse;
use crate::dotenv_tools::{read, API_ENDPOINT, ROCK_ENDPOINT, ROCK_TOKEN};
use crate::networking;
use crate::qsegment_constructor::{QSegmentConstructor, Types};

#[derive(Debug, Deserialize)]
struct Sender {
    user_id: u64,
    nickname: String,
    card: String,
}
#[derive(Debug, Deserialize)]
pub struct Message {
    message_type: String,
    raw_message: String,
    sender: Sender,
    group_id: Option<u64>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct QMessage {
    pub(crate) group_id: u64,
    pub(crate) message: Value,
    pub(crate) auto_escape: bool,
}

#[derive(Debug, Serialize)]
struct Credential {
    message: String,
    from: u32,
    token: String,
}

pub struct Backend;

impl Backend {
    pub async fn handle_input(req_body: String) -> HttpResponse {
        let backend= Backend;
        match Backend::director(&backend, &req_body).await {
            Ok(_) => HttpResponse::Ok().finish(),
            Err(_) => HttpResponse::InternalServerError().finish(),
        }
    }


    pub async fn director(&self, content: &str) -> Result<(), Box<dyn Error>> {
        let json_object: Message = serde_json::from_str(content)?;
        let message_type = &json_object.message_type;
        let user_id = json_object.sender.user_id;
        let msg = &json_object.raw_message;
        let name = &json_object.sender.nickname;
        let group_name = &json_object.sender.card;
        println!("{} 消息： {} 发送了 {}", message_type, user_id, msg);

        if message_type == "group" && json_object.group_id == Some(946085440) {
            match msg.as_str() {
                "/stat" => {
                    let status_object = self.get_server_status().await?;
                    if status_object["code"] == 0 {
                        networking::send_message(&format!(
                            "服务器当前mspt: {}\n服务器当前在线人数: {}",
                            status_object["mspt_3s"], status_object["onlinecount"]
                        )).await?;
                    } else {
                        networking::send_message("服务器当前不在线").await?;
                    }
                }
                "喵喵喵？" => {
                    networking::send_message("喵喵，喵 >_<，我是主人的星怒喵").await?;
                }
                "@Bot 你是谁？" => {
                    networking::send_message("Working with Rust and actix.").await.expect("Panic");
                }
                msg if msg.starts_with("-testconnection") => {
                    self.handle_connection_test(msg).await?;
                }
                msg if msg.starts_with("/approve-register") => {
                    self.handle_approve_register(msg, user_id).await?;
                }
                _ => {
                    let message = self.generate_credential(&format!(
                        "<{}|{}>:{}",
                        if group_name.is_empty() { name } else { group_name },
                        user_id,
                        msg
                    ));
                    self.qclient(message).await?;
                }
            }
        }

        Ok(())
    }

    async fn get_server_status(&self) -> Result<Value, Box<dyn Error>> {
        let status_url = format!("{}/qo/download/status", *API_ENDPOINT);
        networking::send_get_request(&status_url).await
    }

    async fn handle_connection_test(&self, msg: &str) -> Result<(), Box<dyn Error>> {
        let msg_seg: Vec<&str> = msg.split_whitespace().collect();
        if msg_seg.len() == 2 {
            let mut url = msg_seg[1].to_string();
            if !url.starts_with("http://") && !url.starts_with("https://") {
                networking::send_message("请添加https://或者http://前缀，默认使用http请求。").await?;
                url = format!("http://{}", url);
            }
            match networking::send_get_request(&url).await {
                Ok(_) => networking::send_message("成功！").await?,
                Err(_) => networking::send_message("指定的地址连通性测试：失败！").await?,
            }
        }
        Ok(())
    }

    async fn handle_approve_register(&self, msg: &str, user_id: u64) -> Result<(), Box<dyn Error>> {
        let msg_seg: Vec<&str> = msg.split_whitespace().collect();
        if msg_seg.len() == 2 {
            let confirm_url = format!(
                "http://{}/qo/upload/confirmation?token={}&uid={}",
                *API_ENDPOINT, msg_seg[1], user_id
            );
            let result: Value = networking::send_get_request(&confirm_url).await?;
            if result["result"].as_bool().unwrap_or(false) {
                networking::send_message("验证成功").await?;
            } else {
                networking::send_message("验证失败，可能qq号不正确").await?;
            }
        }
        Ok(())
    }



    fn generate_credential(&self, message: &str) -> String {
        let credential = Credential {
            message: message.to_string(),
            from: 0,
            token: "asfieruvnz204@@#vfjne".to_string(),
        };
        serde_json::to_string(&credential).unwrap_or_default()
    }

    async fn qclient(&self, message: String) -> Result<(), Box<dyn Error>> {
        let client = Client::new();
        let response = client
            .post(&format!("http://{}/qo/msglist/upload", *API_ENDPOINT))
            .body(message)
            .send()
            .await?;
        println!("{}", response.text().await?);
        Ok(())
    }
}
