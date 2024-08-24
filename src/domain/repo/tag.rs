pub struct TagRepository {
   pub pool: PgPool,
}

impl TagRepository {
   pub async fn create(&self, dto: &CreateTagDto) -> Result<Tag> {
       let query = Query::insert()
           .into_table(Tag::TABLE)
           .columns(&[
               Tag::USER,
               Tag::NAME,
               Tag::CREATED_AT,
           ])
           .values_panic(&[
               Expr::value(dto.user),
               Expr::value(dto.name.clone()),
               Expr::value(chrono::Utc::now()),
           ])
           .build(PostgresQueryBuilder);

       let id: uuid::Uuid = sqlx::query_scalar(&query.to_string())
           .bind(dto.user)
           .bind(dto.name)
           .bind(chrono::Utc::now())
           .fetch_one(&self.pool)
           .await?;

       Ok(Tag {
           id,
           user: dto.user,
           name: dto.name.clone(),
           created_at: chrono::Utc::now(),
       })
   }

   pub async fn update(&self, id: &Uuid, dto: &UpdateTagDto) -> Result<()> {
       sqlx::query(&Query::update()
           .table(Tag::TABLE)
           .values(&[(
               Tag::NAME,
               Expr::value(dto.name.clone()),
           )])
           .so_that(Condition::all()
               .add(Expr::col(Tag::ID).eq(Expr::value(id))))
           .build(PostgresQueryBuilder))
       .execute(&self.pool)
       .await?;
       Ok(())
   }

   pub async fn delete(&self, id: &Uuid) -> Result<()> {
       sqlx::query(&Query::delete()
           .from_table(Tag::TABLE)
           .so_that(Condition::all()
               .add(Expr::col(Tag::ID).eq(Expr::value(id))))
           .build(PostgresQueryBuilder))
       .execute(&self.pool)
       .await?;
       Ok(())
   }

   pub async fn find_by_user_and_name(&self, user: &Uuid, name: &str) -> Result<Option<Tag>> {
       let row = sqlx::query_as::<_, (Uuid, Uuid, String, DateTime<Utc>)>(
           &Query::select()
               .column(Tag::ID)
               .column(Tag::USER)
               .column(Tag::NAME)
               .column(Tag::CREATED_AT)
               .from_table(Tag::TABLE)
               .so_that(Condition::all()
                   .add(Expr::col(Tag::USER).eq(Expr::value(user)))
                   .add(Expr::col(Tag::NAME).eq(Expr::value(name))))
               .build(PostgresQueryBuilder),
       )
       .fetch_optional(&self.pool)
       .await?;

       row.map(|(id, user, name, created_at)| Tag {
           id,
           user,
           name,
           created_at,
       })
   }
}