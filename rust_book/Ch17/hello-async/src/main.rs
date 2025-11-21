use trpl::{Either, Html};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    // Use provided arguments or fall back to defaults
    let url1 = args.get(1).map(|s| s.as_str()).unwrap_or("https://example.com");
    let url2 = args.get(2).map(|s| s.as_str()).unwrap_or("https://httpbin.org/html");

    trpl::run(async {
        let start = std::time::Instant::now();
        
        let title_fut_1 = page_title(url1);
        let title_fut_2 = page_title(url2);

        println!("Both HTTP requests started!");
        println!("Now spawning tasks with different start times...\n");
        
        // Task 1: Starts immediately, runs 5 items
        let task1_handle = trpl::spawn_task(async {
            let task_start = std::time::Instant::now();
            for i in 1..=5 {
                let elapsed = task_start.elapsed();
                println!("  [{}ms] 🔧 Task 1 - Work item {}", elapsed.as_millis(), i);
                trpl::sleep(std::time::Duration::from_millis(75)).await;
            }
            let elapsed = task_start.elapsed();
            println!("  [{}ms] ✅ Task 1 complete!\n", elapsed.as_millis());
        });

        // Task 2: Spawned immediately, but sleeps 200ms before starting work
        let task2_handle = trpl::spawn_task(async {
            trpl::sleep(std::time::Duration::from_millis(200)).await;
            let task_start = std::time::Instant::now();
            for i in 1..=3 {
                let elapsed = task_start.elapsed();
                println!("  [{}ms] ⚙️  Task 2 - Work item {}", elapsed.as_millis(), i);
                trpl::sleep(std::time::Duration::from_millis(100)).await;
            }
            let elapsed = task_start.elapsed();
            println!("  [{}ms] ✅ Task 2 complete!\n", elapsed.as_millis());
        });

        // Wait for first HTTP to complete (doesn't cancel task handles)
        let (url, maybe_title) =
            match trpl::select(title_fut_1, title_fut_2).await {
                Either::Left(left) => left,
                Either::Right(right) => right,
            };

        let http_elapsed = start.elapsed();
        println!("\n🎯 [{}ms] HTTP result arrived!", http_elapsed.as_millis());
        println!("{url} returned first");
        match maybe_title {
            Some(title) => println!("Its page title was: '{title}'"),
            None => println!("It had no title."),
        }

        println!("\nHTTP done, but tasks continue in background...\n");

        // Wait for both background tasks to finish
        task1_handle.await.unwrap();
        task2_handle.await.unwrap();
        
        let total_elapsed = start.elapsed();
        println!("All tasks complete! Total time: {}ms", total_elapsed.as_millis());
    })
}

async fn page_title(url: &str) -> (&str, Option<String>) {
    let response_text = trpl::get(url).await.text().await;
    let title = Html::parse(&response_text)
        .select_first("title")
        .map(|title| title.inner_html());
    (url, title)
 }
