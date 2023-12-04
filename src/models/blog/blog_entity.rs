use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::{FromRow, PgPool, query};
use uuid::Uuid;

#[derive(sqlx::FromRow, Serialize, PartialEq, Debug, Default)]
pub struct BlogEntity {
    #[sqlx(rename = "blog_id")]
    pub id: Uuid,
    #[sqlx(rename = "blog_title")]
    pub title: String,
    #[sqlx(rename = "blog_description")]
    pub description: String,
    #[sqlx(rename = "blog_content")]
    pub content: String,
    #[sqlx(rename = "blog_created_at")]
    pub created_at: DateTime<Utc>,
}

#[derive(sqlx::FromRow, Serialize, PartialEq, Debug, Default)]
pub struct BlogSummaryEntity {
    #[sqlx(rename = "blog_id")]
    pub id: Uuid,
    #[sqlx(rename = "blog_title")]
    pub title: String,
    #[sqlx(rename = "blog_description")]
    pub description: String,
    #[sqlx(rename = "blog_created_at")]
    pub created_at: DateTime<Utc>,
}

impl BlogSummaryEntity {
    pub async fn get_all_summaries(pool: &PgPool) -> Result<Vec<Self>, sqlx::Error> {
        let result = query(
            //language=PostgreSQL
            r#"
            SELECT
                blog_id,
                blog_title,
                blog_description,
                blog_created_at
            FROM
                blog_post
        "#
        )
            .fetch_all(pool)
            .await?;

        let blog_posts: Vec<Self> = result
            .into_iter()
            .map(|row| Self::from_row(&row).unwrap_or_default())
            .collect::<Vec<Self>>();

        Ok(blog_posts)
    }
}

impl BlogEntity {
    pub async fn get_post(pool: &PgPool, blog_id: Uuid) -> Result<Self, sqlx::Error> {
        let result = query(
            //language=PostgreSQL
            r#"
            SELECT
                blog_id,
                blog_title,
                blog_description,
                blog_content,
                blog_created_at
            FROM
                blog_post
            WHERE
                blog_id = $1
        "#
        )
            .bind(blog_id)
            .fetch_one(pool)
            .await?;

        Ok(Self::from_row(&result).unwrap_or_default())
    }
}