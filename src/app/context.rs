use crate::api::Api;
use std::{io::Write, sync::Mutex};

pub(super) struct Context {
    api: Api,
    output: Mutex<Box<dyn Write>>,
}

impl std::fmt::Debug for Context {
    #[fehler::throws(std::fmt::Error)]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) {
        f.debug_struct("Context")
            .field("api", &self.api)
            .field("output", &"..")
            .finish()?;
    }
}

#[derive(Debug)]
pub(super) struct WithContext<T> {
    wrapped: T,
    context: Context,
}

pub(super) trait With: Sized {
    fn with(self, context: Context) -> WithContext<Self>;
}

impl<T> With for T {
    fn with(self, context: Context) -> WithContext<Self> {
        WithContext {
            wrapped: self,
            context,
        }
    }
}

impl Context {
    pub(super) fn new(api: Api, output: impl Write + 'static) -> Self {
        Context {
            api,
            output: Mutex::new(Box::new(output)),
        }
    }
}

impl<T> WithContext<T> {
    pub(super) fn as_ref(&self) -> &T {
        &self.wrapped
    }

    pub(super) fn api(&self) -> &Api {
        &self.context.api
    }

    pub(super) fn output(&self) -> impl std::ops::DerefMut<Target = impl Write> + '_ {
        self.context.output.lock().unwrap()
    }

    pub(super) fn map<U>(self, f: impl FnOnce(T) -> U) -> WithContext<U> {
        WithContext {
            wrapped: f(self.wrapped),
            context: self.context,
        }
    }

    pub(super) fn and_then<U>(self, f: impl FnOnce(T, Context) -> U) -> U {
        f(self.wrapped, self.context)
    }
}

impl<T> std::ops::Deref for WithContext<T> {
    type Target = T;

    fn deref(&self) -> &T {
        self.as_ref()
    }
}
