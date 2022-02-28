//! SeaORM Entity. Generated by sea-orm-codegen 0.5.0

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "ih_facilities")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: Option<String>,
    pub created_date: Option<DateTime>,
    pub updated_date: Option<DateTime>,
    pub is_delete: Option<i8>,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        panic!("No RelationDef")
    }
}

impl ActiveModelBehavior for ActiveModel {}