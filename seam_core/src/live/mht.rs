use crate::{
    common::CLIENT,
    error::{Result, SeamError},
    util::parse_url,
};

use async_trait::async_trait;
use serde_json::Value;

use super::{Live, Node};

const URL: &str = "https://www.2cq.com/proxy/room/room/info";

/// 棉花糖直播
///
/// https://www.2cq.com/
pub struct Client;

// TODO 似乎某些房间有额外的 flv 地址
#[async_trait]
impl Live for Client {
    async fn get(rid: &str) -> Result<Node> {
        let resp: serde_json::Value = CLIENT
            .get(URL)
            .query(&[("roomId", rid), ("appId", "1004")])
            .send()
            .await?
            .json()
            .await?;
        match &resp["errorMsg"] {
            Value::Null => {
                // 不报错的情况必然有结果返回 直接提取
                let result = &resp["result"];
                match result["liveState"].to_string().parse::<usize>()? {
                    // 开播状态
                    1 => {
                        let urls = vec![parse_url(result["pullUrl"].as_str().unwrap().to_owned())];
                        Ok(Node {
                            rid: rid.to_owned(),
                            title: result["roomName"].as_str().unwrap().to_owned(),
                            urls,
                        })
                    }
                    _ => Err(SeamError::None),
                }
            }
            // 房间不存在或其他错误
            msg => Err(SeamError::Unknown(msg.to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_url() {
        match Client::get("911038").await {
            Ok(node) => println!("{}", node.json()),
            Err(e) => println!("{e}"),
        }
    }
}
