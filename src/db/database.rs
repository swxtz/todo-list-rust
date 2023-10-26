use std::path::Path;

use colored::Colorize;

pub fn create_db() {
    let db: bool = verify_db();

    if !db {
        let message = "Creating DB 📁...".yellow();
        println!("{}",message);

        let conn = rusqlite::Connection::open("todolist.db").unwrap();

        conn.execute(
            "CREATE TABLE todolist (
                id TEXT PRIMARY KEY,
                task TEXT NOT NULL,
                description TEXT NOT NULL,
                completed BOOLEAN NOT NULL DEFAULT 0
            )",
            [],
        )
        .unwrap();

        let message = "DB created ✅".green();
        println!("{}",message);
    }
}

fn verify_db() -> bool {
    let message = "Verify DB 🔎...".yellow();
    println!("{}",message);

    let db_exists = Path::new("todolist.db").exists();

    if db_exists {
        let message = "DB exists ✅".green();
        println!("{}",message);
        return true;
    } else {
        let message = "DB does not exist ❌".red();
        println!("{}",message);
        return false;
    }

}