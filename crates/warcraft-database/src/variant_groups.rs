use std::collections::{BTreeMap, HashMap, HashSet};
use std::sync::LazyLock;

use warcraft_api::{UnitKind, WarcraftObjectMeta};

use crate::{TIERED_UNIT_GROUPS, UNIT_UPGRADE_SWAPS, WARCRAFT_DATABASE};

/// A single logical unit that the game ships as several distinct unit ids —
/// leveled summon tiers (Carrion Beetle `ucs1`/`ucs2`/`ucs3`), upgrade-swaps
/// (Headhunter `ohun` upgraded into Berserker `otbk`), or a hero's duplicate
/// campaign/form ids (Alchemist `Nal2`/`Nal3`/`Nalm` behind `Nalc`). Members
/// are ordered so the `canonical` unit — the one the editor displays and that
/// edits fan out from — is last: the strongest tier, the upgraded unit, or the
/// produced (trained/sold) hero.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VariantGroup {
    members: Vec<&'static str>,
}

impl VariantGroup {
    /// Every member id, ordered weakest → strongest.
    pub fn members(&self) -> &[&'static str] {
        &self.members
    }

    /// The canonical member — the one the editor shows and the fan-out target
    /// (strongest tier / upgraded unit / produced hero). It is always last.
    /// Groups are always built with at least two members, so a member always
    /// exists.
    pub fn canonical(&self) -> &'static str {
        let strongest = self.members.last();
        strongest.copied().unwrap_or_default()
    }

    /// The weaker members an edit on the canonical fans out to (everything but
    /// the canonical itself), ordered weakest → strongest.
    pub fn weaker_members(&self) -> &[&'static str] {
        let member_count = self.members.len();
        let split_index = member_count.saturating_sub(1);
        &self.members[..split_index]
    }
}

/// The merged, validated variant groups plus the member lookups derived from
/// them. Built once from the static database; never mutated.
struct VariantRegistry {
    groups: Vec<VariantGroup>,
    group_index_by_member: HashMap<&'static str, usize>,
    canonical_by_member: HashMap<&'static str, &'static str>,
    hidden_members: HashSet<&'static str>,
}

/// A `predecessor precedes successor` link between two adjacent tiers in one
/// source chain — used to rank members weakest → strongest when chains merge.
struct PrecedenceEdge {
    predecessor: &'static str,
    successor: &'static str,
}

/// Disjoint-set over unit ids, used to merge source chains that share a member
/// (e.g. a 3-level and a 4-level summon of the same unit) into one group.
#[derive(Default)]
struct UnionFind {
    parent: HashMap<&'static str, &'static str>,
}

impl UnionFind {
    fn root(&self, node: &'static str) -> &'static str {
        let mut current = node;
        loop {
            let parent_node = self.parent.get(current).copied().unwrap_or(current);
            if parent_node == current {
                return current;
            }
            current = parent_node;
        }
    }

    fn merge(&mut self, left: &'static str, right: &'static str) {
        let left_root = self.root(left);
        let right_root = self.root(right);
        if left_root != right_root {
            self.parent.insert(left_root, right_root);
        }
    }
}

/// Accumulates the weakest → strongest source chains (from leveled summons and
/// upgrade-swaps) and resolves them into the final, merged `VariantRegistry`.
#[derive(Default)]
struct VariantGraphBuilder {
    chains: Vec<Vec<&'static str>>,
}

impl VariantGraphBuilder {
    fn add_chain(&mut self, chain: Vec<&'static str>) {
        if chain.len() >= 2 {
            self.chains.push(chain);
        }
    }

