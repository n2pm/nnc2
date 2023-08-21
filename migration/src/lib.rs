pub use sea_orm_migration::prelude::*;

mod m20230820_000001_create_tables;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(m20230820_000001_create_tables::Migration)]
    }
}

fn create_index<'a, T, C>(table: T, col: C) -> IndexCreateStatement
where
    T: IntoIden,
    C: IntoIden,
{
    let table = table.into_iden();
    let col = col.into_iden();
    Index::create()
        .name(format!("idx-{}-{}", table.to_string(), col.to_string()))
        .table(table)
        .col(col)
        .take()
}
