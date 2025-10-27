use poise::serenity_prelude as serenity;
use dice_roll::{parser};

struct Data {}
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

/// Replies with pong.
#[poise::command(slash_command, prefix_command)]
async fn ping(
    ctx: Context<'_>,
) -> Result<(), Error> {
    ctx.say("Pong!".to_string()).await?;
    return Ok(());
}

/// Roll a specified set of dice.
#[poise::command(slash_command, prefix_command)]
async fn roll(
    ctx: Context<'_>,
    #[description = "Example: 1d20 + 2d8 + 2"] roll: String,
) -> Result<(), Error> {
    let result = match parser::parse(roll) {
        Ok(val) => val,
        Err(e) => {
            ctx.say(e.to_string()).await?;
            return Ok(());
        }
    };

    let result = match result.roll_dice() {
        Ok(val) => val,
        Err(e) => {
            ctx.say(e.to_string()).await?;
            return Ok(());
        }
    };

    ctx.reply(result.to_string()).await?;
    return Ok(());
}

#[tokio::main]
async fn main() {
    let token =
        std::env::var("DISCORD_TOKEN").expect("Missing environment variable: DISCORD_TOKEN");
    let intents = serenity::GatewayIntents::non_privileged();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![roll(), ping()],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;
    client.unwrap().start().await.unwrap();
}
