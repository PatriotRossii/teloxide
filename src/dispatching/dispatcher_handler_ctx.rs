use crate::{
    dispatching::session::GetChatId,
    requests::{Request, ResponseResult},
    types::Message,
    Bot,
};
use std::sync::Arc;

/// A [`Dispatcher`]'s handler's context of a bot and an update.
///
/// See [the module-level documentation for the design
/// overview](crate::dispatching).
///
/// [`Dispatcher`]: crate::dispatching::Dispatcher
pub struct DispatcherHandlerCtx<Upd> {
    pub bot: Arc<Bot>,
    pub update: Upd,
}

impl<Upd> GetChatId for DispatcherHandlerCtx<Upd>
where
    Upd: GetChatId,
{
    fn chat_id(&self) -> i64 {
        self.update.chat_id()
    }
}

impl DispatcherHandlerCtx<Message> {
    pub async fn reply<T>(&self, text: T) -> ResponseResult<()>
    where
        T: Into<String>,
    {
        self.bot
            .send_message(self.chat_id(), text)
            .send()
            .await
            .map(|_| ())
    }
}