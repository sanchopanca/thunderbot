use std::env;

use sqlx::{Pool, Sqlite, SqlitePool};

#[derive(Clone)]
pub struct Db {
    pool: Pool<Sqlite>,
}

struct DBRule {
    id: i64,
    name: String,
    updated_by: String,
    updated_at: i64,
}

#[allow(dead_code)]
struct DBPattern {
    id: i64,
    pattern: String,
    rule_id: i64,
    updated_by: String,
    updated_at: i64,
}

#[allow(dead_code)]
struct DBResponse {
    id: i64,
    response: String,
    rule_id: i64,
    updated_by: String,
    updated_at: i64,
}

#[derive(Clone, Debug)]
pub struct Rule {
    pub id: i64,
    pub name: String,
    pub patterns: Vec<String>,
    pub responses: Vec<String>,
    pub updated_by: String,
    pub updated_at: i64,
}

impl Db {
    pub async fn new() -> Self {
        let db_url = env::var("DATABASE_URL").expect("Provide DATABASE_URL env variable");
        let pool = SqlitePool::connect(&db_url).await.unwrap();
        Self { pool }
    }

    pub async fn get_rule(&self, id: i64) -> Rule {
        // TODO: return Result
        let db_rule = sqlx::query_as!(
            DBRule,
            "SELECT id, name, updated_by, updated_at FROM rules WHERE id = ?",
            id
        )
        .fetch_one(&self.pool)
        .await
        .unwrap();

        let patterns: Vec<String> =
            sqlx::query!("SELECT pattern FROM patterns WHERE rule_id = ?", id)
                .fetch_all(&self.pool)
                .await
                .unwrap()
                .into_iter()
                .map(|r| r.pattern)
                .collect();

        let responses: Vec<String> =
            sqlx::query!("SELECT response FROM responses WHERE rule_id = ?", id)
                .fetch_all(&self.pool)
                .await
                .unwrap()
                .into_iter()
                .map(|r| r.response)
                .collect();

        Rule {
            id: db_rule.id,
            name: db_rule.name,
            patterns,
            responses,
            updated_by: db_rule.updated_by,
            updated_at: db_rule.updated_at,
        }
    }

    pub async fn get_rules(&self) -> Vec<Rule> {
        let db_rules =
            sqlx::query_as!(DBRule, "SELECT id, name, updated_by, updated_at FROM rules")
                .fetch_all(&self.pool)
                .await
                .unwrap();
        let db_patterns = sqlx::query_as!(
            DBPattern,
            "SELECT id, pattern, rule_id, updated_by, updated_at FROM patterns"
        )
        .fetch_all(&self.pool)
        .await
        .unwrap();

        let db_reponses = sqlx::query_as!(
            DBResponse,
            "SELECT id, response, rule_id, updated_by, updated_at FROM responses"
        )
        .fetch_all(&self.pool)
        .await
        .unwrap();

        let mut rules = Vec::new();

        for db_rule in db_rules {
            let rule = Rule {
                id: db_rule.id,
                name: db_rule.name,
                patterns: db_patterns
                    .iter()
                    .filter(|p| p.rule_id == db_rule.id)
                    .map(|p| p.pattern.clone())
                    .collect(),
                responses: db_reponses
                    .iter()
                    .filter(|r| r.rule_id == db_rule.id)
                    .map(|r| r.response.clone())
                    .collect(),
                updated_by: db_rule.updated_by,
                updated_at: db_rule.updated_at,
            };
            rules.push(rule);
        }

        rules
    }

    pub async fn create_rule(
        &self,
        name: String,
        patterns: Vec<String>,
        responses: Vec<String>,
    ) -> Rule {
        let id = sqlx::query!(
            "INSERT INTO rules (name, updated_by) VALUES (?, ?)",
            name,
            "user"
        )
        .execute(&self.pool)
        .await
        .unwrap()
        .last_insert_rowid();

        // TODO: bulk inserts https://docs.rs/sqlx-core/latest/sqlx_core/query_builder/struct.QueryBuilder.html#method.push_values
        for pattern in &patterns {
            sqlx::query!(
                "INSERT INTO patterns (pattern, rule_id, updated_by) VALUES (?, ?, ?)",
                pattern,
                id,
                "user"
            )
            .execute(&self.pool)
            .await
            .unwrap();
        }

        for response in &responses {
            sqlx::query!(
                "INSERT INTO responses (response, rule_id, updated_by) VALUES (?, ?, ?)",
                response,
                id,
                "user"
            )
            .execute(&self.pool)
            .await
            .unwrap();
        }

        Rule {
            id,
            name,
            patterns,
            responses,
            updated_by: "user".to_string(),
            updated_at: 0, // TODO real data ?
        }
    }
}
