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
    pub id: i64,
    pub player_uuid: &'a str,
    pub alias: &'a str,
}

#[derive(Identifiable, Queryable, Debug)]
#[table_name = "character"]
pub struct Character {
    pub id: i64,
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

/// `Body` represents the body variety for a character, which has a one-to-one
/// relationship with Characters. This data is set during player creation, and
/// while there is currently no in-game functionality to modify it, it will
/// likely be added in the future.
#[derive(Associations, Identifiable, Queryable, Debug, Insertable)]
#[belongs_to(Character)]
#[primary_key(character_id)]
#[table_name = "body"]
pub struct Body {
    pub character_id: i64,
    pub species: i16,
    pub body_type: i16,
    pub hair_style: i16,
    pub beard: i16,
    pub eyes: i16,
    pub accessory: i16,
    pub hair_color: i16,
    pub skin: i16,
    pub eye_color: i16,
}

impl From<&Body> for comp::Body {
    fn from(body: &Body) -> comp::Body {
        comp::Body::Humanoid(comp::humanoid::Body {
            species: comp::humanoid::ALL_SPECIES[body.species as usize],
            body_type: comp::humanoid::ALL_BODY_TYPES[body.body_type as usize],
            hair_style: body.hair_style as u8,
            beard: body.beard as u8,
            eyes: body.eyes as u8,
            accessory: body.accessory as u8,
            hair_color: body.hair_color as u8,
            skin: body.skin as u8,
            eye_color: body.eye_color as u8,
        })
    }
}

#[derive(AsChangeset, Debug, PartialEq)]
#[primary_key(character_id)]
#[table_name = "stats"]
pub struct StatsUpdate {
    pub level: i32,
    pub exp: i32,
    pub endurance: i32,
    pub fitness: i32,
    pub willpower: i32,
    pub skills: SkillSetData,
}

impl From<&comp::Stats> for StatsUpdate {
    fn from(stats: &comp::Stats) -> StatsUpdate {
        StatsUpdate {
            level: stats.level.level() as i32,
            exp: stats.exp.current() as i32,
            endurance: stats.endurance as i32,
            fitness: stats.fitness as i32,
            willpower: stats.willpower as i32,
            skills: SkillSetData(stats.skill_set.clone()),
        }
    }
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::comp;

    #[test]
    fn stats_update_from_stats() {
        let mut stats = comp::Stats::new(
            String::from("Test"),
            comp::Body::Humanoid(comp::humanoid::Body::random()),
        );

        stats.level.set_level(2);
        stats.exp.set_current(20);

        stats.endurance = 2;
        stats.fitness = 3;
        stats.willpower = 4;

        assert_eq!(StatsUpdate::from(&stats), StatsUpdate {
            level: 2,
            exp: 20,
            endurance: 2,
            fitness: 3,
            willpower: 4,
            skills: SkillSetData(stats.skill_set)
        })
    }

    #[test]
    fn loads_stats_with_correct_level() {
        let data = StatsJoinData {
            alias: "test",
            body: &comp::Body::from(&Body {
                character_id: 0,
                species: 0,
                body_type: comp::humanoid::BodyType::Female as i16,
                hair_style: 0,
                beard: 0,
                eyes: 0,
                accessory: 0,
                hair_color: 0,
                skin: 0,
                eye_color: 0,
            }),
            stats: &Stats {
                character_id: 0,
                level: 3,
                exp: 70,
                endurance: 0,
                fitness: 2,
                willpower: 3,
                skills: SkillSetData(comp::SkillSet::new()),
            },
        };

        let stats = comp::Stats::from(data);

        assert_eq!(stats.level.level(), 3);
        assert_eq!(stats.exp.current(), 70);
    }
}
