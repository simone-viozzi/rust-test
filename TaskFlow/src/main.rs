use std::{
    sync::{Arc, Mutex},
    time::SystemTime,
};

use rand::Rng;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

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
    rgn: rand::rngs::ThreadRng,
}

impl TaskProcessor {
    // Constructor to initialize state
    fn new(initial_state: f32) -> Self {
        TaskProcessor {
            state: Arc::new(Mutex::new(initial_state)),
            rgn: rand::thread_rng(),
        }
    }

    // Method to process a task and update the shared state
    fn process_task(&mut self, task: Task) -> f32 {
        // Lock the state for safe access
        let mut state = self.state.lock().expect("Failed to acquire lock on state");

        // Modify the state based on the task type
        match task.task_type {
            TaskType::Add => *state += task.num,
            TaskType::Sub => *state -= task.num,
            TaskType::Mul => *state *= task.num,
        }

        println!("Processed task: id={}, new state={}", task.id, *state);
        thread::sleep(Duration::from_millis(self.rgn.gen_range(100..1000)));

        state.clone()
    }
}


struct ProducerState {
    next_id: u64,
    producer_flag: bool,
}

impl ProducerState {
    fn new() -> Self {
        ProducerState { next_id: 0, producer_flag: true }
    }

    fn get_next_id(&mut self) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        id
    }
}

fn producer(tx: mpsc::SyncSender<Task>, state: Arc<Mutex<ProducerState>>) {
    let mut rng = rand::thread_rng();

    loop {
        // Lock the state and check the producer flag
        let id = {
            let mut state = state.lock().unwrap();
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
            "Producing task: id={}, type={:?}, num={}",
            task.id, task.task_type, task.num
        );
        tx.send(task).unwrap(); // Send the task to the queue
        thread::sleep(Duration::from_millis(rng.gen_range(100..1000)));
    }
}

fn consumer(rx: Arc<Mutex<mpsc::Receiver<Task>>>) {
    let mut rng = rand::thread_rng();
    let mut task_processor = TaskProcessor::new(rng.gen());
    while let Ok(task) = rx.lock().unwrap().recv() {
        // Receive tasks from the queue
        let creation_time = task.created_at; // Copy the creation time (SystemTime is Copy)

        println!(
            "Consuming task: id={}, type={:?}, num={}",
            task.id, task.task_type, task.num
        );

        // Process the task
        let new_state = task_processor.process_task(task);

        // Print the state and the time it took to process the task from its creation
        let elapsed = creation_time.elapsed().unwrap();

        println!(
            "Current state: {:.4}, time elapsed: {} ms",
            new_state,           // State with 4 decimal places
            elapsed.as_millis()  // Elapsed time in milliseconds
        );

        thread::sleep(Duration::from_millis(rng.gen_range(100..1000))); // Simulate task processing delay
    }
}

fn main() {
    println!("Hello, world!");

    let producer_state = Arc::new(Mutex::new(ProducerState::new())); // Shared state for task IDs

    let (tx, rx): (mpsc::SyncSender<Task>, mpsc::Receiver<Task>) = mpsc::sync_channel(100);
    let rx = Arc::new(Mutex::new(rx));


    let mut threads: Vec<thread::JoinHandle<_>> = Vec::new(); // Vector to hold thread handles

    for _ in 0..5 {
        let tx_clone = tx.clone();
        let producer_state_clone = Arc::clone(&producer_state); // Clone shared state

        let t = thread::spawn(move || {
            producer(tx_clone, producer_state_clone);
        });

        threads.push(t);
    }

    for _ in 0..5 {
        let rx_clone = Arc::clone(&rx);
        
        let t = thread::spawn(move || {
            consumer(rx_clone);
        });

        threads.push(t);
    }

    thread::sleep(Duration::from_millis(10000));

    {
        let mut producer_state = producer_state.lock().unwrap();
        producer_state.producer_flag = false;
    }

    // Join all threads
    for thread in threads {
        thread.join().unwrap();
    }

    println!("All threads have finished.");
}
