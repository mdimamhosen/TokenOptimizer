/// Token counter for different AI providers
/// Uses statistical estimation for models without known tokenizers

pub struct TokenCounter;

impl TokenCounter {
    /// Estimate tokens for text (rule of thumb: ~4 chars = 1 token)
    pub fn estimate_tokens(text: &str) -> i64 {
        let words = text.split_whitespace().count() as i64;
        (words + words / 3).max(1)
    }

    /// Calculate tokens for prompt and completion
    pub fn calculate_tokens(prompt: &str, completion: &str) -> (i64, i64) {
        let prompt_tokens = Self::estimate_tokens(prompt);
        let completion_tokens = Self::estimate_tokens(completion);
        (prompt_tokens, completion_tokens)
    }

    /// Estimate cost for Claude models (USD)
    pub fn estimate_claude_cost(prompt_tokens: i64, completion_tokens: i64, model: &str) -> f64 {
        let (prompt_cost_per_mtok, completion_cost_per_mtok) = match model {
            "claude-3-5-sonnet" => (0.003, 0.015),
            "claude-3-opus" => (0.015, 0.075),
            "claude-3-haiku" => (0.00025, 0.00125),
            _ => (0.003, 0.015),
        };

        ((prompt_tokens as f64 * prompt_cost_per_mtok) +
         (completion_tokens as f64 * completion_cost_per_mtok)) / 1_000_000.0
    }

    /// Estimate cost for GPT models (USD)
    pub fn estimate_gpt_cost(prompt_tokens: i64, completion_tokens: i64, model: &str) -> f64 {
        let (prompt_cost_per_1k, completion_cost_per_1k) = match model {
            "gpt-4" => (0.03, 0.06),
            "gpt-4-turbo" => (0.01, 0.03),
            "gpt-3.5-turbo" => (0.0005, 0.0015),
            _ => (0.01, 0.03),
        };

        ((prompt_tokens as f64 * prompt_cost_per_1k) +
         (completion_tokens as f64 * completion_cost_per_1k)) / 1_000.0
    }

    /// Estimate cost for any model
    pub fn estimate_cost(prompt_tokens: i64, completion_tokens: i64, provider: &str, model: &str) -> f64 {
        match provider {
            "anthropic" => Self::estimate_claude_cost(prompt_tokens, completion_tokens, model),
            "openai" => Self::estimate_gpt_cost(prompt_tokens, completion_tokens, model),
            _ => {
                let avg_cost_per_token = 0.000005;
                (prompt_tokens + completion_tokens) as f64 * avg_cost_per_token
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_estimate_tokens() {
        let text = "Hello world this is a test";
        let tokens = TokenCounter::estimate_tokens(text);
        assert!(tokens > 0);
    }

    #[test]
    fn test_claude_cost() {
        let cost = TokenCounter::estimate_claude_cost(1000, 500, "claude-3-5-sonnet");
        assert!(cost > 0.0);
    }
}
