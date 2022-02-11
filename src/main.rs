use async_timer::Interval;

/// Increment Timer
///
/// We increment our counter
/// For now we just increment counter by 1
///
fn increment_timer(counter: &mut usize) {
    *counter += 1;
    println!("Counter is {}", counter);
}

/// Runs our timer and starts jobs
///
/// This sets up a dependency on the future provided by increment_timer
/// For now we just run increment_timer for 5 seconds using the Interval
///
async fn run_timer() -> usize {
    let mut counter: usize = 0;
    let mut times: i32 = 0;
    let mut interval = Interval::platform_new(core::time::Duration::from_secs(1));

    // For now we wait for 10 seconds to complete/
    while times < 10
    {
        increment_timer(&mut counter);
        interval.wait().await;
        times += 1;
    }

    counter
}

// Tokio::main macro translates the main function back to a non-async function
// .await calls are transformed to block_on to make for easy coding
#[tokio::main]
async fn main() {
    let task = tokio::spawn(run_timer());

    // Block on our task
    MainWindow::new().run();

    let counter = task.await.unwrap();

    println!("Count is: {}", counter);
}

slint::slint! {
    MainWindow := Window {
        Text {
            text: "hello world";
            color: green;
        }
    }
}
