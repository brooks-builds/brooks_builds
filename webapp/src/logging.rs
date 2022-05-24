use chrono::prelude::*;
use eyre::Result;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Debug)]
pub enum LogLevel {
    Info,
}

#[derive(Serialize)]
pub struct LogMessage {
    pub timestamp: DateTime<Local>,
    pub message: String,
    pub level: LogLevel,
}

impl LogMessage {
    pub fn new(message: impl Into<String>, level: LogLevel) -> Self {
        let now = Local::now();

        Self {
            timestamp: now,
            message: message.into(),
            level,
        }
    }

    pub fn console(&self) -> Result<()> {
        gloo::console::log!(serde_json::to_string_pretty(self)?);
        Ok(())
    }

    pub fn send(&self) -> Result<()> {
        let body = json!(
          {
            "@t":format!("{}", self.timestamp),
            "@m": self.message,
            "@l": format!("{:?}", self.level)
          }
        );
        let seq_key = env!("SEQ_KEY");
        wasm_bindgen_futures::spawn_local(async move {
            gloo::net::http::Request::post("http://localhost:5341/api/events/raw?clef")
                .header("X-Seq-ApiKey", seq_key)
                .body(serde_json::to_string(&body).unwrap())
                .send()
                .await
                .unwrap();
        });

        Ok(())
    }
}
