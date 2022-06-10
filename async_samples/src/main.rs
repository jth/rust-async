use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    const NUM_THREADS: usize = 2;
    // Runtime or Executor polls the async functions until completion
    // Rust does *not* provide an async runtime by default.
    // Tokio is the community driven, most popular one (low-level runtime)
    let mut task_handles = Vec::with_capacity(NUM_THREADS);

    for i in 0..NUM_THREADS {
        task_handles.push(tokio::spawn(async move {
            iam_an_async_func(i).await;
        }));
    }

    for t in task_handles {
        t.await.unwrap();
    }
    //    let f = iam_an_async_func();
    //    println!("Main"); // executed before the Future
    //    f.await;
}

/// "async"-keyword is syntatic sugar for
/// `fn iam_an_async_func() -> Future<Output = ()>`
///
/// See [doc.rust-lang.org](https://doc.rust-lang.org/std/future/trait.Future.html)
/// for definition of Future. Futures are lazy and don't do anything until called.
async fn iam_an_async_func(i: usize) {
    println!("[{i}] I'm async!");

    // Calling .await will try to run the Future to completion
    let s1 = read_from_db().await;
    println!("[{i}] First Result: {s1}");
    let s2 = read_from_db().await;
    println!("[{i}] Second Result: {s2}");
}

async fn read_from_db() -> String {
    // asynchronous sleep from Tokio
    sleep(Duration::from_millis(100)).await;
    "DB Result".to_owned()
}