    /// Collapses a hero's duplicate ids (campaign-level variants, named-hero and
    /// transform forms — e.g. Alchemist `Nal2`/`Nal3`/`Nalm` behind `Nalc`,
    /// Tinker `Nrob` behind `Ntin`) into one entry. Heroes that share a display
    /// name are one group; the canonical is the single id that is actually
    /// produced — trained at an altar or sold at a tavern — which is the real
    /// playable hero, an authoritative signal rather than a guess. A name group
    /// with no produced member (campaign-only heroes) or more than one is left
    /// alone, since the canonical would be ambiguous. The canonical is placed
    /// last so it ranks as the group's representative.
    fn add_hero_name_groups(&mut self) {
        let mut produced_unit_ids: HashSet<&'static str> = HashSet::new();
        for (_object_id, warcraft_object) in WARCRAFT_DATABASE.iter() {
            let WarcraftObjectMeta::Unit(unit_meta) = warcraft_object.meta() else {
                continue;
            };
            for trained_id in unit_meta.trains() {
                produced_unit_ids.insert(trained_id.value());
            }
            for sold_id in unit_meta.sell_units() {
                produced_unit_ids.insert(sold_id.value());
            }
        }

        let mut members_by_name: BTreeMap<&'static str, Vec<&'static str>> = BTreeMap::new();
        for (object_id, warcraft_object) in WARCRAFT_DATABASE.iter() {
            let WarcraftObjectMeta::Unit(unit_meta) = warcraft_object.meta() else {
                continue;
            };
            if unit_meta.unit_kind() != UnitKind::Hero {
                continue;
            }
            let Some(display_name) = warcraft_object.names().first().copied() else {
                continue;
            };
            if display_name.is_empty() {
                continue;
            }
            let hero_ids = members_by_name.entry(display_name).or_default();
            hero_ids.push(object_id.value());
        }

        for hero_ids in members_by_name.into_values() {
            if hero_ids.len() < 2 {
                continue;
            }
            let produced_members: Vec<&'static str> = hero_ids
                .iter()
                .copied()
                .filter(|hero_id| produced_unit_ids.contains(hero_id))
                .collect();
            if produced_members.len() != 1 {
                continue;
            }
            let canonical = produced_members[0];
            let mut chain: Vec<&'static str> = hero_ids
                .into_iter()
                .filter(|hero_id| *hero_id != canonical)
                .collect();
            chain.sort_unstable();
            chain.push(canonical);
            self.add_chain(chain);
        }
    }

    fn into_registry(self) -> VariantRegistry {
        let mut union_find = UnionFind::default();
        let mut ordered_nodes: Vec<&'static str> = Vec::new();
        let mut seen_nodes: HashSet<&'static str> = HashSet::new();
        let mut edges: Vec<PrecedenceEdge> = Vec::new();
        for chain in &self.chains {
            let mut previous_member: Option<&'static str> = None;
            for member in chain {
                let member_id = *member;
                if seen_nodes.insert(member_id) {
                    ordered_nodes.push(member_id);
                }
                if let Some(previous_id) = previous_member
                    && previous_id != member_id
                {
                    union_find.merge(previous_id, member_id);
                    let edge = PrecedenceEdge {
                        predecessor: previous_id,
                        successor: member_id,
                    };
                    edges.push(edge);
                }
                previous_member = Some(member_id);
            }
        }

        let ranks = Self::assign_ranks(&ordered_nodes, &edges);

        let mut members_by_root: HashMap<&'static str, Vec<&'static str>> = HashMap::new();
        for node in &ordered_nodes {
            let node_id = *node;
            let root = union_find.root(node_id);
            let component = members_by_root.entry(root).or_default();
            component.push(node_id);
        }

        let mut groups: Vec<VariantGroup> = Vec::new();
        for component in members_by_root.into_values() {
            let mut members = component;
            members.sort_by(|left, right| {
                let left_rank = ranks.get(left).copied().unwrap_or(0);
                let right_rank = ranks.get(right).copied().unwrap_or(0);
                left_rank.cmp(&right_rank).then_with(|| left.cmp(right))
            });
            if members.len() >= 2 {
                let group = VariantGroup { members };
                groups.push(group);
            }
        }
        groups.sort_by(|left, right| left.canonical().cmp(right.canonical()));

        let mut group_index_by_member: HashMap<&'static str, usize> = HashMap::new();
        let mut canonical_by_member: HashMap<&'static str, &'static str> = HashMap::new();
        let mut hidden_members: HashSet<&'static str> = HashSet::new();
        for (group_index, group) in groups.iter().enumerate() {
            let canonical = group.canonical();
            for member in group.members() {
                let member_id = *member;
                group_index_by_member.insert(member_id, group_index);
                canonical_by_member.insert(member_id, canonical);
                if member_id != canonical {
                    hidden_members.insert(member_id);
                }
            }
        }

        VariantRegistry {
            groups,
            group_index_by_member,
            canonical_by_member,
            hidden_members,
        }
    }

