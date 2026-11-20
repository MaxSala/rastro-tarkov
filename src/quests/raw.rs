//! The shape of tarkov.dev's task data, as the GraphQL API answers it. json.tarkov.dev's static
//! files are put into this same shape first (see `jsondoc`). Every field is optional: whatever is
//! missing or has the wrong type is simply ignored by the conversion in `query`.

use serde::Deserialize;

#[derive(Deserialize)]
pub struct Response {
    pub data: Option<Data>,
}

#[derive(Deserialize)]
pub struct Data {
    #[serde(default)]
    pub tasks: Vec<Option<RawTask>>,
}

#[derive(Deserialize, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct RawTask {
    pub id: Option<String>,
    pub name: Option<String>,
    pub normalized_name: Option<String>,
    pub wiki_link: Option<String>,
    pub task_image_link: Option<String>,
    pub min_player_level: Option<f64>,
    pub experience: Option<f64>,
    pub kappa_required: Option<bool>,
    pub lightkeeper_required: Option<bool>,
    pub faction_name: Option<String>,
    pub trader: Option<Named>,
    pub map: Option<Named>,
    pub task_requirements: Vec<Option<RawRequirement>>,
    pub trader_requirements: Vec<Option<RawTraderReq>>,
    pub objectives: Vec<Option<RawObjective>>,
    pub finish_rewards: Option<RawRewards>,
}

/// A trader or a map: whichever of these fields the query asked for.
#[derive(Deserialize, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct Named {
    pub id: Option<String>,
    pub normalized_name: Option<String>,
    pub name: Option<String>,
}

#[derive(Deserialize, Default)]
#[serde(default)]
pub struct RawRequirement {
    pub task: Option<RawItem>,
    pub status: Vec<Option<String>>,
}

#[derive(Deserialize, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct RawTraderReq {
    pub trader: Option<Named>,
    pub requirement_type: Option<String>,
    pub value: Option<f64>,
}

#[derive(Deserialize, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct RawObjective {
    pub id: Option<String>,
    #[serde(rename = "type")]
    pub kind: Option<String>,
    pub description: Option<String>,
    pub optional: Option<bool>,
    pub maps: Vec<Option<Named>>,
    pub count: Option<f64>,
    pub found_in_raid: Option<bool>,
    pub items: Option<Vec<Option<RawItem>>>,
    pub item: Option<RawItem>,
    pub marker_item: Option<RawItem>,
    pub quest_item: Option<RawItem>,
    pub zones: Option<Vec<Option<RawZone>>>,
    pub possible_locations: Option<Vec<Option<RawLocations>>>,
    pub required_keys: Option<Vec<Option<Vec<Option<RawItem>>>>>,
}

#[derive(Deserialize, Default)]
#[serde(default)]
pub struct RawItem {
    pub id: Option<String>,
    pub name: Option<String>,
}

#[derive(Deserialize, Default)]
#[serde(default)]
pub struct RawZone {
    pub map: Option<Named>,
    pub position: Option<RawPos>,
    pub outline: Option<Vec<Option<RawPos>>>,
    /// Height of the zone's floor and ceiling.
    pub top: Option<f64>,
    pub bottom: Option<f64>,
}

#[derive(Deserialize, Default)]
#[serde(default)]
pub struct RawLocations {
    pub map: Option<Named>,
    pub positions: Option<Vec<Option<RawPos>>>,
}

#[derive(Deserialize, Default, Clone, Copy)]
#[serde(default)]
pub struct RawPos {
    pub x: Option<f64>,
    pub y: Option<f64>,
    pub z: Option<f64>,
}

/// What finishing the quest gives (`finishRewards`).
#[derive(Deserialize, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct RawRewards {
    pub trader_standing: Vec<Option<RawStanding>>,
    pub items: Vec<Option<RawCount>>,
    pub offer_unlock: Vec<Option<RawOffer>>,
    pub skill_level_reward: Vec<Option<RawSkill>>,
    pub trader_unlock: Vec<Option<Named>>,
    pub craft_unlock: Vec<Option<RawCraft>>,
}

#[derive(Deserialize, Default)]
#[serde(default)]
pub struct RawStanding {
    pub trader: Option<Named>,
    pub standing: Option<f64>,
}

#[derive(Deserialize, Default)]
#[serde(default)]
pub struct RawCount {
    pub item: Option<RawItem>,
    pub count: Option<f64>,
}

#[derive(Deserialize, Default)]
#[serde(default)]
pub struct RawOffer {
    pub trader: Option<Named>,
    pub level: Option<f64>,
    pub item: Option<RawItem>,
}

#[derive(Deserialize, Default)]
#[serde(default)]
pub struct RawSkill {
    pub name: Option<String>,
    pub level: Option<f64>,
}

#[derive(Deserialize, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct RawCraft {
    pub level: Option<f64>,
    pub reward_items: Vec<Option<RawCount>>,
}
