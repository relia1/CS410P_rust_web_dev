use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        db.execute_unprepared(
            "CREATE TABLE IF NOT EXISTS tags (
                id integer GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
                name TEXT NOT NULL
            )"
        ).await?;
        
        db.execute_unprepared(
            "CREATE TABLE IF NOT EXISTS questions (
                id integer GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
                title TEXT NOT NULL,
                content TEXT NOT NULL
            )"
        ).await?;

/*
* Juntion table
*/
        db.execute_unprepared(
            "CREATE TABLE IF NOT EXISTS question_tags (
                question_id integer NOT NULL,
                tag_id integer NOT NULL,
                PRIMARY KEY (question_id, tag_id),
                FOREIGN KEY (question_id) REFERENCES questions(id) ON DELETE CASCADE,
                FOREIGN KEY (tag_id) REFERENCES tags(id) ON DELETE CASCADE
            )"
        ).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        db.execute_unprepared(
            "DROP TABLE IF EXISTS question_tags;
            DROP TABLE IF EXISTS tags;
            DROP TABLE IF EXISTS questions;
        ").await?;

        Ok(())
    }
}

/*
#[derive(DeriveIden)]
enum Post {
    Table,
    Id,
    Title,
    Text,
}
*/