    /// Longest-path rank of each member over the precedence edges: a source
    /// (weakest tier) ranks 0, each step toward the strongest tier ranks one
    /// higher. Relaxation is capped at the node count so a malformed cyclic
    /// chain can never loop forever.
    fn assign_ranks(
        nodes: &[&'static str],
        edges: &[PrecedenceEdge],
    ) -> HashMap<&'static str, u32> {
        let mut ranks: HashMap<&'static str, u32> = HashMap::new();
        for node in nodes {
            let initial_rank: u32 = 0;
            ranks.insert(*node, initial_rank);
        }
        let iteration_limit = nodes.len();
        let mut iteration: usize = 0;
        loop {
            let mut changed = false;
            for edge in edges {
                let predecessor_rank = ranks.get(edge.predecessor).copied().unwrap_or(0);
                let candidate_rank = predecessor_rank + 1;
                let successor_rank = ranks.get(edge.successor).copied().unwrap_or(0);
                if candidate_rank > successor_rank {
                    ranks.insert(edge.successor, candidate_rank);
                    changed = true;
                }
            }
            iteration += 1;
            if !changed || iteration >= iteration_limit {
                break;
            }
        }
        ranks
    }
}

/// A unit id is mergeable into a tier/upgrade-swap chain only when it is a real
/// non-hero unit. This drops the authoritative-data false positives: research/
/// upgrade ids that are unit-id-shaped (`Rguv`/`Reuv`), and hero forms a summon
/// ability happens to reference (Alchemist `Nal2`/`Nal3`). Heroes are excluded
/// here on purpose — they collapse through a separate name-and-production path
/// (`add_hero_name_groups`), never via summon tiers.
fn is_mergeable_variant_unit(unit_id: &str) -> bool {
    let lookup_result = WARCRAFT_DATABASE.by_id(unit_id);
    lookup_result.is_some_and(|warcraft_object| {
        let object_meta = warcraft_object.meta();
        matches!(
            object_meta,
            WarcraftObjectMeta::Unit(unit_meta) if unit_meta.unit_kind() != UnitKind::Hero
        )
    })
}

/// Tiered units the game data does *not* link authoritatively: their summon
/// ability omits the tier unit ids, no `rtma` upgrade swaps them, and nothing
/// references them — only the shared name and the `1`/`2`/`3` id suffix relate
/// them. Hand-curated (with the project owner's sign-off) because the set is
/// tiny and stable, and a verified id list is reliable where a name heuristic
/// would not be. Each entry is ordered weakest → strongest.
///
/// - Carrion Beetle (`ucs2` carries Burrow `Abu2`, `ucs3` carries `Abu3`): the
///   only tier whose abilities use *different* ids per level, so it is also the
///   only one that needs edit fan-out.
/// - Burrowed Carrion Beetle (`ucsB`/`ucsC`): the burrowed forms of the tier-2
///   and tier-3 beetles, listed separately too. They carry the same `Abu2`/
///   `Abu3` ids, so the existing fan-out covers them.
/// - Clockwerk Goblin (`ncg1`/`ncg2`/`ncg3`/`ncgb`): the Goblin Tinker's Pocket
///   Factory ships one logical unit as four stat-identical ids whose only
///   difference is the Self Destruct id (`Asd2`/`Asd3`/`Asdg`). Nothing in the
///   data links them, so they are curated. The Self Destruct abilities share the
///   `Asds` code and the same cell, so the existing fan-out reaches every form.
const CURATED_TIER_GROUPS: &[&[&str]] = &[
    &["ucs1", "ucs2", "ucs3"],
    &["ucsB", "ucsC"],
    &["ncg1", "ncg2", "ncg3", "ncgb"],
];

static VARIANT_REGISTRY: LazyLock<VariantRegistry> = LazyLock::new(|| {
    let mut builder = VariantGraphBuilder::default();
    for tiered_group in TIERED_UNIT_GROUPS {
        let mut chain: Vec<&'static str> = Vec::new();
        for member_object_id in tiered_group.iter() {
            let member_id = member_object_id.value();
            if is_mergeable_variant_unit(member_id) {
                chain.push(member_id);
            }
        }
        builder.add_chain(chain);
    }
    for curated_group in CURATED_TIER_GROUPS {
        let mut chain: Vec<&'static str> = Vec::new();
        for member_id in curated_group.iter() {
            if is_mergeable_variant_unit(member_id) {
                chain.push(member_id);
            }
        }
        builder.add_chain(chain);
    }
    for swap in UNIT_UPGRADE_SWAPS {
        let from_object_id = swap.from_unit_id();
        let to_object_id = swap.to_unit_id();
        let from_id = from_object_id.value();
        let to_id = to_object_id.value();
        if is_mergeable_variant_unit(from_id) && is_mergeable_variant_unit(to_id) {
            let chain: Vec<&'static str> = vec![from_id, to_id];
            builder.add_chain(chain);
        }
    }
    builder.add_hero_name_groups();
    builder.into_registry()
});

/// Identifies an ability's "role" on a command card: its mechanic `code` plus
/// its default cell. Two tier abilities pair as siblings only when both match,
/// so an edit fans out to the genuinely-corresponding ability and never to an
/// unrelated same-code ability elsewhere on the card.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct AbilityRoleKey {
    code: &'static str,
    column: u8,
    row: u8,
}

