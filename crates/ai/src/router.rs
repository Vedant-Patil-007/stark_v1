use crate::action::AiAction;
use crate::error::{AiError, Result};
use crate::parser;
use crate::provider::{AiProvider, CommandContext, ProviderResponse};

/// Which tier produced an action.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tier {
    /// Local regex parser. No network, no cost, instant.
    Deterministic,
    /// Cloud provider.
    Remote,
}

#[derive(Debug, Clone)]
pub struct RoutedAction {
    pub action: AiAction,
    pub tier: Tier,
    pub raw: Option<String>,
    pub model: Option<String>,
    pub latency_ms: u64,
}

/// Try the deterministic parser first; escalate only if it doesn't match.
///
/// This ordering is the whole point: common commands cost nothing and work
/// offline. Only genuinely open-ended requests reach the network.
pub async fn route(
    ctx: &CommandContext,
    provider: Option<&dyn AiProvider>,
) -> Result<RoutedAction> {
    if let Some(action) = parser::parse(&ctx.instruction, &ctx.today) {
        return Ok(RoutedAction {
            action,
            tier: Tier::Deterministic,
            raw: None,
            model: None,
            latency_ms: 0,
        });
    }

    let provider = provider.ok_or_else(|| {
        AiError::Unavailable(
            "no AI provider configured; add an API key in Settings".into(),
        )
    })?;

    let ProviderResponse { action, raw, model, latency_ms } = provider.interpret(ctx).await?;

    Ok(RoutedAction {
        action,
        tier: Tier::Remote,
        raw: Some(raw),
        model: Some(model),
        latency_ms,
    })
}