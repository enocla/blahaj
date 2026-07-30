use color_eyre::eyre::Result;
use poise::serenity_prelude::{Context, FullEvent, GuildId, User};

use crate::commands::misc::avatarsync::{
    get_avatar_emojis_for_user, refresh_avatar_emoji, sanitize_emoji_name,
};
use crate::types::Data;

pub async fn handle(ctx: &Context, event: &FullEvent, data: &Data) -> Result<()> {
    match event {
        FullEvent::GuildMemberUpdate {
            old_if_available,
            event,
            ..
        } => {
            if old_if_available
                .as_ref()
                .is_some_and(|old| old.user.avatar == event.user.avatar)
            {
                return Ok(());
            }

            refresh_guild_avatar(ctx, data, &event.user, event.guild_id).await;
        }
        FullEvent::UserUpdate { old_data, new } => {
            if old_data
                .as_ref()
                .is_some_and(|old| old.avatar == new.avatar)
            {
                return Ok(());
            }

            let user = User::from(new.clone());
            for existing in get_avatar_emojis_for_user(user.id)? {
                refresh_guild_avatar_with_name(
                    ctx,
                    data,
                    &user,
                    existing.guild_id,
                    &existing.emoji_name,
                )
                .await;
            }
        }
        _ => {}
    }

    Ok(())
}

async fn refresh_guild_avatar(ctx: &Context, data: &Data, user: &User, guild_id: GuildId) {
    let emoji_name = sanitize_emoji_name(&user.name);
    refresh_guild_avatar_with_name(ctx, data, user, guild_id, &emoji_name).await;
}

async fn refresh_guild_avatar_with_name(
    ctx: &Context,
    data: &Data,
    user: &User,
    guild_id: GuildId,
    emoji_name: &str,
) {
    let avatar_url = user.face();
    if let Err(e) = refresh_avatar_emoji(
        ctx.http.as_ref(),
        &data.client,
        user.id,
        guild_id,
        emoji_name,
        &avatar_url,
        false,
    )
    .await
    {
        eprintln!(
            "Failed to auto-update avatar emoji for user {} in guild {}: {e}",
            user.id, guild_id
        );
    }
}
