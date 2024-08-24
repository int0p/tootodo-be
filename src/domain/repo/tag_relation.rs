pub struct TagRelationRepository {
   pub pool: PgPool,
}

impl TagRelationRepository {
   pub async fn create(&self, tag_id: &Uuid, group_id: &Uuid) -> Result<TagRelation> {
       let query = Query::insert()
           .into_table(TagRelation::TABLE)
           .columns(&[
               TagRelation::TAG_ID,
               TagRelation::GROUP_ID,
           ])
           .values_panic(&[
               Expr::value(tag_id),
               Expr::value(group_id),
           ])
           .build(PostgresQueryBuilder);

       sqlx::query(&query.to_string())
           .bind(tag_id)
           .bind(group_id)
           .execute(&self.pool)
           .await?;

       Ok(TagRelation {
           tag_id: *tag_id,
           group_id: *group_id,
       })
   }

   pub async fn delete_by_tag_id(&self, tag_id: &Uuid) -> Result<()> {
       sqlx::query(&Query::delete()
           .from_table(TagRelation::TABLE)
           .so_that(Condition::all()
               .add(Expr::col(TagRelation::TAG_ID).eq(Expr::value(tag_id))))
           .build(PostgresQueryBuilder))
       .execute(&self.pool)
       .await?;
       Ok(())
   }

   pub async fn delete_by_group_id(&self, group_id: &Uuid) -> Result<()> {
       sqlx::query(&Query::delete()
           .from_table(TagRelation::TABLE)
           .so_that(Condition::all()
               .add(Expr::col(TagRelation::GROUP_ID).eq(Expr::value(group_id))))
           .build(PostgresQueryBuilder))
       .execute(&self.pool)
       .await?;
       Ok(())
   }

   pub async fn find_by_tag_and_group(&self, tag_id: &Uuid, group_id: &Uuid) -> Result<Option<TagRelation>> {
       let row = sqlx::query_as::<_, (Uuid, Uuid)>(
           &Query::select()
               .column(TagRelation::TAG_ID)
               .column(TagRelation::GROUP_ID)
               .from_table(TagRelation::TABLE)
               .so_that(Condition::all()
                   .add(Expr::col(TagRelation::TAG_ID).eq(Expr::value(tag_id)))
                   .add(Expr::col(TagRelation::GROUP_ID).eq(Expr::value(group_id))))
               .build(PostgresQueryBuilder),
       )
       .fetch_optional(&self.pool)
       .await?;

       row.map(|(tag_id, group_id)| TagRelation {
           tag_id,
           group_id,
       })
   }
}