/// One button-positioned ability of a unit, tagged with its role key.
struct AbilityDescriptor {
    ability_id: &'static str,
    role: AbilityRoleKey,
}

/// The button-positioned abilities a unit carries (its own and any hero
/// abilities), each tagged with the role key used to pair tier siblings.
/// Abilities with no mechanic code or no default cell can't be paired and are
/// skipped.
fn unit_ability_descriptors(unit_id: &str) -> Vec<AbilityDescriptor> {
    let mut descriptors: Vec<AbilityDescriptor> = Vec::new();
    let Some(object) = WARCRAFT_DATABASE.by_id(unit_id) else {
        return descriptors;
    };
    let WarcraftObjectMeta::Unit(unit_meta) = object.meta() else {
        return descriptors;
    };
    let own_abilities = unit_meta.abilities().iter();
    let hero_abilities = unit_meta.hero_abilities().iter();
    for ability_id in own_abilities.chain(hero_abilities) {
        let ability_value = ability_id.value();
        let Some(ability_object) = WARCRAFT_DATABASE.by_id(ability_value) else {
            continue;
        };
        let WarcraftObjectMeta::Ability(ability_meta) = ability_object.meta() else {
            continue;
        };
        let Some(code) = ability_meta.code() else {
            continue;
        };
        let Some(default_position) = ability_meta.default_button_position() else {
            continue;
        };
        let column = u8::from(default_position.column());
        let row = u8::from(default_position.row());
        let role = AbilityRoleKey { code, column, row };
        let descriptor = AbilityDescriptor {
            ability_id: ability_value,
            role,
        };
        descriptors.push(descriptor);
    }
    descriptors
}

