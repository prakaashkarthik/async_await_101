// Practicing things I read in: https://rust-lang.github.io/async-book/01_getting_started/04_async_await_primer.html


/*
Trial and Error thought process: 
1. The book said that if a thread is "blocked" then it would execute another thread. I foolishly thought that "sleeping" might be considered a block, so I called `sleep` for 1sec in the for-loops of `learn_song()` and `dance()`. But this is still "work". At this point, I still did not fully understand what might be considered a "block", but I had to replace `sleep` with something else... 
2. So I replaced this with the DANCE_STEP_NUMBER variable that would be used to coordinate learning the song lyrics. `dance()` would increment/write to DANCE_STEP_NUMBER, and `learn_song()` would read it and continue learning the song only if  `DANCE_STEP_NUMBER == num_words` learned. 
	1. Since two different functions are responsible for this, I (again foolishly) thought the "yield" would happen automatically since `learn_song()` would be "blocked". Here as well, there is still work for the thread to do (loop and print), so it isn't really a "block"
3. I realized that I had to explicitly tell the scheduler that `learn_song()` and `dance()` were blocked, and that I'd need to manually yield to the other function. I then used the `std::threads::yield_now()` call. However, this gave me the same result as #2 -- `learn_song()` was not yielding to `dance()` 
4. Then I remembered that Rust does not really have an Async runtime -- so I would probably need to use `tokio` and it's `yield_now().await`. Once I implemented that, I got it to work!


*/


use std::thread::{self}; // , yield_now};
use std::time::Duration;
use futures::executor::block_on;
use tokio::task::yield_now;

// async fn hello_world() {
//     println!("Hello, world!");
// }

async fn learn_song() -> String {
    let lyrics = "One Two Three Four, Get On The Dance Floor";
    let mut song = String::new();

    for w in lyrics.split_whitespace() {
        song = song + w + " ";
        let num_words:i32 = song.split_whitespace().count().try_into().unwrap();
        unsafe {
            loop {
                if DANCE_STEP_NUMBER == num_words {
                    println!("Going to the next lyric");
                    break;
                } else {
                    println!("Dance step number: {DANCE_STEP_NUMBER}. num_words: {num_words}. Waiting to learn dance step");
                    yield_now().await;
                }
            }
        }
        // thread::sleep(Duration::from_secs(1));
        println!("Song learned so far: {song}\n");
    }
    song
}

async fn sing_song(song: &str) {
    println!("The full song is: {song}");
}

static mut DANCE_STEP_NUMBER: i32 = 0;

async fn dance() {
    for i in 1..10 {
        thread::sleep(Duration::from_secs(1));
        println!("Learning dance step #{i}");
        unsafe {
            DANCE_STEP_NUMBER = i;
        }
        yield_now().await;
    }
}

async fn learn_and_sing() {
    let song = learn_song().await;
    sing_song(&song).await; // NOTE: Turns out .await is optional for this program to function correctly
}

async fn async_main() {
    let f1 = learn_and_sing();
    let f2 = dance();
    futures::join!(f1, f2);
}


#[tokio::main]
async fn main() {
    block_on(async_main());
}
