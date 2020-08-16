//! Database operations related to character data
//!
//! Methods in this module should remain private - database updates and loading
//! are communicated via requests to the [`CharacterLoader`] and
//! [`CharacterUpdater`] while results/responses are polled and handled each
//! server tick.

extern crate diesel;

use super::{error::Error, establish_connection, models::*, schema};
use crate::{
    comp,
    persistence::{
        conversions::{
            convert_character_from_database, convert_inventory_from_database_items,
            convert_inventory_to_database_items, convert_loadout_from_database_items,
            convert_loadout_to_database_items, convert_stats_from_database,
            convert_stats_to_database,
        },
        error::Error::DatabaseError,
    },
};
use common::character::{CharacterId, CharacterItem, MAX_CHARACTERS_PER_PLAYER};
use crossbeam::{channel, channel::TryIter};
use diesel::prelude::*;
use std::sync::atomic::Ordering;
use tracing::{error, info, warn};

pub(crate) type EntityId = i64;

/// A tuple of the components that are persisted to the DB for each character
pub type PersistedComponents = (comp::Body, comp::Stats, comp::Inventory, comp::Loadout);

type CharacterListResult = Result<Vec<CharacterItem>, Error>;
type CharacterDataResult = Result<PersistedComponents, Error>;
type CharacterLoaderRequest = (specs::Entity, CharacterLoaderRequestKind);

const CHARACTER_PSEUDO_CONTAINER_DEF_ID: &str = "veloren.core.pseudo_containers.character";
const INVENTORY_PSEUDO_CONTAINER_DEF_ID: &str = "veloren.core.pseudo_containers.inventory";
const LOADOUT_PSEUDO_CONTAINER_DEF_ID: &str = "veloren.core.pseudo_containers.loadout";
const WORLD_PSEUDO_CONTAINER_ID: EntityId = 1;

/// Available database operations when modifying a player's character list
enum CharacterLoaderRequestKind {
    CreateCharacter {
        player_uuid: String,
        character_alias: String,
        persisted_components: PersistedComponents,
    },
    DeleteCharacter {
        player_uuid: String,
        character_id: CharacterId,
    },
    LoadCharacterList {
        player_uuid: String,
    },
    LoadCharacterData {
        player_uuid: String,
        character_id: CharacterId,
    },
}

/// Wrapper for results for character actions. Can be a list of
/// characters, or component data belonging to an individual character
#[derive(Debug)]
pub enum CharacterLoaderResponseType {
    CharacterList(CharacterListResult),
    CharacterData(Box<CharacterDataResult>),
}

/// Common message format dispatched in response to an update request
#[derive(Debug)]
pub struct CharacterLoaderResponse {
    pub entity: specs::Entity,
    pub result: CharacterLoaderResponseType,
}

/// A bi-directional messaging resource for making requests to modify or load
/// character data in a background thread.
///
/// This is used on the character selection screen, and after character
/// selection when loading the components associated with a character.
///
/// Requests messages are sent in the form of
/// [`CharacterLoaderRequestKind`] and are dispatched at the character select
/// screen.
///
/// Responses are polled on each server tick in the format
/// [`CharacterLoaderResponse`]
pub struct CharacterLoader {
    update_rx: Option<channel::Receiver<CharacterLoaderResponse>>,
    update_tx: Option<channel::Sender<CharacterLoaderRequest>>,
    handle: Option<std::thread::JoinHandle<()>>,
}

