use super::agent::CloudAgentPython;
use agent::error::Result;
use agent::modules::basic_message::{BasicMessageModule, SendBasicMessageOptions};
use async_trait::async_trait;
use serde_json::{json, Value};

#[async_trait]
impl BasicMessageModule for CloudAgentPython {
    async fn send_message(&self, options: SendBasicMessageOptions) -> Result<String> {
        let url = self.cloud_agent.create_url(vec![
            "connections",
            &options.connection_id,
            "send-message",
        ])?;

        let body = json!({
          "content": options.message,
        });

        self.cloud_agent
            .post::<Value>(url, None, Some(body))
            .await?;

        Ok(options.message)
    }
}
