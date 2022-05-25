use chrono::prelude::*;
use serde_json::json;

const SEQ_KEY: &str = env!("SEQ_KEY");
const SEQ_URI: &str = "http://localhost:5341/api/events/raw?clef";

pub struct LogMessage;

impl LogMessage {
    pub fn info(message: impl Into<String> + Clone) {
        let body = json!(
          {
            "@t":format!("{}", Local::now()),
            "@m": message.clone().into(),
            "@l": "info"
          }
        );

        match serde_json::to_string(&body) {
            Ok(body) => Self::send(body),
            Err(error) => Self::console_error(
                "Error converting info log message to string",
                Some(format!("{:?}", error)),
            ),
        }

        if cfg!(debug_assertions) {
            Self::console_log(message.into())
        }
    }

    pub fn error(message: impl Into<String> + Clone, error: eyre::Report) {
        let stack_trace = format!("{:?}", error);
        let body = json!(
          {
            "@t":format!("{}", Local::now()),
            "@m": message.clone().into(),
            "@x": stack_trace.clone(),
            "@l": "error"
          }
        );

        match serde_json::to_string(&body) {
            Ok(body) => Self::send(body),
            Err(error) => Self::console_error(
                "Error converting error log message to string",
                Some(format!("{:?}", error)),
            ),
        }

        if cfg!(debug_assertions) {
            Self::console_error(message.into(), Some(stack_trace));
        }
    }

    fn send(body: String) {
        wasm_bindgen_futures::spawn_local(async move {
            match gloo::net::http::Request::post(SEQ_URI)
                .header("X-Seq-ApiKey", SEQ_KEY)
                .body(body)
                .send()
                .await
            {
                Ok(result) => {
                    if !result.ok() {
                        Self::console_error("Error sending log", None);
                    }
                }
                Err(error) => Self::console_error(
                    "Error sending log, please check network tab to find what happened",
                    Some(format!("{:?}", error)),
                ),
            }
        });
    }

    fn console_error(message: impl Into<String>, stack_trace: Option<String>) {
        gloo::console::error!(message.into(), stack_trace);
    }

    fn console_log(message: impl Into<String>) {
        gloo::console::log!(message.into());
    }
}
