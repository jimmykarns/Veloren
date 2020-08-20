extern crate serde_json;

use super::schema::{body, character, entity, item, stats};
use crate::comp;
use diesel::sql_types::Text;
use serde::{Deserialize, Serialize};
use tracing::warn;

#[derive(Debug, Insertable, PartialEq)]
#[table_name = "entity"]
pub struct Entity {
    pub entity_id: i64,
}

#[derive(Insertable)]
#[table_name = "character"]
pub struct NewCharacter<'a> {
    pub character_id: i64,
    pub body_id: i32,
    pub player_uuid: &'a str,
    pub alias: &'a str,
}

#[derive(Identifiable, Queryable, Debug)]
#[primary_key(character_id)]
#[table_name = "character"]
pub struct Character {
    pub character_id: i64,
    pub body_id: i32,
    pub player_uuid: String,
    pub alias: String,
}

#[derive(Debug, Insertable, PartialEq, Queryable, AsChangeset)]
#[table_name = "item"]
pub struct NewItem {
    pub item_id: Option<i64>,
    pub parent_container_item_id: i64,
    pub item_definition_id: String,
    pub stack_size: Option<i32>,
    pub position: Option<String>,
}

#[derive(Debug, Queryable)]
pub struct Item {
    pub item_id: i64,
    pub parent_container_item_id: i64,
    pub item_definition_id: String,
    pub stack_size: Option<i32>,
    pub position: Option<String>,
}

#[derive(Associations, AsChangeset, Identifiable, Queryable, Debug, Insertable)]
#[belongs_to(Character)]
#[primary_key(character_id)]
#[table_name = "stats"]
pub struct Stats {
    pub character_id: i64,
    pub level: i32,
    pub exp: i32,
    pub endurance: i32,
    pub fitness: i32,
    pub willpower: i32,
    pub skills: Option<String>,
}

#[derive(Associations, Identifiable, Debug, Insertable)]
#[primary_key(body_id)]
#[table_name = "body"]
pub struct NewBody {
    pub body_id: Option<i32>,
    pub variant: String,
    pub body_data: String
}

#[derive(Associations, Identifiable, Queryable, Debug)]
#[primary_key(body_id)]
#[table_name = "body"]
pub struct Body {
    pub body_id: i32,
    pub variant: String,
    pub body_data: String
}

/// A wrapper type for the SkillSet of a character used to serialise to and from
/// JSON If the column contains malformed JSON, a default skillset is returned
#[derive(AsExpression, Debug, Deserialize, Serialize, PartialEq, FromSqlRow)]
#[sql_type = "Text"]
pub struct SkillSetData(pub comp::SkillSet);

impl<DB> diesel::deserialize::FromSql<Text, DB> for SkillSetData
where
    DB: diesel::backend::Backend,
    String: diesel::deserialize::FromSql<Text, DB>,
{
    fn from_sql(
        bytes: Option<&<DB as diesel::backend::Backend>::RawValue>,
    ) -> diesel::deserialize::Result<Self> {
        let t = String::from_sql(bytes)?;

        match serde_json::from_str(&t) {
            Ok(data) => Ok(Self(data)),
            Err(e) => {
                warn!(?e, "Failed to deserialize skill set data");
                Ok(Self(comp::SkillSet::default()))
            },
        }
    }
}

impl<DB> diesel::serialize::ToSql<Text, DB> for SkillSetData
where
    DB: diesel::backend::Backend,
{
    fn to_sql<W: std::io::Write>(
        &self,
        out: &mut diesel::serialize::Output<W, DB>,
    ) -> diesel::serialize::Result {
        let s = serde_json::to_string(&self.0)?;
        <String as diesel::serialize::ToSql<Text, DB>>::to_sql(&s, out)
    }
}