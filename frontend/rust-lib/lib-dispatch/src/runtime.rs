use std::future::Future;
use std::{io, thread};

use tokio::runtime;
use tokio::runtime::Runtime;
use tokio::task::{JoinHandle, LocalSet};

pub struct AFPluginRuntime {
  inner: Runtime,
  local: LocalSet,
}

impl AFPluginRuntime {
  pub fn new() -> io::Result<Self> {
    let inner = default_tokio_runtime()?;
    Ok(Self {
      inner,
      local: LocalSet::new(),
    })
  }

  #[track_caller]
  pub fn spawn<F>(&self, future: F) -> JoinHandle<F::Output>
  where
    F: Future + 'static,
  {
    self.local.spawn_local(future)
  }

  #[track_caller]
  pub fn block_on<F>(&self, f: F) -> F::Output
  where
    F: Future,
  {
    self.local.block_on(&self.inner, f)
  }
}

pub fn default_tokio_runtime() -> io::Result<Runtime> {
  runtime::Builder::new_multi_thread()
    .thread_name("dispatch-rt")
    .enable_io()
    .enable_time()
    .on_thread_start(move || {
      tracing::trace!(
        "{:?} thread started: thread_id= {}",
        thread::current(),
        thread_id::get()
      );
    })
    .on_thread_stop(move || {
      tracing::trace!(
        "{:?} thread stopping: thread_id= {}",
        thread::current(),
        thread_id::get(),
      );
    })
    .build()
}