/// Maps an ability id to the *different-id* abilities that must receive the same
/// hotkey/position edits — its same-role counterparts on the other tiers of its
/// variant group. Tiers that reuse one ability id (the common case, e.g. the
/// Feral Spirit wolves) already share a single binding, so they produce no
/// entry here; only different-id tiers like the Carrion Beetle's Burrow
/// (`Abu2` ↔ `Abu3`) do.
static ABILITY_FANOUT: LazyLock<HashMap<&'static str, Vec<&'static str>>> = LazyLock::new(|| {
    let mut fanout: HashMap<&'static str, Vec<&'static str>> = HashMap::new();
    for group in &VARIANT_REGISTRY.groups {
        let mut ids_by_role: HashMap<AbilityRoleKey, Vec<&'static str>> = HashMap::new();
        for member in group.members() {
            for descriptor in unit_ability_descriptors(member) {
                let role_ids = ids_by_role.entry(descriptor.role).or_default();
                if !role_ids.contains(&descriptor.ability_id) {
                    role_ids.push(descriptor.ability_id);
                }
            }
        }
        for role_ids in ids_by_role.into_values() {
            if role_ids.len() < 2 {
                continue;
            }
            for ability_id in &role_ids {
                let siblings = fanout.entry(*ability_id).or_default();
                for other_id in &role_ids {
                    if other_id != ability_id && !siblings.contains(other_id) {
                        siblings.push(*other_id);
                    }
                }
            }
        }
    }
    fanout
});

/// Read-only façade over the merged variant groups. The catalog uses it to hide
/// the weaker variants, and the keybind facade uses it to fan edits out across
/// a group.
pub struct VariantUnits;

