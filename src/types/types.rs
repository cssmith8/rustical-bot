use pickledb::PickleDb;
use tokio::sync::Mutex;

pub struct Data {
    pub db: Mutex<PickleDb>,
} // User data, which is stored and accessible in all command invocations
pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;
pub type AppContext<'a> = poise::ApplicationContext<'a, Data, Error>;