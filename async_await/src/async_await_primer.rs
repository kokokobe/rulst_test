#[cfg(test)]
mod tests {
    // `block_on` blocks the current thread until the provided future has run to
    // completion. Other executors provide more complex behavior, like scheduling
    // multiple futures onto the same thread.
    use futures::executor::block_on;

    #[test]
    fn primer() {
        async fn hello_world() {
            println!("hello, world!");
        }
        // Nothing is printed
        let future = hello_world();
        // `future` is run and "hello, world!" is printed
        block_on(future);
    }

    #[test]
    fn primer2() {
        struct Song {}
        async fn learn_song() -> Song {
            /* ... */
            Song {}
        }
        async fn sing_song(song: Song) { /* ... */ }
        async fn dance() { /* ... */ }
        async fn learn_and_sing() {
            // Wait until the song has been learned before singing it.
            // We use `.await` here rather than `block_on` to prevent blocking the
            // thread, which makes it possible to `dance` at the same time.
            let song = learn_song().await;
            sing_song(song).await;
        }
        async fn async_main() {
            let f1 = learn_and_sing();
            let f2 = dance();
            // `join!` is like `.await` but can wait for multiple futures concurrently.
            // If we're temporarily blocked in the `learn_and_sing` future, the `dance`
            // future will take over the current thread. If `dance` becomes blocked,
            // `learn_and_sing` can take back over. If both futures are blocked, then
            // `async_main` is blocked and will yield to the executor.
            futures::join!(f1,f2);
        }
        block_on(async_main());
    }

    #[test]
    fn build_a_timer() {
        use {
            std::{
                future::Future,
                pin::Pin,
                sync::{Arc, Mutex},
                task::{Context, Poll, Waker},
                thread,
                time::Duration,
            }
        };
        use {
            futures::{
                future::{FutureExt, BoxFuture},
                task::{ArcWake, waker_ref},
            },
            std::{
                sync::mpsc::{sync_channel, SyncSender, Receiver},
            },
        };
        pub struct TimerFuture {
            shared_state: Arc<Mutex<SharedState>>,
        }
        /// Shared state between the future and the waiting thread
        struct SharedState {
            ///Whether or not the sleep time has elapsed
            completed: bool,
            /// The waker for the task that `TimerFuture` is running on.
            /// The thread can use this after setting `completed = true` to tell
            /// `TimerFuture`'s task to wake up, see that `completed = true`, and
            /// move forward.
            waker: Option<Waker>,
        }
        impl Future for TimerFuture {
            type Output = ();

            fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
                // Look at the shared state to see if the timer has already completed.
                let mut shared_state = self.shared_state.lock().unwrap();
                if shared_state.completed {
                    Poll::Ready(())
                } else {
                    // Set waker so that the thread can wake up the current task
                    // when the timer has completed, ensuring that the future is polled
                    // again and sees that `completed = true`.
                    //
                    // It's tempting to do this once rather than repeatedly cloning
                    // the waker each time. However, the `TimerFuture` can move between
                    // tasks on the executor, which could cause a stale waker pointing
                    // to the wrong task, preventing `TimerFuture` from waking up
                    // correctly.
                    //
                    // N.B. it's possible to check for this using the `Waker::will_wake`
                    // function, but we omit that here to keep things simple.
                    shared_state.waker = Some(cx.waker().clone());
                    Poll::Pending;
                }
            }
        }
        impl TimerFuture {
            /// Create a new `TimerFuture` which will complete after the provided
            /// timeout.
            pub fn new(duration: Duration) -> Self {
                let shared_state = Arc::new(Mutex::new(SharedState {
                    completed: false,
                    waker: None,
                }));
                //Spawn the thread
                let thread_shared_state = shared_state.clone();
                thread::spawn(move || {
                    thread::sleep(duration);
                    let mut shared_state = thread_shared_state.lock().unwrap();
                    // Signal that the timer has completed and wake up the last
                    // task on which the future was polled, if one exists.
                    shared_state.completed = true;
                    if let Some(waker) = shared_state.waker.take() {
                        waker.wake()
                    }
                });
                TimerFuture { shared_state }
            }
        }
        /// Task executor that receives tasks off of a channel and runs them.
        struct Executor {
            ready_queue: Receiver<Arc<Task>>,
        }
        /// `Spawner` spawns new futures onto the task channel.
        struct Spawner {
            task_sender: SyncSender<Arc<Task>>,
        }

        /// A future that can reschedule itself to be polled by an `Executor`.
        struct Task {
            /// In-progress future that should be pushed to completion.
           ///
           /// The `Mutex` is not necessary for correctness, since we only have
           /// one thread executing tasks at once. However, Rust isn't smart
           /// enough to know that `future` is only mutated from one thread,
           /// so we need use the `Mutex` to prove thread-safety. A production
           /// executor would not need this, and could use `UnsafeCell` instead.
            future: Mutex<Option<BoxFuture<'static, ()>>>,
            /// Handle to place the task itself back onto the task queue.
            task_sender: SyncSender<Arc<Task>>,
        }
        fn new_executor_and_spawner() -> (Executor, Spawner) {
            // Maximum number of tasks to allow queueing in the channel at once.
            // This is just to make `sync_channel` happy, and wouldn't be present in
            // a real executor.
            const MAX_QUEUED_TASKS: usize = 10_000;
            let (task_sender, ready_queue) = sync_channel(MAX_QUEUED_TASKS);
            (Executor { ready_queue }, Spawner { task_sender })
        }
        impl Spawner {
            fn spawn(&self, future: impl Future<Output=()> + 'static + Send) {
                let future = future.boxed();
            }
        }
    }
}