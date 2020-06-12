extern crate diesel;

use super::{
    error::Error,
    establish_connection,
    models::{Achievement as AchievementModel, DataMigration, NewAchievement, NewDataMigration},
    schema,
};
use common::achievement::*;
use diesel::{
    prelude::*,
    result::{DatabaseErrorKind, Error as DieselError},
};
use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};
use tracing::{info, warn};

pub fn sync(db_dir: &str) -> Result<(), Error> {
    let achievements = load_data();
    let connection = establish_connection(db_dir);

    // Get a hash of the Vec<Achievement> and compare to the migration table
    let result = schema::data_migration::dsl::data_migration
        .filter(schema::data_migration::title.eq(String::from("achievements")))
        .load::<DataMigration>(&connection)?;

    // First check whether the table has an entry for this data type
    if result.is_empty() {
        let migration = NewDataMigration {
            title: "achievements",
            checksum: &hash(&achievements).to_string(),
            last_run: chrono::Utc::now().naive_utc(),
        };

        diesel::insert_into(schema::data_migration::table)
            .values(&migration)
            .execute(&connection)?;
    }

    // Also check checksum. Bail if same, continue if changed
    if result.is_empty() {
        info!("Achievements need updating...");

        // Use the full dataset for checks
        let persisted_achievements =
            schema::achievement::dsl::achievement.load::<AchievementModel>(&connection)?;

        // Make use of the unique constraint in the DB, attempt to insert, on unique
        // failure check if it needs updating and do so if necessary
        for item in &achievements {
            let new_item = NewAchievement::from(item);

            if let Err(error) = diesel::insert_into(schema::achievement::table)
                .values(&new_item)
                .execute(&connection)
            {
                match error {
                    DieselError::DatabaseError(DatabaseErrorKind::UniqueViolation, _) => {
                        let entry = persisted_achievements
                            .iter()
                            .find(|&a| &a.checksum == &new_item.checksum);

                        if let Some(existing_item) = entry {
                            if existing_item.details != new_item.details {
                                match diesel::update(
                                    schema::achievement::dsl::achievement.filter(
                                        schema::achievement::checksum
                                            .eq(String::from(&existing_item.checksum)),
                                    ),
                                )
                                .set(schema::achievement::details.eq(new_item.details))
                                .execute(&connection)
                                {
                                    Ok(_) => warn!(?existing_item.checksum, "Updated achievement"),
                                    Err(err) => return Err(Error::DatabaseError(err)),
                                }
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

    Ok(())
}

fn load_data() -> Vec<AchievementItem> {
    if let Ok(path) = std::fs::canonicalize("../data/achievements.ron") {
        let path = std::path::PathBuf::from(path);

        info!(?path, "Path: ");

        match std::fs::File::open(path) {
            Ok(file) => ron::de::from_reader(file).expect("Error parsing achievement data"),
            Err(error) => panic!(error.to_string()),
        }
    } else {
        Vec::new()
    }
}

pub fn hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}