impl VariantUnits {
    /// Every merged variant group, ordered by canonical id for determinism.
    pub fn groups() -> &'static [VariantGroup] {
        &VARIANT_REGISTRY.groups
    }

    /// The variant group a unit belongs to, or `None` when the unit stands
    /// alone (the common case).
    pub fn group_for(unit_id: &str) -> Option<&'static VariantGroup> {
        let group_index = VARIANT_REGISTRY
            .group_index_by_member
            .get(unit_id)
            .copied()?;
        VARIANT_REGISTRY.groups.get(group_index)
    }

    /// The strongest sibling of a unit (itself when it already is the
    /// strongest), or `None` when the unit is not part of any group.
    pub fn canonical_for(unit_id: &str) -> Option<&'static str> {
        VARIANT_REGISTRY.canonical_by_member.get(unit_id).copied()
    }

    /// True when the unit is a weaker variant that the editor hides behind its
    /// canonical sibling.
    pub fn is_hidden_variant(unit_id: &str) -> bool {
        VARIANT_REGISTRY.hidden_members.contains(unit_id)
    }

    /// The other tier abilities that must receive the same hotkey/position edit
    /// as `ability_id` — its same-mechanic, same-cell counterparts on the other
    /// tiers of its variant group, restricted to abilities with a *different*
    /// id (same-id tiers already share one binding). Empty for almost every
    /// ability; non-empty only for different-id tiers like the Carrion Beetle's
    /// Burrow (`Abu3` → `Abu2`).
    pub fn fanout_siblings(ability_id: &str) -> &'static [&'static str] {
        let siblings = ABILITY_FANOUT.get(ability_id);
        siblings.map(Vec::as_slice).unwrap_or(&[])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Leveled summon tiers collapse to the strongest tier as canonical, with
    /// the weaker tiers hidden and pointing at it. The Feral Spirit wolves
    /// `osw1`/`osw2`/`osw3` are the reference case (the user's example: editing
    /// the strongest wolf's Critical Strike must reach the weaker wolves).
    #[test]
    fn leveled_summon_tiers_collapse_to_strongest() {
        let group = VariantUnits::group_for("osw1").expect("osw1 belongs to a variant group");
        assert_eq!(group.canonical(), "osw3");
        assert_eq!(group.members(), ["osw1", "osw2", "osw3"]);
        assert_eq!(group.weaker_members(), ["osw1", "osw2"]);
        assert_eq!(VariantUnits::canonical_for("osw1"), Some("osw3"));
        assert_eq!(VariantUnits::canonical_for("osw2"), Some("osw3"));
        assert!(VariantUnits::is_hidden_variant("osw1"));
        assert!(VariantUnits::is_hidden_variant("osw2"));
        assert!(!VariantUnits::is_hidden_variant("osw3"));
    }

    /// A 3-tier and a 4-tier summon of the same unit (Quillbeast
    /// `nqb1`/`nqb2`/`nqb3` and `nqb1`/`nqb2`/`nqb3`/`nqb4`; Spiderling
    /// `osp1..3` and `osp1..4`) ship as two overlapping source chains. They
    /// must union-merge into one ordered group, not two, with the highest tier
    /// canonical and no member appearing twice.
    #[test]
    fn overlapping_summon_chains_union_merge() {
        let quillbeast = VariantUnits::group_for("nqb1").expect("nqb1 belongs to a group");
        assert_eq!(quillbeast.members(), ["nqb1", "nqb2", "nqb3", "nqb4"]);
        assert_eq!(quillbeast.canonical(), "nqb4");
        assert_eq!(VariantUnits::canonical_for("nqb3"), Some("nqb4"));

        let spiderling = VariantUnits::group_for("osp1").expect("osp1 belongs to a group");
        assert_eq!(spiderling.members(), ["osp1", "osp2", "osp3", "osp4"]);
        assert_eq!(spiderling.canonical(), "osp4");
    }

    /// An upgrade-swap collapses to the upgraded unit: Headhunter `ohun` hides
    /// behind Berserker `otbk`.
    #[test]
    fn headhunter_berserker_swap_canonical_is_berserker() {
        assert_eq!(VariantUnits::canonical_for("ohun"), Some("otbk"));
        assert!(VariantUnits::is_hidden_variant("ohun"));
        assert!(!VariantUnits::is_hidden_variant("otbk"));
    }

    /// The Barrage upgrade-swap collapses Siege Engine `hmtt` into the
    /// barrage-capable `hrtt`.
    #[test]
    fn siege_engine_barrage_swap_canonical_is_upgraded() {
        assert_eq!(VariantUnits::canonical_for("hmtt"), Some("hrtt"));
        assert!(VariantUnits::is_hidden_variant("hmtt"));
        assert!(!VariantUnits::is_hidden_variant("hrtt"));
    }

    /// The hand-curated Carrion Beetle group collapses to the strongest tier
    /// `ucs3`, hiding `ucs1`/`ucs2`. This is the one tier the game data does not
    /// link, so it must come from the curated list.
    #[test]
    fn curated_carrion_beetle_group_collapses() {
        let group = VariantUnits::group_for("ucs2").expect("ucs2 is a curated tier member");
        assert_eq!(group.members(), ["ucs1", "ucs2", "ucs3"]);
        assert_eq!(group.canonical(), "ucs3");
        assert!(VariantUnits::is_hidden_variant("ucs1"));
        assert!(VariantUnits::is_hidden_variant("ucs2"));
        assert!(!VariantUnits::is_hidden_variant("ucs3"));

        // The burrowed beetle forms collapse the same way.
        let burrowed = VariantUnits::group_for("ucsB").expect("ucsB is a curated tier member");
        assert_eq!(burrowed.members(), ["ucsB", "ucsC"]);
        assert_eq!(burrowed.canonical(), "ucsC");
        assert!(VariantUnits::is_hidden_variant("ucsB"));
        assert!(!VariantUnits::is_hidden_variant("ucsC"));
    }

    /// The hand-curated Clockwerk Goblin group collapses its four stat-identical
    /// ids to `ncgb` as canonical, hiding `ncg1`/`ncg2`/`ncg3`. Nothing in the
    /// game data links them, so the group must come from the curated list.
    #[test]
    fn curated_clockwerk_goblin_group_collapses() {
        let group = VariantUnits::group_for("ncg1").expect("ncg1 is a curated tier member");
        assert_eq!(group.members(), ["ncg1", "ncg2", "ncg3", "ncgb"]);
        assert_eq!(group.canonical(), "ncgb");
        assert!(VariantUnits::is_hidden_variant("ncg1"));
        assert!(VariantUnits::is_hidden_variant("ncg2"));
        assert!(VariantUnits::is_hidden_variant("ncg3"));
        assert!(!VariantUnits::is_hidden_variant("ncgb"));
    }

    /// The Clockwerk Goblin tiers carry the Self Destruct ability under three
    /// different ids (`Asd2`/`Asd3`/`Asdg`), all sharing the `Asds` code and the
    /// same default cell, so editing one must fan out to the others.
    #[test]
    fn clockwerk_goblin_self_destruct_abilities_fan_out() {
        let siblings = VariantUnits::fanout_siblings("Asdg");
        assert!(siblings.contains(&"Asd2"), "Asdg must fan out to Asd2");
        assert!(siblings.contains(&"Asd3"), "Asdg must fan out to Asd3");
        let from_low = VariantUnits::fanout_siblings("Asd2");
        assert!(from_low.contains(&"Asdg"), "Asd2 must fan out to Asdg");
    }

    /// The Burrow ability uses a different id per beetle tier (`Abu2` on `ucs2`,
    /// `Abu3` on `ucs3`), so editing one must fan out to the other. They share
    /// the `Abur` code and the same default cell, so they pair as siblings.
    #[test]
    fn carrion_beetle_burrow_abilities_fan_out() {
        assert_eq!(VariantUnits::fanout_siblings("Abu3"), ["Abu2"]);
        assert_eq!(VariantUnits::fanout_siblings("Abu2"), ["Abu3"]);
    }

    /// Same-id tiers (the Feral Spirit wolves share `Asal`/`ACct`/`Apiv`) need
    /// no fan-out — one binding already covers every tier — so they produce no
    /// fan-out entry.
    #[test]
    fn shared_id_tier_abilities_have_no_fan_out() {
        assert!(VariantUnits::fanout_siblings("Asal").is_empty());
        assert!(VariantUnits::fanout_siblings("ACct").is_empty());
    }

    /// Research/upgrade ids that are unit-id-shaped (`Rguv`/`Reuv`) leak into
    /// the extracted tiered groups but are not units, so they must never form a
    /// variant group.
    #[test]
    fn non_unit_false_positives_are_filtered_out() {
        for false_positive_id in ["Rguv", "Reuv"] {
            assert!(
                VariantUnits::group_for(false_positive_id).is_none(),
                "{false_positive_id} must not form a variant group",
            );
            assert!(
                !VariantUnits::is_hidden_variant(false_positive_id),
                "{false_positive_id} must not be hidden as a variant",
            );
        }
    }

    /// Heroes with duplicate ids collapse to the one that is actually produced
    /// (trained/sold). Alchemist `Nal2`/`Nal3`/`Nalm` hide behind `Nalc`;
    /// Tinker `Nrob` hides behind `Ntin`; Paladin's many campaign/variant ids
    /// hide behind `Hpal`. The campaign/form ids that the tiered extraction
    /// once surfaced as a false positive (`Nal2`/`Nal3`/`Nalm`) are now
    /// correctly grouped here instead.
    #[test]
    fn heroes_collapse_to_the_produced_hero() {
        assert_eq!(VariantUnits::canonical_for("Nal2"), Some("Nalc"));
        assert_eq!(VariantUnits::canonical_for("Nal3"), Some("Nalc"));
        assert_eq!(VariantUnits::canonical_for("Nalm"), Some("Nalc"));
        assert!(VariantUnits::is_hidden_variant("Nal2"));
        assert!(!VariantUnits::is_hidden_variant("Nalc"));

        assert_eq!(VariantUnits::canonical_for("Nrob"), Some("Ntin"));
        assert!(VariantUnits::is_hidden_variant("Nrob"));

        assert_eq!(VariantUnits::canonical_for("Huth"), Some("Hpal"));
        assert!(VariantUnits::is_hidden_variant("Huth"));
        assert!(!VariantUnits::is_hidden_variant("Hpal"));
    }
}
