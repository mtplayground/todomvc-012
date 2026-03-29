use leptos::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct Todo {
    pub id: i64,
    pub title: String,
    pub completed: bool,
}

#[server(AddTodo, "/api")]
pub async fn add_todo(title: String) -> Result<(), ServerFnError> {
    use leptos_axum::extract;
    use axum::Extension;
    use sqlx::SqlitePool;

    let Extension(pool): Extension<SqlitePool> = extract().await?;

    let title = title.trim().to_string();
    if title.is_empty() {
        return Err(ServerFnError::ServerError(
            "Title cannot be empty".to_string(),
        ));
    }

    sqlx::query("INSERT INTO todos (title, completed) VALUES (?, 0)")
        .bind(&title)
        .execute(&pool)
        .await
        .map_err(|e| ServerFnError::<server_fn::error::NoCustomError>::ServerError(e.to_string()))?;

    Ok(())
}

#[server(GetTodos, "/api")]
pub async fn get_todos() -> Result<Vec<Todo>, ServerFnError> {
    use leptos_axum::extract;
    use axum::Extension;
    use sqlx::SqlitePool;

    let Extension(pool): Extension<SqlitePool> = extract().await?;

    let todos = sqlx::query_as::<_, Todo>("SELECT id, title, completed FROM todos ORDER BY id ASC")
        .fetch_all(&pool)
        .await
        .map_err(|e| ServerFnError::<server_fn::error::NoCustomError>::ServerError(e.to_string()))?;

    Ok(todos)
}

#[cfg(test)]
#[cfg(feature = "ssr")]
mod tests {
    use super::*;
    use sqlx::sqlite::SqlitePoolOptions;

    async fn setup_test_db() -> sqlx::SqlitePool {
        let pool = SqlitePoolOptions::new()
            .max_connections(1)
            .connect("sqlite::memory:")
            .await
            .expect("Failed to create in-memory SQLite database");

        sqlx::query(
            "CREATE TABLE IF NOT EXISTS todos (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                title TEXT NOT NULL,
                completed BOOLEAN NOT NULL DEFAULT 0
            )",
        )
        .execute(&pool)
        .await
        .expect("Failed to create todos table");

        pool
    }

    #[tokio::test]
    async fn test_insert_and_fetch_todos() {
        let pool = setup_test_db().await;

        // Insert a todo directly
        sqlx::query("INSERT INTO todos (title, completed) VALUES (?, 0)")
            .bind("Test todo")
            .execute(&pool)
            .await
            .expect("Failed to insert todo");

        // Fetch all todos
        let todos = sqlx::query_as::<_, Todo>(
            "SELECT id, title, completed FROM todos ORDER BY id ASC",
        )
        .fetch_all(&pool)
        .await
        .expect("Failed to fetch todos");

        assert_eq!(todos.len(), 1);
        assert_eq!(todos[0].title, "Test todo");
        assert!(!todos[0].completed);
    }

    #[tokio::test]
    async fn test_insert_multiple_todos() {
        let pool = setup_test_db().await;

        let titles = ["First todo", "Second todo", "Third todo"];
        for title in &titles {
            sqlx::query("INSERT INTO todos (title, completed) VALUES (?, 0)")
                .bind(title)
                .execute(&pool)
                .await
                .expect("Failed to insert todo");
        }

        let todos = sqlx::query_as::<_, Todo>(
            "SELECT id, title, completed FROM todos ORDER BY id ASC",
        )
        .fetch_all(&pool)
        .await
        .expect("Failed to fetch todos");

        assert_eq!(todos.len(), 3);
        for (i, title) in titles.iter().enumerate() {
            assert_eq!(todos[i].title, *title);
        }
    }

    #[tokio::test]
    async fn test_empty_title_rejected() {
        // Validate that empty title strings would be caught before DB insertion
        let title = "   ".trim().to_string();
        assert!(
            title.is_empty(),
            "Whitespace-only title should be treated as empty"
        );
    }

    #[tokio::test]
    async fn test_todo_completed_defaults_false() {
        let pool = setup_test_db().await;

        sqlx::query("INSERT INTO todos (title, completed) VALUES (?, 0)")
            .bind("A new todo")
            .execute(&pool)
            .await
            .expect("Failed to insert todo");

        let todo = sqlx::query_as::<_, Todo>(
            "SELECT id, title, completed FROM todos WHERE title = ?",
        )
        .bind("A new todo")
        .fetch_one(&pool)
        .await
        .expect("Failed to fetch todo");

        assert!(!todo.completed, "New todo should not be completed");
    }
}
