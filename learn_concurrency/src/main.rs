
fn main() {
    learn_thread::thread_spawn();
    learn_message_passing::sync_mpsc();
    learn_message_passing::muti_message();
    learn_message_passing::muti_producer();
    learn_shared_state::mutex_lock();
    learn_shared_state::mutex_shared();
}

mod learn_thread;
mod learn_message_passing;
mod learn_shared_state;
