use std::error::Error;
use tokio::sync::mpsc;
use tokio::time::{interval, Duration, Instant};

/// Structure representing an event indicating that a VU did not meet its execution interval.
#[derive(Debug)]
struct VuEvent {
    vu_id: usize,
    scheduled_interval: Duration,
    actual_duration: Duration,
    timestamp: Instant,
}

/// Structure to track the statistics of each VU.
#[derive(Clone)]
struct VuStats {
    vu_id: usize,
    execution_count: usize,
    total_duration: Duration,
}

impl VuStats {
    fn new(vu_id: usize) -> Self {
        VuStats {
            vu_id,
            execution_count: 0,
            total_duration: Duration::new(0, 0),
        }
    }

    fn update(&mut self, duration: Duration) {
        self.execution_count += 1;
        self.total_duration += duration;
    }

    fn frequency(&self) -> f64 {
        self.execution_count as f64 / self.total_duration.as_secs_f64()
    }
}

/// Function representing the execution loop of a VU.
///
/// - `vu_id`: virtual user identifier (for logging or tracking)
/// - `interval_duration`: duration between executions (desired fixed period)
/// - `simulated_work_duration`: duration of simulated work (may vary)
/// - `event_tx`: channel to send a notification if the VU exceeds its deadline.
async fn vu_task(
    vu_id: usize,
    interval_duration: Duration,
    simulated_work_duration: Duration,
    event_tx: mpsc::Sender<VuEvent>,
    stats_tx: mpsc::Sender<VuStats>,
) {
    // Create an interval that ticks every `interval_duration`
    let mut ticker = interval(interval_duration);
    let mut vu_stats = VuStats::new(vu_id);

    loop {
        // Record the start time before beginning work.
        let start = Instant::now();

        // Wait for the next tick of the interval.
        ticker.tick().await;

        // --- Simulate the VU's work --- //
        // For example, we simulate work that takes `simulated_work_duration`.
        tokio::time::sleep(simulated_work_duration).await;
        // --------------------------------- //

        let elapsed = start.elapsed();
        vu_stats.update(elapsed);

        // Check if the actual duration exceeds the expected interval.
        if elapsed > interval_duration {
            // Create an event to signal that the VU did not meet its frequency.
            let event = VuEvent {
                vu_id,
                scheduled_interval: interval_duration,
                actual_duration: elapsed,
                timestamp: Instant::now(),
            };

            // Send the event on the channel.
            // Here we ignore any error if the receiver is no longer available.
            let _ = event_tx.send(event).await;
        } else {
            println!(
                "VU {}: executed within the expected duration ({:?})",
                vu_id, elapsed
            );
        }

        // Send the updated stats.
        let _ = stats_tx.send(vu_stats.clone()).await;
    }
}

/// Supervisor function that receives notifications from VUs and processes them (here, it logs them).
async fn supervisor(mut event_rx: mpsc::Receiver<VuEvent>, mut stats_rx: mpsc::Receiver<VuStats>) {
    // Infinite loop to listen for messages on the channel.

    loop {
        tokio::select! {
            Some(event) = event_rx.recv() => {
                eprintln!(
                    "Alert for VU {}: execution duration of {:?} exceeded the expected interval of {:?} at {:?}",
                    event.vu_id,
                    event.actual_duration,
                    event.scheduled_interval,
                    event.timestamp,
                );
            },
            Some(stats) = stats_rx.recv() => {
                println!("VU {}: execution frequency: {:.2} executions per second", stats.vu_id, stats.frequency());
            },
        }
    }
}

/// Main function
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // --- Configurable parameters --- //
    let number_of_vus = 10; // Number of desired virtual users
    let interval_duration = Duration::from_secs(1); // Frequency: 1 execution per second
    let simulated_work_duration = Duration::from_millis(800); // Simulated work duration of 800 ms
                                                              // ----------------------------------- //

    // Create an mpsc channel to centralize error events (buffer size = 100)
    let (event_tx, event_rx) = mpsc::channel::<VuEvent>(100);
    let (stats_tx, stats_rx) = mpsc::channel::<VuStats>(100);

    // Launch the supervisor task to listen for notifications from the VUs.
    tokio::spawn(supervisor(event_rx, stats_rx));

    // Launch VU tasks in parallel.
    for i in 0..number_of_vus {
        // Each VU receives a clone of the channel to send its notifications.
        let tx_clone = event_tx.clone();
        let stats_clone = stats_tx.clone();
        tokio::spawn(vu_task(
            i,
            interval_duration,
            simulated_work_duration,
            tx_clone,
            stats_clone,
        ));
    }

    // To prevent the program from terminating immediately,
    // wait indefinitely (or define a stopping condition).
    loop {
        tokio::time::sleep(Duration::from_secs(60)).await;
    }
}
