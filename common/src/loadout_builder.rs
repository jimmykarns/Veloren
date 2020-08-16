use crate::{
    assets,
    comp::{
        item::{Item, ItemKind},
        Alignment, Body, biped_large, CharacterAbility, ItemConfig, Loadout,
    },
};
use rand::Rng;
use std::time::Duration;

/// Builder for character Loadouts, containing weapon and armour items belonging
/// to a character, along with some helper methods for loading Items and
/// ItemConfig
///
/// ```
/// use veloren_common::LoadoutBuilder;
///
/// // Build a loadout with character starter defaults and a specific sword with default sword abilities
/// let loadout = LoadoutBuilder::new()
///     .defaults()
///     .active_item(LoadoutBuilder::default_item_config_from_str(
///         Some("common.items.weapons.sword.zweihander_sword_0"),
///     ))
///     .build();
/// ```
pub struct LoadoutBuilder(Loadout);

impl LoadoutBuilder {
    #[allow(clippy::new_without_default)] // TODO: Pending review in #587
    pub fn new() -> Self {
        Self(Loadout {
            active_item: None,
            second_item: None,
            shoulder: None,
            chest: None,
            belt: None,
            hand: None,
            pants: None,
            foot: None,
            back: None,
            ring: None,
            neck: None,
            lantern: None,
            head: None,
            tabard: None,
        })
    }

    /// Set default armor items for the loadout. This may vary with game
    /// updates, but should be safe defaults for a new character.
    pub fn defaults(self) -> Self {
        self.chest(Some(assets::load_expect_cloned(
            "common.items.armor.starter.rugged_chest",
        )))
        .pants(Some(assets::load_expect_cloned(
            "common.items.armor.starter.rugged_pants",
        )))
        .foot(Some(assets::load_expect_cloned(
            "common.items.armor.starter.sandals_0",
        )))
        .lantern(Some(assets::load_expect_cloned(
            "common.items.armor.starter.lantern",
        )))
    }

    /// Builds loadout of creature when spawned 
    pub fn build_loadout(body: Body, alignment: Alignment, mut main_tool: Option<Item>) -> Self {
        match body {
            Body::BipedLarge(biped_large) => match biped_large.species {
                biped_large::Species::Cyclops => {
                    main_tool = Some(assets::load_expect_cloned("common.items.weapons.bossweapon.cyclops_hammer"));
                },
                _ => {},
            }
            _ => {},
        };

        let active_item =
            if let Some(ItemKind::Tool(tool)) = main_tool.as_ref().map(|i| &i.kind) {
                let mut abilities = tool.get_abilities();
                let mut ability_drain = abilities.drain(..);

                main_tool.map(|item| ItemConfig {
                    item,
                    ability1: ability_drain.next(),
                    ability2: ability_drain.next(),
                    ability3: ability_drain.next(),
                    block_ability: None,
                    dodge_ability: Some(CharacterAbility::Roll),
                })
            } else {
                Some(ItemConfig {
                    // We need the empty item so npcs can attack
                    item: assets::load_expect_cloned("common.items.weapons.empty.empty"),
                    ability1: Some(CharacterAbility::BasicMelee {
                        energy_cost: 0,
                        buildup_duration: Duration::from_millis(0),
                        recover_duration: Duration::from_millis(400),
                        base_healthchange: -40,
                        knockback: 0.0,
                        range: 3.5,
                        max_angle: 15.0,
                    }),
                    ability2: None,
                    ability3: None,
                    block_ability: None,
                    dodge_ability: None,
                })
            };

        let loadout = match body {
            Body::Humanoid(_) => match alignment {
                Alignment::Npc => Loadout {
                    active_item,
                    second_item: None,
                    shoulder: None,
                    chest: Some(assets::load_expect_cloned(
                        match rand::thread_rng().gen_range(0, 10) {
                            0 => "common.items.armor.chest.worker_green_0",
                            1 => "common.items.armor.chest.worker_green_1",
                            2 => "common.items.armor.chest.worker_red_0",
                            3 => "common.items.armor.chest.worker_red_1",
                            4 => "common.items.armor.chest.worker_purple_0",
                            5 => "common.items.armor.chest.worker_purple_1",
                            6 => "common.items.armor.chest.worker_yellow_0",
                            7 => "common.items.armor.chest.worker_yellow_1",
                            8 => "common.items.armor.chest.worker_orange_0",
                            _ => "common.items.armor.chest.worker_orange_1",
                        },
                    )),
                    belt: Some(assets::load_expect_cloned(
                        "common.items.armor.belt.leather_0",
                    )),
                    hand: None,
                    pants: Some(assets::load_expect_cloned(
                        "common.items.armor.pants.worker_blue_0",
                    )),
                    foot: Some(assets::load_expect_cloned(
                        match rand::thread_rng().gen_range(0, 2) {
                            0 => "common.items.armor.foot.leather_0",
                            _ => "common.items.armor.starter.sandals_0",
                        },
                    )),
                    back: None,
                    ring: None,
                    neck: None,
                    lantern: None,
                    head: None,
                    tabard: None,
                },
                Alignment::Enemy => Loadout {
                    active_item,
                    second_item: None,
                    shoulder: Some(assets::load_expect_cloned(
                        "common.items.armor.shoulder.cultist_shoulder_purple",
                    )),
                    chest: Some(assets::load_expect_cloned(
                        "common.items.armor.chest.cultist_chest_purple",
                    )),
                    belt: Some(assets::load_expect_cloned(
                        "common.items.armor.belt.cultist_belt",
                    )),
                    hand: Some(assets::load_expect_cloned(
                        "common.items.armor.hand.cultist_hands_purple",
                    )),
                    pants: Some(assets::load_expect_cloned(
                        "common.items.armor.pants.cultist_legs_purple",
                    )),
                    foot: Some(assets::load_expect_cloned(
                        "common.items.armor.foot.cultist_boots",
                    )),
                    back: Some(assets::load_expect_cloned(
                        "common.items.armor.back.dungeon_purple-0",
                    )),
                    ring: None,
                    neck: None,
                    lantern: Some(assets::load_expect_cloned("common.items.lantern.black_0")),
                    head: None,
                    tabard: None,
                },
                _ => LoadoutBuilder::animal(body).build(),
            },
            Body::BipedLarge(biped_large) => match biped_large.species {
                biped_large::Species::Cyclops => Loadout {
                    active_item,
                    second_item: None,
                    shoulder: None,
                    chest: None,
                    belt: None,
                    hand: None,
                    pants: None,
                    foot: None,
                    back: None,
                    ring: None,
                    neck: None,
                    lantern: None,
                    head: None,
                    tabard: None,
                },
                _ => LoadoutBuilder::animal(body).build(), 
            }
            _ => LoadoutBuilder::animal(body).build(),
        };

        Self(loadout)
    }

