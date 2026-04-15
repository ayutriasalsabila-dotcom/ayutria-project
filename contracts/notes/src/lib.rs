#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short, Env, String, Symbol, Vec,
};

// Storage key
const TASKS: Symbol = symbol_short!("TASKS");

// Enum Priority
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Priority {
    Low,
    Medium,
    High,
}

// Struct Task
#[contracttype]
#[derive(Clone, Debug)]
pub struct Task {
    pub id: u64,
    pub title: String,
    pub completed: bool,
    pub priority: Priority,
}

#[contract]
pub struct TaskContract;

#[contractimpl]
impl TaskContract {
    // Ambil semua task
    pub fn get_tasks(env: Env) -> Vec<Task> {
        env.storage()
            .instance()
            .get(&TASKS)
            .unwrap_or(Vec::new(&env))
    }

    // Tambah task baru
    pub fn add_task(env: Env, title: String, priority: Priority) -> String {
        let mut tasks: Vec<Task> = env
            .storage()
            .instance()
            .get(&TASKS)
            .unwrap_or(Vec::new(&env));

        let task = Task {
            id: env.prng().gen::<u64>(),
            title,
            completed: false,
            priority,
        };

        tasks.push_back(task);
        env.storage().instance().set(&TASKS, &tasks);

        String::from_str(&env, "Task berhasil ditambahkan")
    }

    // Tandai selesai
    pub fn complete_task(env: Env, id: u64) -> String {
        let mut tasks: Vec<Task> = env
            .storage()
            .instance()
            .get(&TASKS)
            .unwrap_or(Vec::new(&env));

        for i in 0..tasks.len() {
            let mut task = tasks.get(i).unwrap();

            if task.id == id {
                task.completed = true;
                tasks.set(i, task);

                env.storage().instance().set(&TASKS, &tasks);
                return String::from_str(&env, "Task selesai");
            }
        }

        String::from_str(&env, "Task tidak ditemukan")
    }

    // Hapus task
    pub fn delete_task(env: Env, id: u64) -> String {
        let mut tasks: Vec<Task> = env
            .storage()
            .instance()
            .get(&TASKS)
            .unwrap_or(Vec::new(&env));

        for i in 0..tasks.len() {
            if tasks.get(i).unwrap().id == id {
                tasks.remove(i);

                env.storage().instance().set(&TASKS, &tasks);
                return String::from_str(&env, "Task dihapus");
            }
        }

        String::from_str(&env, "Task tidak ditemukan")
    }
}

mod test;