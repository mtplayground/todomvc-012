//! Integration tests for all todo server functions.
//! Each test uses a fresh in-memory SQLite database so tests are fully isolated.

#[cfg(feature = "ssr")]
mod tests {
    use sqlx::sqlite::SqlitePoolOptions;
    use sqlx::SqlitePool;
    use todomvc::{
        db_add_todo, db_clear_completed, db_delete_todo, db_get_todos, db_toggle_all,
        db_toggle_todo, db_update_todo_title,
    };

    async fn setup_db() -> SqlitePool {
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

    // ── add_todo ────────────────────────────────────────────────────────────

    #[tokio::test]
    async fn test_add_todo_success() {
        let pool = setup_db().await;
        db_add_todo(&pool, "Buy milk").await.expect("Should add todo");
        let todos = db_get_todos(&pool).await.expect("Should get todos");
        assert_eq!(todos.len(), 1);
        assert_eq!(todos[0].title, "Buy milk");
        assert!(!todos[0].completed);
    }

    #[tokio::test]
    async fn test_add_todo_trims_whitespace() {
        let pool = setup_db().await;
        db_add_todo(&pool, "  trimmed  ")
            .await
            .expect("Should add trimmed todo");
        let todos = db_get_todos(&pool).await.expect("Should get todos");
        assert_eq!(todos[0].title, "trimmed");
    }

    #[tokio::test]
    async fn test_add_todo_empty_title_rejected() {
        let pool = setup_db().await;
        let result = db_add_todo(&pool, "").await;
        assert!(result.is_err(), "Empty title should be rejected");
    }

    #[tokio::test]
    async fn test_add_todo_whitespace_only_rejected() {
        let pool = setup_db().await;
        let result = db_add_todo(&pool, "   ").await;
        assert!(result.is_err(), "Whitespace-only title should be rejected");
    }

    #[tokio::test]
    async fn test_add_multiple_todos() {
        let pool = setup_db().await;
        db_add_todo(&pool, "First").await.expect("Should add first");
        db_add_todo(&pool, "Second").await.expect("Should add second");
        db_add_todo(&pool, "Third").await.expect("Should add third");
        let todos = db_get_todos(&pool).await.expect("Should get todos");
        assert_eq!(todos.len(), 3);
        assert_eq!(todos[0].title, "First");
        assert_eq!(todos[1].title, "Second");
        assert_eq!(todos[2].title, "Third");
    }

    // ── get_todos ───────────────────────────────────────────────────────────

    #[tokio::test]
    async fn test_get_todos_empty_database() {
        let pool = setup_db().await;
        let todos = db_get_todos(&pool).await.expect("Should return empty list");
        assert!(todos.is_empty());
    }

    #[tokio::test]
    async fn test_get_todos_ordered_by_id() {
        let pool = setup_db().await;
        db_add_todo(&pool, "A").await.unwrap();
        db_add_todo(&pool, "B").await.unwrap();
        db_add_todo(&pool, "C").await.unwrap();
        let todos = db_get_todos(&pool).await.unwrap();
        assert!(todos[0].id < todos[1].id);
        assert!(todos[1].id < todos[2].id);
    }

    // ── toggle_todo ─────────────────────────────────────────────────────────

    #[tokio::test]
    async fn test_toggle_todo_marks_completed() {
        let pool = setup_db().await;
        db_add_todo(&pool, "Task").await.unwrap();
        let id = db_get_todos(&pool).await.unwrap()[0].id;

        db_toggle_todo(&pool, id).await.expect("Should toggle");
        let todos = db_get_todos(&pool).await.unwrap();
        assert!(todos[0].completed, "Todo should be completed after toggle");
    }

    #[tokio::test]
    async fn test_toggle_todo_twice_restores_state() {
        let pool = setup_db().await;
        db_add_todo(&pool, "Task").await.unwrap();
        let id = db_get_todos(&pool).await.unwrap()[0].id;

        db_toggle_todo(&pool, id).await.unwrap();
        db_toggle_todo(&pool, id).await.unwrap();
        let todos = db_get_todos(&pool).await.unwrap();
        assert!(!todos[0].completed, "Double-toggle should restore original state");
    }

    #[tokio::test]
    async fn test_toggle_todo_nonexistent_id_no_error() {
        let pool = setup_db().await;
        // Toggling a non-existent ID should succeed (no rows affected, no error)
        let result = db_toggle_todo(&pool, 9999).await;
        assert!(result.is_ok(), "Toggle on non-existent ID should not error");
    }

    // ── delete_todo ─────────────────────────────────────────────────────────

    #[tokio::test]
    async fn test_delete_todo_removes_it() {
        let pool = setup_db().await;
        db_add_todo(&pool, "To delete").await.unwrap();
        let id = db_get_todos(&pool).await.unwrap()[0].id;

        db_delete_todo(&pool, id).await.expect("Should delete");
        let todos = db_get_todos(&pool).await.unwrap();
        assert!(todos.is_empty(), "Deleted todo should be gone");
    }

    #[tokio::test]
    async fn test_delete_todo_only_removes_target() {
        let pool = setup_db().await;
        db_add_todo(&pool, "Keep me").await.unwrap();
        db_add_todo(&pool, "Delete me").await.unwrap();
        let todos = db_get_todos(&pool).await.unwrap();
        let delete_id = todos[1].id;

        db_delete_todo(&pool, delete_id).await.unwrap();
        let remaining = db_get_todos(&pool).await.unwrap();
        assert_eq!(remaining.len(), 1);
        assert_eq!(remaining[0].title, "Keep me");
    }

    #[tokio::test]
    async fn test_delete_todo_nonexistent_id_no_error() {
        let pool = setup_db().await;
        let result = db_delete_todo(&pool, 9999).await;
        assert!(result.is_ok(), "Deleting non-existent ID should not error");
    }

    // ── update_todo_title ───────────────────────────────────────────────────

    #[tokio::test]
    async fn test_update_todo_title_success() {
        let pool = setup_db().await;
        db_add_todo(&pool, "Old title").await.unwrap();
        let id = db_get_todos(&pool).await.unwrap()[0].id;

        db_update_todo_title(&pool, id, "New title")
            .await
            .expect("Should update title");
        let todos = db_get_todos(&pool).await.unwrap();
        assert_eq!(todos[0].title, "New title");
    }

    #[tokio::test]
    async fn test_update_todo_title_trims_whitespace() {
        let pool = setup_db().await;
        db_add_todo(&pool, "Original").await.unwrap();
        let id = db_get_todos(&pool).await.unwrap()[0].id;

        db_update_todo_title(&pool, id, "  spaced  ").await.unwrap();
        let todos = db_get_todos(&pool).await.unwrap();
        assert_eq!(todos[0].title, "spaced");
    }

    #[tokio::test]
    async fn test_update_todo_title_empty_deletes_todo() {
        let pool = setup_db().await;
        db_add_todo(&pool, "Will be deleted").await.unwrap();
        let id = db_get_todos(&pool).await.unwrap()[0].id;

        db_update_todo_title(&pool, id, "")
            .await
            .expect("Empty title should delete the todo, not error");
        let todos = db_get_todos(&pool).await.unwrap();
        assert!(todos.is_empty(), "Todo should be deleted when title is set to empty");
    }

    #[tokio::test]
    async fn test_update_todo_title_whitespace_only_deletes_todo() {
        let pool = setup_db().await;
        db_add_todo(&pool, "Will be deleted").await.unwrap();
        let id = db_get_todos(&pool).await.unwrap()[0].id;

        db_update_todo_title(&pool, id, "   ").await.unwrap();
        let todos = db_get_todos(&pool).await.unwrap();
        assert!(todos.is_empty(), "Todo should be deleted when title is whitespace-only");
    }

    #[tokio::test]
    async fn test_update_todo_title_nonexistent_id_no_error() {
        let pool = setup_db().await;
        let result = db_update_todo_title(&pool, 9999, "New title").await;
        assert!(result.is_ok(), "Updating non-existent ID should not error");
    }

    // ── toggle_all ──────────────────────────────────────────────────────────

    #[tokio::test]
    async fn test_toggle_all_marks_all_completed() {
        let pool = setup_db().await;
        db_add_todo(&pool, "A").await.unwrap();
        db_add_todo(&pool, "B").await.unwrap();
        db_add_todo(&pool, "C").await.unwrap();

        db_toggle_all(&pool, true).await.expect("Should toggle all complete");
        let todos = db_get_todos(&pool).await.unwrap();
        assert!(todos.iter().all(|t| t.completed), "All todos should be completed");
    }

    #[tokio::test]
    async fn test_toggle_all_marks_all_active() {
        let pool = setup_db().await;
        db_add_todo(&pool, "A").await.unwrap();
        db_add_todo(&pool, "B").await.unwrap();

        db_toggle_all(&pool, true).await.unwrap();
        db_toggle_all(&pool, false).await.expect("Should toggle all active");
        let todos = db_get_todos(&pool).await.unwrap();
        assert!(todos.iter().all(|t| !t.completed), "All todos should be active");
    }

    #[tokio::test]
    async fn test_toggle_all_empty_database_no_error() {
        let pool = setup_db().await;
        let result = db_toggle_all(&pool, true).await;
        assert!(result.is_ok(), "Toggle all on empty DB should not error");
    }

    #[tokio::test]
    async fn test_toggle_all_mixed_state_all_set_completed() {
        let pool = setup_db().await;
        db_add_todo(&pool, "A").await.unwrap();
        db_add_todo(&pool, "B").await.unwrap();
        // Toggle only the first one
        let id = db_get_todos(&pool).await.unwrap()[0].id;
        db_toggle_todo(&pool, id).await.unwrap();

        db_toggle_all(&pool, true).await.unwrap();
        let todos = db_get_todos(&pool).await.unwrap();
        assert!(todos.iter().all(|t| t.completed));
    }

    // ── clear_completed ─────────────────────────────────────────────────────

    #[tokio::test]
    async fn test_clear_completed_removes_only_completed() {
        let pool = setup_db().await;
        db_add_todo(&pool, "Active").await.unwrap();
        db_add_todo(&pool, "Done").await.unwrap();
        let id = db_get_todos(&pool).await.unwrap()[1].id;
        db_toggle_todo(&pool, id).await.unwrap();

        db_clear_completed(&pool)
            .await
            .expect("Should clear completed");
        let todos = db_get_todos(&pool).await.unwrap();
        assert_eq!(todos.len(), 1);
        assert_eq!(todos[0].title, "Active");
    }

    #[tokio::test]
    async fn test_clear_completed_removes_all_when_all_completed() {
        let pool = setup_db().await;
        db_add_todo(&pool, "A").await.unwrap();
        db_add_todo(&pool, "B").await.unwrap();
        db_toggle_all(&pool, true).await.unwrap();

        db_clear_completed(&pool).await.unwrap();
        let todos = db_get_todos(&pool).await.unwrap();
        assert!(todos.is_empty(), "All completed todos should be cleared");
    }

    #[tokio::test]
    async fn test_clear_completed_keeps_all_when_none_completed() {
        let pool = setup_db().await;
        db_add_todo(&pool, "A").await.unwrap();
        db_add_todo(&pool, "B").await.unwrap();

        db_clear_completed(&pool).await.unwrap();
        let todos = db_get_todos(&pool).await.unwrap();
        assert_eq!(todos.len(), 2, "Active todos should not be removed");
    }

    #[tokio::test]
    async fn test_clear_completed_empty_database_no_error() {
        let pool = setup_db().await;
        let result = db_clear_completed(&pool).await;
        assert!(result.is_ok(), "Clear completed on empty DB should not error");
    }
}