    /// Default animal configuration
    pub fn animal(body: Body) -> Self {
        Self(Loadout {
            active_item: Some(ItemConfig {
                item: assets::load_expect_cloned("common.items.weapons.empty.empty"),
                ability1: Some(CharacterAbility::BasicMelee {
                    energy_cost: 10,
                    buildup_duration: Duration::from_millis(600),
                    recover_duration: Duration::from_millis(100),
                    base_healthchange: -(body.base_dmg() as i32),
                    knockback: 0.0,
                    range: body.base_range(),
                    max_angle: 20.0,
                }),
                ability2: None,
                ability3: None,
                block_ability: None,
                dodge_ability: None,
            }),
            second_item: None,
            shoulder: None,
            chest: None,
            belt: None,
            hand: None,
            pants: None,
            foot: None,
            back: None,
            ring: None,
            neck: None,
            lantern: None,
            head: None,
            tabard: None,
        })
    }

    /// Get the default [ItemConfig](../comp/struct.ItemConfig.html) for a tool
    /// (weapon). This information is required for the `active` and `second`
    /// weapon items in a loadout. If some customisation to the item's
    /// abilities or their timings is desired, you should create and provide
    /// the item config directly to the [active_item](#method.active_item)
    /// method
    pub fn default_item_config_from_item(maybe_item: Option<Item>) -> Option<ItemConfig> {
        if let Some(item) = maybe_item {
            if let ItemKind::Tool(tool) = &item.kind {
                let mut abilities = tool.get_abilities();
                let mut ability_drain = abilities.drain(..);

                return Some(ItemConfig {
                    item,
                    ability1: ability_drain.next(),
                    ability2: ability_drain.next(),
                    ability3: ability_drain.next(),
                    block_ability: Some(CharacterAbility::BasicBlock),
                    dodge_ability: Some(CharacterAbility::Roll),
                });
            }
        }

        None
    }

    /// Get an [Item](../comp/struct.Item.html) by its string
    /// reference by loading its asset
    pub fn item_from_str(item_ref: Option<&str>) -> Option<Item> {
        item_ref.and_then(|specifier| assets::load_cloned::<Item>(&specifier).ok())
    }

    /// Get an item's (weapon's) default
    /// [ItemConfig](../comp/struct.ItemConfig.html)
    /// by string reference. This will first attempt to load the Item, then
    /// the default abilities for that item via the
    /// [default_item_config_from_item](#method.default_item_config_from_item)
    /// function
    pub fn default_item_config_from_str(item_ref: Option<&str>) -> Option<ItemConfig> {
        Self::default_item_config_from_item(Self::item_from_str(item_ref))
    }

    pub fn active_item(mut self, item: Option<ItemConfig>) -> Self {
        self.0.active_item = item;

        self
    }

    pub fn second_item(mut self, item: Option<ItemConfig>) -> Self {
        self.0.second_item = item;

        self
    }

    pub fn shoulder(mut self, item: Option<Item>) -> Self {
        self.0.shoulder = item;
        self
    }

    pub fn chest(mut self, item: Option<Item>) -> Self {
        self.0.chest = item;
        self
    }

    pub fn belt(mut self, item: Option<Item>) -> Self {
        self.0.belt = item;
        self
    }

    pub fn hand(mut self, item: Option<Item>) -> Self {
        self.0.hand = item;
        self
    }

    pub fn pants(mut self, item: Option<Item>) -> Self {
        self.0.pants = item;
        self
    }

    pub fn foot(mut self, item: Option<Item>) -> Self {
        self.0.foot = item;
        self
    }

    pub fn back(mut self, item: Option<Item>) -> Self {
        self.0.back = item;
        self
    }

    pub fn ring(mut self, item: Option<Item>) -> Self {
        self.0.ring = item;
        self
    }

    pub fn neck(mut self, item: Option<Item>) -> Self {
        self.0.neck = item;
        self
    }

    pub fn lantern(mut self, item: Option<Item>) -> Self {
        self.0.lantern = item;
        self
    }

    pub fn head(mut self, item: Option<Item>) -> Self {
        self.0.head = item;
        self
    }

    pub fn tabard(mut self, item: Option<Item>) -> Self {
        self.0.tabard = item;
        self
    }

    pub fn build(self) -> Loadout { self.0 }
}
