use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Hash, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum Kind {
    Ammunition,
    Armor,
    Backpack,
    Barter,
    Clothing,
    Common,
    Container,
    Firearm,
    Food,
    Grenade,
    Headphone,
    Key,
    Magazine,
    Map,
    Medical,
    Melee,
    Modification,
    #[serde(rename = "modificationBarrel")]
    Barrel,
    #[serde(rename = "modificationBipod")]
    Bipod,
    #[serde(rename = "modificationCharge")]
    Charge,
    #[serde(rename = "modificationDevice")]
    DeviceMod,
    #[serde(rename = "modificationForegrip")]
    Foregrip,
    #[serde(rename = "modificationGasblock")]
    Gasblock,
    #[serde(rename = "modificationGoggles")]
    Goggles,
    #[serde(rename = "modificationHandguard")]
    Handguard,
    #[serde(rename = "modificationLauncher")]
    Launcher,
    #[serde(rename = "modificationMount")]
    Mount,
    #[serde(rename = "modificationMuzzle")]
    Muzzle,
    #[serde(rename = "modificationPistolgrip")]
    Pistolgrip,
    #[serde(rename = "modificationReceiver")]
    Receiver,
    #[serde(rename = "modificationSight")]
    Sight,
    #[serde(rename = "modificationSightSpecial")]
    SightSpecial,
    #[serde(rename = "modificationStock")]
    Stock,
    Money,
    Tacticalrig,
}

impl Kind {
    pub(crate) fn to_path(&self) -> &str {
        match self {
            Kind::Ammunition => "ammunition",
            Kind::Armor => "armor",
            Kind::Backpack => "backpack",
            Kind::Barter => "barter",
            Kind::Clothing => "clothing",
            Kind::Common => "common",
            Kind::Container => "container",
            Kind::Firearm => "firearm",
            Kind::Food => "food",
            Kind::Grenade => "grenade",
            Kind::Headphone => "headphone",
            Kind::Key => "key",
            Kind::Magazine => "magazine",
            Kind::Map => "map",
            Kind::Medical => "medical",
            Kind::Melee => "melee",
            Kind::Modification => "modification",
            Kind::Barrel => "modificationBarrel",
            Kind::Bipod => "modificationBipod",
            Kind::Charge => "modificationCharge",
            Kind::DeviceMod => "modificationDevice",
            Kind::Foregrip => "modificationForegrip",
            Kind::Gasblock => "modificationGasblock",
            Kind::Goggles => "modificationGoggles",
            Kind::Handguard => "modificationHandguard",
            Kind::Launcher => "modificationLauncher",
            Kind::Mount => "modificationMount",
            Kind::Muzzle => "modificationMuzzle",
            Kind::Pistolgrip => "modificationPistolgrip",
            Kind::Receiver => "modificationReceiver",
            Kind::Sight => "modificationSight",
            Kind::SightSpecial => "modificationSightSpecial",
            Kind::Stock => "modificationStock",
            Kind::Money => "money",
            Kind::Tacticalrig => "tacticalrig"
        }
    }
}