use rusqlite::{params, Connection, Result};

pub fn init_database() -> Result<Connection, rusqlite::Error> {
    let conn = Connection::open("todo.db")?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS tasks (
        id INTEGER PRIMARY KEY,
        job TEXT NOT NULL,
        status BOOLEAN NOT NULL
    )",
        [],
    )?;

    Ok(conn)
}

pub fn add_task(conn: &Connection, task: String) -> Result<(), rusqlite::Error> {
    let mut stmt = conn.prepare("SELECT job fROM tasks WHERE job = ?1")?;

    let status = stmt.exists(params![task])?;
    if !status {
        conn.execute(
            "INSERT INTO tasks (job, status) VALUES (?1, ?2)",
            params![task, true],
        )?;
        println!("Task {} added in todo list",task);
    } else {
        println!("Task already exists in database!!")
    }

    Ok(())
}

pub fn show_tasks(conn: &Connection) -> Result<(), rusqlite::Error> {
    let mut stmt = conn.prepare("SELECT job FROM tasks WHERE status=1")?;

    let tasks = stmt.query_map([], |row| Ok(row.get::<_, String>(0)?))?;

    for task in tasks {
        match task {
            Ok(value) => println!("{}!", value),
            Err(_error) => continue,
        }
    }

    Ok(())
}

pub fn done_task(conn: &Connection, task: String) -> Result<(), rusqlite::Error> {
    let mut stmt = conn.prepare("SELECT job FROM tasks WHERE job=?1")?;
    let result = stmt.exists(params![task])?;
    if result {
        conn.execute(
            "UPDATE tasks SET status=?1 WHERE job=?",
            params![false, task],
        )?;
        println!("Done Great !!");
    }

    Ok(())
}
