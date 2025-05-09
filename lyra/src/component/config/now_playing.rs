use lyra_ext::num::u64_to_i64_truncating;
use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::{
    command::{SlashCtx, macros::out, model::BotSlashCommand, require},
    core::model::DatabaseAware,
    error::CommandResult,
    gateway::GuildIdAware,
};
use lyra_proc::BotCommandGroup;

#[derive(CommandModel, CreateCommand, BotCommandGroup)]
#[command(name = "now-playing", desc = ".")]
pub enum NowPlaying {
    #[command(name = "toggle")]
    Toggle(Toggle),
}

/// Toggles whether now-playing track messages should be automatically sent or not
#[derive(CommandModel, CreateCommand)]
#[command(name = "toggle")]
pub struct Toggle;

impl BotSlashCommand for Toggle {
    async fn run(self, ctx: SlashCtx) -> CommandResult {
        let mut ctx = require::guild(ctx)?;
        let new_now_playing = sqlx::query!(
            "UPDATE guild_configs SET now_playing = NOT now_playing WHERE id = $1 RETURNING now_playing;",
            u64_to_i64_truncating(ctx.guild_id().get()),
        )
        .fetch_one(ctx.db())
        .await?
        .now_playing;

        let (emoji, action) = if new_now_playing {
            ("🔔", "Sending")
        } else {
            ("🔕", "Not sending")
        };

        out!(
            format!("{emoji} **{action}** now-playing track messages from now on."),
            ctx
        );
    }
}
