#[cfg(test)]
mod tests {
    use std::thread;
    use std::time::Duration;
    use std::sync::{mpsc, Mutex, Arc};

    #[test]
    fn using_thread_run_code_simultaneously() {
        let handle = thread::spawn(|| {
            println!("current thread in spawn:{:?}", thread::current());
            for i in 1..10 {
                println!("hi number {} from the spawned thead!", i);
                thread::sleep(Duration::from_millis(1));
            };
        });
        for i in 1..5 {
            println!("hi number {} from the main thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
        println!("current thread:{:?}", thread::current());
        handle.join().unwrap();
    }

    #[test]
    fn move_ownership_in_thread() {
        let v = vec![1, 2, 3];
        //由于V在不同的线程执行，rust不知道每个线程的执行时间，没办法回收v，所以外部变量不能直接在其他线程使用
        let handle = thread::spawn(move || {
            println!("Here's a vector:{:?}", v);
        });
        handle.join().unwrap();
    }

    #[test]
    fn thread_communication_with_channel() {
        //create channel
        //mpsc meaning multiple producer and single consumer
        let (tx, rx) = mpsc::channel();
        thread::spawn(move || {
            let value = String::from("hi");
            tx.send(value).unwrap();
        });
        let received = rx.recv().unwrap();
        println!("Got:{}", received);
    }

    #[test]
    fn thread_sending_multiple_value_through_channel() {
        let (tx, rx) = mpsc::channel();
        thread::spawn(move || {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("thread"),
            ];
            for val in vals {
                tx.send(val).unwrap();
                thread::sleep(Duration::from_secs(1));
            }
        });
        for x in rx {
            println!("God: {}", x);
        }
    }

    #[test]
    fn thread_sending_multiple_value_in_different_thread() {
        let (tx, rx) = mpsc::channel();
        let tx1 = tx.clone();
        thread::spawn(move || {
            let values = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("thread"),
            ];
            for v in values {
                tx1.send(v).unwrap();
                thread::sleep(Duration::from_secs(1));
            }
        });
        thread::spawn(move || {
            let vals = vec![
                String::from("more"),
                String::from("messages"),
                String::from("for"),
                String::from("you"),
            ];
            for val in vals {
                tx.send(val).unwrap();
                thread::sleep(Duration::from_secs(1));
            }
        });
        for received in rx {
            println!("Got:{}", received);
        }
    }

    #[test]
    fn concurrency_with_shared_state() {
        let m = Mutex::new(5);
        {
            let mut num = m.lock().unwrap();
            *num = 6;
            //num MuteGuard会在作用域结束自动释放锁
        }
        println!("m = {:?}", m);
    }

    #[test]
    fn sharing_mutex_between_multiple_threads() {
        let counter = Arc::new(Mutex::new(0));
        let mut handles = vec![];
        for _ in 0..10 {
            let counter = Arc::clone(&counter);
            let handle = thread::spawn(move || {
                let mut mutex_guard = counter.lock().unwrap();
                *mutex_guard += 1;
            });
            handles.push(handle);
        }
        for handle in handles {
            handle.join().unwrap();
        }
        println!("Result:{}", counter.lock().unwrap());
    }

    #[test]
    fn extensible_concurrency_with_sync_and_send_trait(){

    }
}