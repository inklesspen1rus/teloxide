use crate::{
    dispatching::{update_listeners, update_listeners::UpdateListener},
    dispatching2::Dispatcher,
    error_handlers::{LoggingErrorHandler, OnError},
    utils::command::BotCommand,
};
use dptree::di::{DependencyMap, Injector};
use futures::StreamExt;
use std::{fmt::Debug, future::Future, marker::PhantomData, sync::Arc};
use teloxide_core::{requests::Requester, types::Message};
use tokio_stream::wrappers::UnboundedReceiverStream;

/// A [REPL] for commands.
///
/// All errors from an update listener and handler will be logged.
///
/// # Caution
/// **DO NOT** use this function together with [`Dispatcher`] and other REPLs,
/// because Telegram disallow multiple requests at the same time from the same
/// bot.
///
/// [REPL]: https://en.wikipedia.org/wiki/Read-eval-print_loop
/// [`Dispatcher`]: crate::dispatching::Dispatcher
#[cfg(feature = "ctrlc_handler")]
pub async fn commands_repl<'a, R, Cmd, H, N, E, Args>(
    requester: R,
    bot_name: N,
    handler: H,
    cmd: PhantomData<Cmd>,
) where
    Cmd: BotCommand + Send + Sync + 'static,
    H: Injector<DependencyMap, Result<(), E>, Args> + Send + Sync + 'static,
    N: Into<String> + Send + 'static,
    R: Requester + Clone + Send + Sync + 'static,
    <R as Requester>::GetUpdatesFaultTolerant: Send,
    E: Send + Sync + 'static,
{
    let cloned_requester = requester.clone();

    commands_repl_with_listener(
        requester,
        bot_name,
        handler,
        update_listeners::polling_default(cloned_requester).await,
        cmd,
    )
    .await;
}

/// Like [`commands_repl`], but with a custom [`UpdateListener`].
///
/// All errors from an update listener and handler will be logged.
///
/// # Caution
/// **DO NOT** use this function together with [`Dispatcher`] and other REPLs,
/// because Telegram disallow multiple requests at the same time from the same
/// bot.
///
/// [`Dispatcher`]: crate::dispatching::Dispatcher
/// [`commands_repl`]: crate::dispatching::repls::commands_repl()
/// [`UpdateListener`]: crate::dispatching::update_listeners::UpdateListener
#[cfg(feature = "ctrlc_handler")]
pub async fn commands_repl_with_listener<'a, R, Cmd, H, L, ListenerE, N, E, Args>(
    requester: R,
    bot_name: N,
    handler: H,
    listener: L,
    _cmd: PhantomData<Cmd>,
) where
    Cmd: BotCommand + Send + Sync + 'static,
    H: Injector<DependencyMap, Result<(), E>, Args> + Send + Sync + 'static,
    L: UpdateListener<ListenerE> + Send + 'a,
    ListenerE: Debug + Send + 'a,
    N: Into<String> + Send + 'static,
    R: Requester + Clone + Send + Sync + 'static,
    E: Send + Sync + 'static,
{
    let bot_name = bot_name.into();

    let dispatcher = Dispatcher::new(Arc::new(requester)).messages_handler(|h| {
        h.chain(dptree::filter_map(move |message: Arc<Message>| {
            let bot_name = bot_name.clone();
            async move { message.text().and_then(|text| Cmd::parse(text, bot_name).ok()) }
        }))
        .branch(dptree::endpoint(handler))
    });

    #[cfg(feature = "ctrlc_handler")]
    let dispatcher = dispatcher.setup_ctrlc_handler();

    // To make mutable var from immutable.
    let mut dispatcher = dispatcher;

    dispatcher
        .dispatch_with_listener(
            listener,
            LoggingErrorHandler::with_custom_text("An error from the update listener"),
        )
        .await;
}
