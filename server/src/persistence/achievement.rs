extern crate diesel;

use super::{
    error::Error,
    establish_connection,
    models::{
        Achievement as AchievementModel, CharacterAchievements, DataMigration, NewDataMigration,
    },
    schema,
};
use common::comp;
use crossbeam::{channel, channel::TryIter};
use diesel::{
    prelude::*,
    result::{DatabaseErrorKind, Error as DieselError},
};
use hashbrown::HashSet;
use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};
use tracing::{error, info, warn};

/// Available database operations when modifying a player's characetr list
enum AchievementLoaderRequestKind {
    LoadCharacterAchievementList {
        entity: specs::Entity,
        character_id: i32,
    },
}

type LoadCharacterAchievementsResult = (
    specs::Entity,
    Result<HashSet<comp::CharacterAchievement>, Error>,
);

/// Wrapper for results
#[derive(Debug)]
pub enum AchievementLoaderResponse {
    LoadCharacterAchievementListResponse(LoadCharacterAchievementsResult),
}

pub struct AchievementLoader {
    update_rx: Option<channel::Receiver<AchievementLoaderResponse>>,
    update_tx: Option<channel::Sender<AchievementLoaderRequestKind>>,
    handle: Option<std::thread::JoinHandle<()>>,
}

impl AchievementLoader {
    pub fn new(db_dir: String) -> Self {
        let (update_tx, internal_rx) = channel::unbounded::<AchievementLoaderRequestKind>();
        let (internal_tx, update_rx) = channel::unbounded::<AchievementLoaderResponse>();

        let handle = std::thread::spawn(move || {
            while let Ok(request) = internal_rx.recv() {
                if let Err(e) = internal_tx.send(match request {
                    AchievementLoaderRequestKind::LoadCharacterAchievementList {
                        entity,
                        character_id,
                    } => AchievementLoaderResponse::LoadCharacterAchievementListResponse((
                        entity,
                        load_character_achievement_list(character_id, &db_dir),
                    )),
                }) {
                    error!(?e, "Could not send send persistence request");
                }
            }
        });

        Self {
            update_tx: Some(update_tx),
            update_rx: Some(update_rx),
            handle: Some(handle),
        }
    }

    pub fn load_character_achievement_list(&self, entity: specs::Entity, character_id: i32) {
        if let Err(e) = self.update_tx.as_ref().unwrap().send(
            AchievementLoaderRequestKind::LoadCharacterAchievementList {
                entity,
                character_id,
            },
        ) {
            error!(?e, "Could not send character achievement load request");
        }
    }

    /// Returns a non-blocking iterator over AchievementLoaderResponse messages
    pub fn messages(&self) -> TryIter<AchievementLoaderResponse> {
        self.update_rx.as_ref().unwrap().try_iter()
    }
}

impl Drop for AchievementLoader {
    fn drop(&mut self) {
        drop(self.update_tx.take());
        if let Err(e) = self.handle.take().unwrap().join() {
            error!(?e, "Error from joining character loader thread");
        }
    }
}

fn load_character_achievement_list(
    character_id: i32,
    db_dir: &str,
) -> Result<HashSet<comp::CharacterAchievement>, Error> {
    let character_achievements = schema::character_achievements::dsl::character_achievements
        .filter(schema::character_achievements::character_id.eq(character_id))
        .first::<CharacterAchievements>(&establish_connection(db_dir))?;

    let result: HashSet<comp::CharacterAchievement> =
        character_achievements.items.0.iter().cloned().collect();

    Ok(result)
}

pub fn sync(db_dir: &str) -> Result<Vec<comp::Achievement>, Error> {
    let achievements = load_data();
    let connection = establish_connection(db_dir);

    // Use the full dataset for checksums
    let persisted_achievements =
        schema::achievements::dsl::achievements.load::<AchievementModel>(&connection)?;

    // Get a hash of the Vec<Achievement> we have in config to compare
    let result = schema::data_migration::dsl::data_migration
        .filter(schema::data_migration::title.eq(String::from("achievements")))
        .first::<DataMigration>(&connection);

    let should_run = match result {
        Ok(migration_entry) => {
            // If these don't match, we need to sync data
            migration_entry.checksum != hash(&achievements).to_string()
        },
        Err(diesel::result::Error::NotFound) => {
            // If there was no migration entry (first run on this server) we need to run
            let migration = NewDataMigration {
                title: "achievements",
                checksum: &hash(&achievements).to_string(),
                last_run: chrono::Utc::now().naive_utc(),
            };

            diesel::insert_into(schema::data_migration::table)
                .values(&migration)
                .execute(&connection)?;

            true
        },
        Err(_) => {
            error!("Failed to run migrations"); // TODO better error messaging

            false
        },
    };

    if (should_run || persisted_achievements.is_empty()) && !achievements.is_empty() {
        // Make use of the unique constraint in the DB, attempt to insert, on unique
        // failure check if it needs updating and do so if necessary
        for item in &achievements {
            if let Err(error) = diesel::insert_into(schema::achievements::table)
                .values(item)
                .execute(&connection)
            {
                match error {
                    DieselError::DatabaseError(DatabaseErrorKind::UniqueViolation, _) => {
                        // This uuid already exists, so overwrite the data
                        if let Some(existing_item) = persisted_achievements
                            .iter()
                            .find(|&a| &a.uuid == &item.uuid)
                        {
                            match diesel::update(
                                schema::achievements::dsl::achievements
                                    .filter(schema::achievements::uuid.eq(&existing_item.uuid)),
                            )
                            .set(item)
                            .execute(&connection)
                            {
                                Ok(_) => warn!(?existing_item.uuid, "Updated achievement"),
                                Err(err) => return Err(Error::DatabaseError(err)),
                            }
                        }
                    },
                    _ => return Err(Error::DatabaseError(error)),
                }
            }
        }

        // Update the checksum for the migration
        diesel::update(schema::data_migration::dsl::data_migration)
            .filter(schema::data_migration::title.eq(String::from("achievements")))
            .set((
                schema::data_migration::checksum.eq(hash(&achievements).to_string()),
                schema::data_migration::last_run.eq(chrono::Utc::now().naive_utc()),
            ))
            .execute(&connection)?;
    } else {
        info!("No achievement updates required");
    }

    let data = schema::achievements::dsl::achievements.load::<AchievementModel>(&connection)?;

    // Ok(data)
    Ok(data.iter().map(comp::Achievement::from).collect::<_>())
}

fn load_data() -> Vec<AchievementModel> {
    // TODO this better
    let manifest_dir = format!("{}/{}", env!("CARGO_MANIFEST_DIR"), "data/achievements.ron");

    match std::fs::canonicalize(manifest_dir) {
        Ok(path) => match std::fs::File::open(path) {
            Ok(file) => ron::de::from_reader(file).expect("Error parsing achievement data"),
            Err(error) => panic!(error.to_string()),
        },
        Err(error) => {
            warn!(?error, "Unable to find achievement data file");
            Vec::new()
        },
    }
}

fn hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}
