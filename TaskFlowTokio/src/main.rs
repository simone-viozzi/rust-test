use std::{sync::Arc, time::SystemTime};

use rand::Rng;
use std::thread;
use rand::rngs::StdRng;
use rand::SeedableRng;

use tokio::sync::Mutex;
use tokio::time::{sleep, Duration};

use crossbeam::channel::{bounded, Sender, Receiver};


#[derive(Debug)]
enum TaskType {
    Add,
    Sub,
    Mul,
}

impl TaskType {
    // Associated function to generate a random TaskType
    fn random() -> Self {
        let mut rng = rand::thread_rng();
        const VARIANT_COUNT: u8 = 3;
        match rng.gen_range(0..VARIANT_COUNT) {
            0 => TaskType::Add,
            1 => TaskType::Sub,
            2 => TaskType::Mul,
            _ => unreachable!(), // This branch should never happen
        }
    }
}

struct Task {
    id: u64,
    task_type: TaskType,
    num: f32,
    created_at: SystemTime,
}

struct TaskProcessor {
    state: Arc<Mutex<f32>>, // Shared, mutable state
    rng: rand::rngs::StdRng,
}

impl TaskProcessor {
    // Constructor to initialize state
    fn new(initial_state: f32) -> Self {
        TaskProcessor {
            state: Arc::new(Mutex::new(initial_state)),
            rng: StdRng::seed_from_u64(42),
        }
    }

    // Method to process a task and update the shared state
    async fn process_task(&mut self, task: Task) -> f32 {
        // Lock the state for safe access
        let mut state = self.state.lock().await;

        // Modify the state based on the task type
        match task.task_type {
            TaskType::Add => *state += task.num,
            TaskType::Sub => *state -= task.num,
            TaskType::Mul => *state *= task.num,
        }

        println!("\tProcessed task: id={}, new state={}", task.id, *state);
        sleep(Duration::from_millis(self.rng.gen_range(100..1000))).await;

        state.clone()
    }
}

struct ProducerState {
    next_id: u64,
    producer_flag: bool,
}

impl ProducerState {
    fn new() -> Self {
        ProducerState {
            next_id: 0,
            producer_flag: true,
        }
    }

    fn get_next_id(&mut self) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        id
    }
}

async fn producer(tx: Sender<Task>, state: Arc<Mutex<ProducerState>>) {
    let mut rng = StdRng::seed_from_u64(42);

    loop {
        // Lock the state and check the producer flag
        let id = {
            let mut state = state.lock().await;
            if !state.producer_flag {
                break;
            }
            state.get_next_id()
        };

        let num: f32 = rng.gen();
        let task_type = TaskType::random();

        let task = Task {
            id,
            num,
            task_type,
            created_at: SystemTime::now(),
        };

        println!(
            "++ Producing task: thread={:?}, id={}, type={:?}, num={}",
            thread::current().id(),
            task.id,
            task.task_type,
            task.num
        );
        if let Err(e) = tx.send(task) {
            println!("Failed to send task: {}", e);
            break;
        }
        sleep(Duration::from_millis(rng.gen_range(100..1000))).await;
    }
}

async fn consumer(rx: Receiver<Task>) {
    let mut rng = StdRng::seed_from_u64(42);
    let mut task_processor = TaskProcessor::new(rng.gen());
    while let Ok(task) = rx.recv() {
        // Receive tasks from the queue
        let creation_time = task.created_at; // Copy the creation time (SystemTime is Copy)

        println!(
            "-- Consuming task: thread={:?}, id={}, type={:?}, num={}",
            thread::current().id(),
            task.id,
            task.task_type,
            task.num
        );

        // Process the task
        let new_state = task_processor.process_task(task).await;

        // Print the state and the time it took to process the task from its creation
        let elapsed = creation_time.elapsed().unwrap();

        println!(
            "\tCurrent state: {:.4}, time elapsed: {} ms",
            new_state,
            elapsed.as_millis()
        );

        sleep(Duration::from_millis(rng.gen_range(100..1000))).await; // Simulate task processing delay
    }
}

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    let producer_state = Arc::new(Mutex::new(ProducerState::new())); // Shared state for task IDs

    let (tx, rx): (Sender<Task>, Receiver<Task>) = bounded(100);

    let mut threads: Vec<tokio::task::JoinHandle<_>> = Vec::new(); // Vector to hold thread handles

    for _ in 0..5 {
        let tx_clone = tx.clone(); // Clone tx for each producer
        let producer_state_clone = Arc::clone(&producer_state);

        let t = tokio::spawn(async move {
            producer(tx_clone, producer_state_clone).await;
        });

        threads.push(t);
    }

    for _ in 0..5 {
        let rx_clone = rx.clone();

        let t = tokio::spawn(async move {
            consumer(rx_clone).await;
        });

        threads.push(t);
    }

    tokio::time::sleep(Duration::from_millis(5000)).await;

    {
        let mut producer_state = producer_state.lock().await;
        producer_state.producer_flag = false;
    }

    drop(tx);

    // Join all threads
    for thread in threads {
        thread.await.unwrap();
    }

    println!("All threads have finished.");
}
