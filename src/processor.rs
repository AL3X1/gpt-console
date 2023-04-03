use std::io;
use std::io::Write;
use termion::color;
use crate::models::gpt::{GPTError, GPTMessage, GPTRequest, GPTResponse};

const TOKEN: &str = "Bearer "; // the GPT token should be here
const REQUEST_URL: &str = "https://api.openai.com/v1/chat/completions";
const GPT_MODEL: &str = "gpt-3.5-turbo";
const CONTENT_TYPE: &str = "application/json";
const ROLE: &str = "assistant";
const STOP_CMD: &str = "exit";

pub struct QueryProcessor {
    debug_mode: bool,
}

impl QueryProcessor {
    pub fn new(debug_mode: bool) -> Self {
        QueryProcessor {
            debug_mode,
        }
    }

    pub async fn start(&self) {
        println!("GPT session started. Ask your question.");

        loop {
            let mut input = String::new();
            print!("{}> ", color::Fg(color::Cyan));

            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut input).unwrap();

            if input.trim().len() == 0 {
                continue;
            }

            if input.trim() == STOP_CMD {
                return;
            }
            
            let model = GPTRequest {
                model: GPT_MODEL.to_string(),
                messages: Vec::from([
                    GPTMessage {
                        role: ROLE.to_string(),
                        content: input.to_string(),
                    }
                ]),
            };

            let response_result = QueryProcessor::process(&model).await;
            
            if response_result.is_err() {
                continue;
            }
            
            let response = response_result.unwrap();
            println!("{}{}", color::Fg(color::Green), response.choices[0].message.content);

            if self.debug_mode {
                println!("Sent: {}", serde_json::to_string(&model).unwrap());
                println!("Received: {}", serde_json::to_string(&response).unwrap());
            }
        }
    }

    async fn process(query: &GPTRequest) -> serde_json::Result<GPTResponse> {
        println!("{}Loading...", color::Fg(color::LightGreen));
        let json = serde_json::to_string(&query).unwrap();

        let client = reqwest::Client::new();
        let request = client
            .post(REQUEST_URL)
            .header("Authorization", TOKEN)
            .header("Content-Type", CONTENT_TYPE)
            .body(json)
            .send();

        let response_string = request
            .await
            .unwrap()
            .text()
            .await
            .unwrap();
        
        let response = serde_json::from_str(&response_string);
        if response.is_err() {
            let error: GPTError = serde_json::from_str(&response_string).unwrap();
            println!("{}Unable to process query: {}", color::Fg(color::Red), error.error.message);
        }
        
        return response;
    }
}