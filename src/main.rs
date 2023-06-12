mod ffi;
use poise::serenity_prelude as serenity;
use poise::{Framework, FrameworkOptions};
use songbird::SerenityInit;

struct Data {}
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(slash_command)]
async fn join(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild_id().expect("guild only");
    let guild = ctx.guild().unwrap();
    let channel_id = guild
        .voice_states
        .get(&ctx.author().id)
        .and_then(|voice_state| voice_state.channel_id)
        .ok_or("not in a voice channel")?;
    let manager = songbird::get(ctx.serenity_context())
        .await
        .expect("Songbird Voice client is not initialized.")
        .clone();
    let (call, _) = manager.join(guild_id, channel_id).await;
    Ok(())
}

// resampling 16ghz to 48ghz
fn resampling(wave: Vec<u8>) -> Vec<u8> {
    let mut resampled = Vec::new();
    for i in 0..wave.len() / 2 {
        resampled.push(wave[i * 2]);
        resampled.push(wave[i * 2 + 1]);
        resampled.push(0);
        resampled.push(0);
    }
    resampled
}

#[poise::command(slash_command)]
async fn play(ctx: Context<'_>, text: String) -> Result<(), Error> {
    let guild_id = ctx.guild_id().expect("guild only");
    let manager = songbird::get(ctx.serenity_context())
        .await
        .ok_or("Songbird Voice client is not initialized.")?;
    let call = manager.get(guild_id).expect("call not found");
    println!("text: {}", text);
    let result = ffi::synthe(text).unwrap();
    let result = resampling(result);
    let source = songbird::input::Input::new(
        false, songbird::input::Reader::from(result),
        songbird::input::Codec::Pcm, songbird::input::Container::Raw,
        None,
    );
    call.lock().await.play_source(source);
    ctx.say("ok").await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let wav = ffi::synthe("てすと".to_string()).unwrap();
    std::fs::write("test.wav", wav).unwrap();
    let framework = Framework::builder()
        .options(FrameworkOptions {
            commands: vec![join(), play()],
            ..Default::default()
        })
        .token(std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN"))
        .intents(serenity::GatewayIntents::non_privileged())
        .client_settings(|client| {
            client.register_songbird()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        });

    framework.run().await.unwrap();
}
