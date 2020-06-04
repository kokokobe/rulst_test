#[cfg(test)]
mod tests {
    // `block_on` blocks the current thread until the provided future has run to
    // completion. Other executors provide more complex behavior, like scheduling
    // multiple futures onto the same thread.
    use futures::{executor};
    use futures::{
        future::{Fuse, FusedFuture, FutureExt, TryFutureExt},
        stream::{StreamExt, Stream, FusedStream},
        join, try_join,
        pin_mut, select,
    };
    use futures::executor::block_on;

    struct Book {}

    struct Music {}

    #[test]
    fn join() {
        async fn get_book() -> Book {
            println!("get a book");
            Book {}
        }
        async fn get_music() -> Music {
            println!("get a music");
            Music {}
        }
        async fn get_book_and_music() -> (Book, Music) {
            let book_fut = get_book();
            let music_fut = get_music();
            join!(book_fut,music_fut)
        }
        executor::block_on(get_book_and_music());
    }

    #[test]
    fn try_join() {
        // future 返回 result时使用
        // Unlike join!, try_join! will complete immediately if one of the subfutures returns an error.
        async fn get_book() -> Result<Book, String> {
            println!("try join get book");
            Ok(Book {})
        }
        async fn get_music() -> Result<Music, String> {
            println!("try join get music");
            Ok(Music {})
        }
        async fn get_book_and_music() -> Result<(Book, Music), String> {
            let book_fut = get_book();
            let music_fut = get_music();
            try_join!(book_fut,music_fut)
        }
        executor::block_on(get_book_and_music());

        //try join different result type
        async fn get_book2() -> Result<Book, ()> {
            println!("try get book 2");
            Ok(Book {})
        }
        async fn get_music2() -> Result<Music, String> {
            println!("try get music 2");
            Ok(Music {})
        }
        async fn get_book_and_music2() -> Result<(Book, Music), String> {
            let book_fut = get_book2()
                .map_err(|()| "Unable to get book".to_string());
            let music_fut = get_music2();
            try_join!(book_fut,music_fut)
        }
        executor::block_on(get_book_and_music2());
    }

    #[test]
    fn select() {
        async fn task_one() {}
        async fn task_two() {}
        async fn race_tasks() {
            let t1 = task_one().fuse();
            let t2 = task_two().fuse();
            pin_mut!(t1,t2);
            select! {
                ()= t1=> println!("task one completed first"),
                ()= t2 => println!("task two completed first"),
            }
        }
        block_on(race_tasks());
    }

    #[test]
    fn select2() {
        use futures::{future, select};
        async fn count() {
            let mut a_fut = future::ready(4);
            let mut b_fut = future::ready(6);
            let mut total = 0;
            loop {
                select! {
                    a= a_fut =>total+=a,
                    b= b_fut =>total+=b,
                    complete=>break,
                    // never runs (futures are ready, then complete)
                    default=>unreachable!(),
                }
            }
            println!("total count is:{}", total);
            assert_eq!(total, 10);
        }
        block_on(count())
    }

    #[test]
    fn fused_future() {
        async fn _add_two_stream(
            mut s1: impl Stream<Item=u8> + FusedStream + Unpin,
            mut s2: impl Stream<Item=u8> + FusedStream + Unpin,
        ) -> u8 {
            let mut total = 0;
            loop {
                let item = select! {
                    x = s1.next()=>x,
                    x = s2.next()=>x,
                    complete=>break,
                };
                if let Some(next_num) = item {
                    total += next_num;
                }
            }
            total
        }
    }

    #[test]
    fn concurrent_task_in_select_loop() {
        async fn get_new_num() -> u8 { 5 }
        async fn run_on_new_num(_: u8) {}
        async fn run_loop(mut interval_timer: impl Stream<Item=()> + FusedStream + Unpin, staring_num: u8) {
            let run_on_new_num_fut = run_on_new_num(staring_num).fuse();
            let get_new_num_fut = Fuse::terminated();
            pin_mut!(run_on_new_num_fut,get_new_num_fut);
            loop {
                select! {
                    () = interval_timer.select_next_some() =>{
                        // The timer has elapsed. Start a new `get_new_num_fut`
                        // if one was not already running.
                        if get_new_num_fut.is_terminated(){
                            get_new_num_fut.set(get_new_num().fuse());
                        }
                    },
                    new_num = get_new_num_fut =>{
                        // A new number has arrived-- start a new `run_on_new_num_fut`,
                        // dropping the old one.
                        run_on_new_num_fut.set(run_on_new_num(new_num).fuse());
                    },
                    // Run the `run_on_new_num_fut`
                    () = run_on_new_num_fut => {},
                    // panic if everything completed, since the `interval_timer` should
                    // keep yielding values indefinitely.
                    complete => panic!("`interval_timer` completed unexpectedly"),
                }
            }
        }
    }
}