impl CharacterLoader {
    pub fn new(db_dir: String) -> Self {
        let (update_tx, internal_rx) = channel::unbounded::<CharacterLoaderRequest>();
        let (internal_tx, update_rx) = channel::unbounded::<CharacterLoaderResponse>();

        let handle = std::thread::spawn(move || {
            while let Ok(request) = internal_rx.recv() {
                let (entity, kind) = request;

                if let Err(e) = internal_tx.send(CharacterLoaderResponse {
                    entity,
                    result: match kind {
                        CharacterLoaderRequestKind::CreateCharacter {
                            player_uuid,
                            character_alias,
                            persisted_components,
                        } => CharacterLoaderResponseType::CharacterList(create_character(
                            &player_uuid,
                            &character_alias,
                            persisted_components,
                            &db_dir,
                        )),
                        CharacterLoaderRequestKind::DeleteCharacter {
                            player_uuid,
                            character_id,
                        } => CharacterLoaderResponseType::CharacterList(delete_character(
                            &player_uuid,
                            character_id,
                            &db_dir,
                        )),
                        CharacterLoaderRequestKind::LoadCharacterList { player_uuid } => {
                            CharacterLoaderResponseType::CharacterList(load_character_list(
                                &player_uuid,
                                &db_dir,
                            ))
                        },
                        CharacterLoaderRequestKind::LoadCharacterData {
                            player_uuid,
                            character_id,
                        } => CharacterLoaderResponseType::CharacterData(Box::new(
                            load_character_data(player_uuid, character_id, &db_dir),
                        )),
                    },
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

    /// Create a new character belonging to the player identified by
    /// `player_uuid`
    pub fn create_character(
        &self,
        entity: specs::Entity,
        player_uuid: String,
        character_alias: String,
        persisted_components: PersistedComponents,
    ) {
        if let Err(e) = self.update_tx.as_ref().unwrap().send((
            entity,
            CharacterLoaderRequestKind::CreateCharacter {
                player_uuid,
                character_alias,
                persisted_components,
            },
        )) {
            error!(?e, "Could not send character creation request");
        }
    }

    /// Delete a character by `id` and `player_uuid`
    pub fn delete_character(
        &self,
        entity: specs::Entity,
        player_uuid: String,
        character_id: CharacterId,
    ) {
        if let Err(e) = self.update_tx.as_ref().unwrap().send((
            entity,
            CharacterLoaderRequestKind::DeleteCharacter {
                player_uuid,
                character_id,
            },
        )) {
            error!(?e, "Could not send character deletion request");
        }
    }

    /// Loads a list of characters belonging to the player identified by
    /// `player_uuid`
    pub fn load_character_list(&self, entity: specs::Entity, player_uuid: String) {
        if let Err(e) = self
            .update_tx
            .as_ref()
            .unwrap()
            .send((entity, CharacterLoaderRequestKind::LoadCharacterList {
                player_uuid,
            }))
        {
            error!(?e, "Could not send character list load request");
        }
    }

    /// Loads components associated with a character
    pub fn load_character_data(
        &self,
        entity: specs::Entity,
        player_uuid: String,
        character_id: CharacterId,
    ) {
        if let Err(e) = self.update_tx.as_ref().unwrap().send((
            entity,
            CharacterLoaderRequestKind::LoadCharacterData {
                player_uuid,
                character_id,
            },
        )) {
            error!(?e, "Could not send character data load request");
        }
    }

    /// Returns a non-blocking iterator over CharacterLoaderResponse messages
    pub fn messages(&self) -> TryIter<CharacterLoaderResponse> {
        self.update_rx.as_ref().unwrap().try_iter()
    }
}

impl Drop for CharacterLoader {
    fn drop(&mut self) {
        drop(self.update_tx.take());
        if let Err(e) = self.handle.take().unwrap().join() {
            error!(?e, "Error from joining character loader thread");
        }
    }
}

/// Load stored data for a character.
///
/// After first logging in, and after a character is selected, we fetch this
/// data for the purpose of inserting their persisted data for the entity.
fn load_character_data(
    requesting_player_uuid: String,
    char_id: CharacterId,
    db_dir: &str,
) -> CharacterDataResult {
    use schema::{character::dsl::*, item::dsl::*, stats::dsl::*};
    let connection = establish_connection(db_dir)?;

    // TODO: Store the character's pseudo-container IDs during login so we don't
    // have to fetch them each save?
    let inventory_container_id =
        get_pseudo_container_id(&connection, char_id, INVENTORY_PSEUDO_CONTAINER_DEF_ID)?;

    let loadout_container_id =
        get_pseudo_container_id(&connection, char_id, LOADOUT_PSEUDO_CONTAINER_DEF_ID)?;

    let inventory_items = item
        .filter(parent_container_item_id.eq(inventory_container_id))
        .load::<Item>(&connection)?;

    let loadout_items = item
        .filter(parent_container_item_id.eq(loadout_container_id))
        .load::<Item>(&connection)?;

    let (character_data, stats_data) = character
        .filter(
            character_id
                .eq(char_id)
                .and(player_uuid.eq(requesting_player_uuid)),
        )
        .inner_join(stats)
        .first::<(Character, Stats)>(&connection)?;

    let body = comp::Body::Humanoid(comp::body::humanoid::Body::random()); // TODO: actual body
    Ok((
        body,
        convert_stats_from_database(&stats_data, character_data.alias),
        convert_inventory_from_database_items(&inventory_items),
        convert_loadout_from_database_items(&loadout_items),
    ))
}

/// Loads a list of characters belonging to the player. This data is a small
/// subset of the character's data, and is used to render the character and
/// their level in the character list.
///
/// In the event that a join fails, for a character (i.e. they lack an entry for
/// stats, body, etc...) the character is skipped, and no entry will be
/// returned.
fn load_character_list(player_uuid_: &str, db_dir: &str) -> CharacterListResult {
    use schema::{character::dsl::*, item::dsl::*, stats::dsl::*};

    let connection = establish_connection(db_dir)?;

    let result = character
        .filter(player_uuid.eq(player_uuid_))
        .inner_join(stats)
        .order(id.desc())
        .load::<(Character, Stats)>(&connection);

    match result {
        Ok(data) => Ok(data
            .iter()
            .map(|(character_data, char_stats)| {
                // TODO: Database failures here should skip the character, not crash the server
                let char = convert_character_from_database(character_data);
                let char_body = comp::Body::Humanoid(comp::body::humanoid::Body::random());

                let loadout_container_id =
                    get_pseudo_container_id(&connection, char.id.unwrap(), LOADOUT_PSEUDO_CONTAINER_DEF_ID)
                        .expect("failed to get loadout container for character");
                let loadout_items = item
                    .filter(parent_container_item_id.eq(loadout_container_id))
                    .load::<Item>(&connection)
                    .expect("failed to fetch loadout items for character");

                let loadout = convert_loadout_from_database_items(&loadout_items);

                CharacterItem {
                    character: char,
                    body: char_body,
                    level: char_stats.level as usize,
                    loadout,
                }
            })
            .collect()),
        Err(e) => {
            error!(?e, ?player_uuid, "Failed to load character list for player");
            Err(Error::CharacterDataError)
        },
    }
}

/// Create a new character with provided comp::Character and comp::Body data.
///
/// Note that sqlite does not support returning the inserted data after a
/// successful insert. To workaround, we wrap this in a transaction which
/// inserts, queries for the newly created character id, then uses the character
/// id for subsequent insertions
fn create_character(
    uuid: &str,
    character_alias: &str,
    persisted_components: PersistedComponents,
    db_dir: &str,
) -> CharacterListResult {
    use schema::item::dsl::*;

    check_character_limit(uuid, db_dir)?;

    let connection = establish_connection(db_dir)?;

    connection.transaction::<_, diesel::result::Error, _>(|| {
        use schema::{body, character, stats};

        let (body, stats, inventory, loadout) = persisted_components;
        match body {
            comp::Body::Humanoid(body_data) => {
                let character_id = get_new_entity_id(&connection)?;

                // Insert character record
                let new_character = NewCharacter {
                    id: character_id,
                    player_uuid: uuid,
                    alias: &character_alias,
                };
                diesel::insert_into(character::table)
                    .values(&new_character)
                    .execute(&connection)?;

                // Create pseudo-container items for character
                let inventory_container_id = get_new_entity_id(&connection)?;
                let loadout_container_id = get_new_entity_id(&connection)?;
                let pseudo_containers = vec![
                    NewItem {
                        stack_size: None,
                        item_id: Some(character_id),
                        parent_container_item_id: WORLD_PSEUDO_CONTAINER_ID,
                        item_definition_id: CHARACTER_PSEUDO_CONTAINER_DEF_ID.to_owned(),
                        position: None,
                    },
                    NewItem {
                        stack_size: None,
                        item_id: Some(inventory_container_id),
                        parent_container_item_id: character_id,
                        item_definition_id: INVENTORY_PSEUDO_CONTAINER_DEF_ID.to_owned(),
                        position: None,
                    },
                    NewItem {
                        stack_size: None,
                        item_id: Some(loadout_container_id),
                        parent_container_item_id: character_id,
                        item_definition_id: LOADOUT_PSEUDO_CONTAINER_DEF_ID.to_owned(),
                        position: None,
                    },
                ];
                diesel::insert_into(item)
                    .values(pseudo_containers)
                    .execute(&connection)?;

                // Insert stats record
                let db_stats = convert_stats_to_database(character_id, &stats);
                diesel::insert_into(stats::table)
                    .values(&db_stats)
                    .execute(&connection)?;

                // Insert default inventory and loadout item records
                let mut item_pairs =
                    convert_inventory_to_database_items(inventory, inventory_container_id);
                item_pairs.extend(convert_loadout_to_database_items(
                    loadout,
                    loadout_container_id,
                ));

                for mut item_pair in item_pairs.into_iter() {
                    let id = get_new_entity_id(&connection)?;
                    item_pair.model.item_id = Some(id);
                    diesel::insert_into(item)
                        .values(item_pair.model)
                        .execute(&connection)?;
                }

                // Insert body record
                let new_body = Body {
                    character_id,
                    species: body_data.species as i16,
                    body_type: body_data.body_type as i16,
                    hair_style: body_data.hair_style as i16,
                    beard: body_data.beard as i16,
                    eyes: body_data.eyes as i16,
                    accessory: body_data.accessory as i16,
                    hair_color: body_data.hair_color as i16,
                    skin: body_data.skin as i16,
                    eye_color: body_data.eye_color as i16,
                };
                diesel::insert_into(body::table)
                    .values(&new_body)
                    .execute(&connection)?;
            },
            _ => warn!("Creating non-humanoid characters is not supported."),
        };

        Ok(())
    })?;

    load_character_list(uuid, db_dir)
}

/// Delete a character. Returns the updated character list.
fn delete_character(uuid: &str, character_id: CharacterId, db_dir: &str) -> CharacterListResult {
    use schema::character::dsl::*;

    let connection = establish_connection(db_dir)?;
    connection.transaction::<_, diesel::result::Error, _>(|| {
        diesel::delete(
            character
                .filter(id.eq(character_id))
                .filter(player_uuid.eq(uuid)),
        )
        .execute(&connection)?;

        Ok(())
    })?;

    load_character_list(uuid, db_dir)
}

/// Before creating a character, we ensure that the limit on the number of
/// characters has not been exceeded
fn check_character_limit(uuid: &str, db_dir: &str) -> Result<(), Error> {
    use diesel::dsl::count_star;
    use schema::character::dsl::*;

    let character_count = character
        .select(count_star())
        .filter(player_uuid.eq(uuid))
        .load::<i64>(&establish_connection(db_dir)?)?;

    match character_count.first() {
        Some(count) => {
            if count < &(MAX_CHARACTERS_PER_PLAYER as i64) {
                Ok(())
            } else {
                Err(Error::CharacterLimitReached)
            }
        },
        _ => Ok(()),
    }
}

type CharacterUpdateData = (comp::Stats, comp::Inventory, comp::Loadout);

/// A unidirectional messaging resource for saving characters in a
/// background thread.
///
/// This is used to make updates to a character and their persisted components,
/// such as inventory, loadout, etc...
pub struct CharacterUpdater {
    update_tx: Option<channel::Sender<Vec<(CharacterId, CharacterUpdateData)>>>,
    handle: Option<std::thread::JoinHandle<()>>,
}

impl CharacterUpdater {
    pub fn new(db_dir: String) -> Self {
        let (update_tx, update_rx) =
            channel::unbounded::<Vec<(CharacterId, CharacterUpdateData)>>();
        let handle = std::thread::spawn(move || {
            while let Ok(updates) = update_rx.recv() {
                info!("Persistence batch update starting");
                batch_update(updates.into_iter(), &db_dir);
                info!("Persistence batch update finished");
            }
        });

        Self {
            update_tx: Some(update_tx),
            handle: Some(handle),
        }
    }

    /// Updates a collection of characters based on their id and components
    pub fn batch_update<'a>(
        &self,
        updates: impl Iterator<
            Item = (
                CharacterId,
                &'a comp::Stats,
                &'a comp::Inventory,
                &'a comp::Loadout,
            ),
        >,
    ) {
        let updates = updates
            .map(|(character_id, stats, inventory, loadout)| {
                (
                    character_id,
                    (stats.clone(), inventory.clone(), loadout.clone()),
                )
            })
            .collect::<Vec<(CharacterId, (comp::Stats, comp::Inventory, comp::Loadout))>>();

        if let Err(e) = self.update_tx.as_ref().unwrap().send(updates) {
            error!(?e, "Could not send stats updates");
        }
    }

    /// Updates a single character based on their id and components
    pub fn update(
        &self,
        character_id: CharacterId,
        stats: &comp::Stats,
        inventory: &comp::Inventory,
        loadout: &comp::Loadout,
    ) {
        self.batch_update(std::iter::once((character_id, stats, inventory, loadout)));
    }
}

fn batch_update(updates: impl Iterator<Item = (CharacterId, CharacterUpdateData)>, db_dir: &str) {
    let connection = establish_connection(db_dir);

    if let Err(e) = connection.and_then(|connection| {
        connection.transaction::<_, diesel::result::Error, _>(|| {
            updates.for_each(|(character_id, (stats, inventory, loadout))| {
                // Create a nested transaction (savepoint) per character update so that a single
                // error for a particular character doesn't prevent all other characters being
                // saved
                if let Err(e) = connection.transaction::<_, Error, _>(|| {
                    update(character_id, stats, inventory, loadout, &connection)
                }) {
                    error!(?character_id, ?e, "Persistence update failed for character");
                }
            });

            Ok(())
        })
    }) {
        error!(?e, "Error during stats batch update transaction");
    }
}

fn get_new_entity_id(conn: &SqliteConnection) -> Result<EntityId, diesel::result::Error> {
    use super::schema::entity::dsl::*;

    diesel::insert_into(entity).default_values().execute(conn)?;

    let new_entity_id = entity
        .order(entity_id.desc())
        .select(entity_id)
        .first::<EntityId>(conn)?;

    info!("Created new persistence entity_id: {}", new_entity_id);
    Ok(new_entity_id)
}

fn get_pseudo_container_id(
    connection: &SqliteConnection,
    character_id: CharacterId,
    pseudo_container_id: &str,
) -> Result<EntityId, Error> {
    use super::schema::item::dsl::*;
    match item
        .select(item_id)
        .filter(
            parent_container_item_id
                .eq(character_id)
                .and(item_definition_id.eq(pseudo_container_id)),
        )
        .first::<EntityId>(connection)
    {
        Ok(id) => Ok(id),
        Err(e) => {
            error!(
                ?e,
                ?character_id,
                ?pseudo_container_id,
                "Failed to retrieve pseudo container ID"
            );
            Err(DatabaseError(e))
        },
    }
}

/// NOTE: Only call while a transaction is held!
fn update(
    char_id: CharacterId,
    char_stats: comp::Stats,
    inventory: comp::Inventory,
    loadout: comp::Loadout,
    connection: &SqliteConnection,
) -> Result<(), Error> {
    use super::schema::{item::dsl::*, stats::dsl::*};

    // TODO: Store the character's pseudo-container IDs during login so we don't
    // have to fetch them each save?
    let inventory_container_id =
        get_pseudo_container_id(connection, char_id, INVENTORY_PSEUDO_CONTAINER_DEF_ID)?;

    let loadout_container_id =
        get_pseudo_container_id(connection, char_id, LOADOUT_PSEUDO_CONTAINER_DEF_ID)?;

    let mut item_pairs = convert_inventory_to_database_items(inventory, inventory_container_id);
    item_pairs.extend(convert_loadout_to_database_items(
        loadout,
        loadout_container_id,
    ));

    // Fetch all existing items from the database for the character so that we can use it to keep
    // track of which items still exist and which don't and should be deleted from the database.
    let mut existing_items = item
        .filter(
            parent_container_item_id
                .eq(inventory_container_id)
                .or(parent_container_item_id.eq(loadout_container_id)),
        )
        .load::<Item>(connection)?;

    // TODO: Refactor this, batch into multiple inserts/updates etc
    for mut item_pair in item_pairs.into_iter() {
        if let Some(model_item_id) = item_pair.model.item_id {
            // Remove each item that is saved from the list of items to delete
            existing_items.retain(|x| x.item_id != model_item_id);

            diesel::update(item.filter(item_id.eq(model_item_id)))
                .set(item_pair.model)
                .execute(connection)?;
        } else {
            let id = get_new_entity_id(connection)?;
            item_pair.model.item_id = Some(id);

            // TODO: Fix this cast.
            // TODO: Set this for all items only after the sub-transaction for this
            // character has succeeded
            // Set the item_id inside the Arc to the new
            // entity_id - this results in the original item_id on the Item
            // instance on the main game thread being updated.
            item_pair.comp.item_id.store(id as u64, Ordering::Relaxed);

            diesel::insert_into(item)
                .values(item_pair.model)
                .execute(connection)?;
        }
    }

    // Any items left in existing_items after saving the character's inventory and loadout must
    // no longer exist (consumed, dropped, etc) so should be deleted from the database.
    for existing_item in existing_items {
        // TODO: Single delete statement using all item IDs in existing_items
        diesel::delete(item.filter(item_id.eq(existing_item.item_id))).execute(connection)?;
    }

    let db_stats = convert_stats_to_database(char_id, &char_stats);
    diesel::update(stats.filter(character_id.eq(char_id)))
        .set(db_stats)
        .execute(connection)?;

    Ok(())
}

impl Drop for CharacterUpdater {
    fn drop(&mut self) {
        drop(self.update_tx.take());
        if let Err(e) = self.handle.take().unwrap().join() {
            error!(?e, "Error from joining character update thread");
        }
    }
}
