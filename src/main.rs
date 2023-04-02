#![allow(clippy::identity_op, clippy::erasing_op, clippy::enum_variant_names)]

use crate::{Ability::*, BootsEnch::*, DbwProc::*, MeleeEnch::*, Potion::*, Race::*, Scope::*};
use rand::{seq::SliceRandom, Rng};

// encounter details
const FIGHT_TIME: u64 = 100; // seconds
const BELOW_20: f64 = 0.2; // ratio
const HELLSCREAM_WARSONG: bool = true;

// professions
const BLACKSMITHING: bool = true;
const JEWELCRAFTING: bool = true;

// consumables, gems, enchants
const BLACKENED_DRAGONFIN: bool = true;
const HEARTY_RHINO: bool = false;
const SPICED_MAMMOTH_TREATS: bool = true;
const FLASK_OF_ENDLESS_RAGE: bool = true;
const ELIXIR_OF_MIGHTY_AGILITY: bool = false;
const ELIXIR_OF_ARMOR_PIERCING: bool = false;
const ELIXIR_OF_MIGHTY_THOUGHTS: bool = false;
const DELICATE_CARDINAL_RUBY: u16 = 9;
const DELICATE_DRAGONS_EYE: u16 = if ZOD { 1 } else { 2 };
const FRACTURED_CARDINAL_RUBY: u16 = 7;
const FRACTURED_DRAGONS_EYE: u16 = if ZOD { 2 } else { 1 };
const DEADLY_AMETRINE: u16 = 2;
const GLINTING_AMETRINE: u16 = 2;
const RIGID_DRAGONS_EYE: u16 = 0;
const NIGHTMARE_TEAR: bool = true;
const RELENTLESS_ED: bool = true;
const POTION: Potion = PotionOfSpeed;
const BOOTS_ENCH: BootsEnch = Icewalker;
const MELEE_ENCH: MeleeEnch = Massacre;
const SCOPE: Scope = HeartseekerScope;
const T10_2: bool = true;
const T10_4: bool = true;
const STS: bool = true;
const DBW: bool = true;
const WFS: bool = false;

// playstyle
const START_WITH_MARK: bool = true; // false if another hunter will
const MARK_RAP: f64 = 500.;
const PREPOT: bool = true;
const POT: bool = PREPOT && true;
const PRE_EXPLOSIVE: bool = false;
const _USE_MELEE: bool = false; // didn't implement
const ROTATION: [Ability; 6] = [Chimera, Aimed, Free, Free, Free, Free];
const OPENING: [Ability; 7] = [
    Mark, Serpent, Chimera, Aimed, Explosive, Readiness, Explosive,
];

// buffs
const HYSTERIA: u8 = 0; // how many DKs will give it, 0 to 6
const BLOODLUST: bool = true;
const BLESSING_OF_KINGS: bool = true;
const HORN_OF_WINTER: bool = true;
const BLESSING_OF_MIGHT: bool = true;
const IMPROVED_BLESSING_OF_MIGHT: bool = true;
const ABOMINATION_MIGHT: bool = true;
const SANCTIFIED_RETRIBUTION: bool = true;
const IMPROVED_MOONKIN_FORM: bool = true;
const ARCANE_INTELLECT: bool = true;
const RAMPAGE: bool = true;
const MARK_OF_THE_WILD: bool = true;
const IMPROVED_MARK_OF_THE_WILD: bool = true;
const EARTH_AND_MOON: bool = true;
const IMPROVED_ICY_TALONS: bool = true;

// debuffs
const SUNDER_ARMOR: bool = true;
const FAERIE_FIRE: bool = true;
const SAVAGE_COMBAT: bool = true;
const MANGLE: bool = true;
const TOTEM_OF_WRATH: bool = true;

// glyphs
const GLYPH_OF_EXPLOSIVE_TRAP: bool = true;
const GLYPH_OF_SERPENT_STING: bool = true;
const GLYPH_OF_STEADY_SHOT: bool = true;
const GLYPH_OF_THE_HAWK: bool = false;
const GLYPH_OF_KILL_SHOT: bool = false;

// talents
const LETHAL_SHOTS: u8 = 5;
const FOCUSED_AIM: u8 = 3;
const CAREFUL_AIM: u8 = 3;
const MORTAL_SHOTS: u8 = 5;
const BARRAGE: u8 = 3;
const COMBAT_EXPERIENCE: u8 = 2;
const RANGED_WEAPON_SPECIALIZATION: u8 = 3;
const IMPROVED_BARRAGE: u8 = 3;
const IMPROVED_STEADY_SHOT: u8 = 3;
const PIERCING_SHOTS: u8 = 3;
const MASTER_MARKSMAN: u8 = 5;
const WILD_QUIVER: u8 = 3;
const MARKED_FOR_DEATH: u8 = 5;

const IMPROVED_TRACKING: u8 = 5;
const SURVIVAL_INSTINCTS: u8 = 2;
const TRAP_MASTERY: u8 = 3;

const IMPROVED_ASPECT_OF_THE_HAWK: u8 = 5;
const FOCUSED_FIRE: u8 = 2; // gives 2% damage per point

const COBRA_REFLEXES: u8 = 2;
const SPIDERS_BITE: u8 = 3;
const CULLING_THE_HERD: u8 = 3;
const SPIKED_COLLAR: u8 = 3;
const SHARK_ATTACK: u8 = 1;
const _WILD_HUNT: u8 = 0; // doesn't work
const _RABID: u8 = 0; // didn't implement

// Fal hc
const RANGED_DAMAGE: f64 = (782.79 + 1070.79) / 2.;
// Fal nm
// const RANGED_DAMAGE: f64 = (688.07 + 948.07) / 2.;
// Zod
// const RANGED_DAMAGE: f64 = (618.28 + 999.28) / 2.;
const ZOD: bool = false;
const RANGED_SPEED: f64 = if ZOD { 2.8 } else { 3. };
const AMMO_DAMAGE: f64 = 91.5;
const _MELEE_DAMAGE: f64 = (991. + 1487.) / 2.;

struct Item {
    agi: u16,
    int: u16,
    ap: u16,
    cri: u16,
    arp: u16,
    haste: u16,
    hit: u16,
}

const HEAD: Item = Item {
    agi: 175 + 8,
    int: 96,
    ap: 212 + 50,
    cri: 106 + 20,
    arp: 88,
    haste: 0,
    hit: 0,
};
const NECK: Item = Item {
    agi: 102 + 4 * 0,
    int: 0,
    ap: 120,
    cri: 0,
    arp: 68,
    haste: 60,
    hit: 0,
};
// const NECK: Item = Item {
//     agi: 90 + 4,
//     int: 0,
//     ap: 113,
//     cri: 52,
//     arp: 60,
//     haste: 0,
//     hit: 0,
// };
const SHOULDER: Item = Item {
    agi: 136 + 4,
    int: 71,
    ap: 165 + 40,
    cri: 90 + 15,
    arp: 63,
    haste: 0,
    hit: 0,
};
const BACK: Item = Item {
    agi: 97 + 22,
    int: 0,
    ap: 114 + 8,
    cri: 65,
    arp: 57,
    haste: 0,
    hit: 0,
};
const CHEST: Item = Item {
    agi: 183 + 6 + 10,
    int: 96 + 10,
    ap: 212,
    cri: 114,
    arp: 88,
    haste: 0,
    hit: 0,
};
const WRIST: Item = Item {
    agi: 102 + 4 * 0,
    int: 54,
    ap: 120 + 50,
    cri: 68,
    arp: 46,
    haste: 0,
    hit: 0,
};
const HANDS: Item = Item {
    agi: 136 + 4 + 20,
    int: 71,
    ap: 165,
    cri: 82,
    arp: 0,
    haste: 0,
    hit: 71,
};
const WAIST: Item = Item {
    agi: 120 + 6,
    int: 71,
    ap: 181,
    cri: 74,
    arp: 71,
    haste: 0,
    hit: 0,
};
const LEGS: Item = Item {
    agi: 167 + 8,
    int: 96,
    ap: 228 + 75,
    cri: 106 + 22,
    arp: 88,
    haste: 0,
    hit: 0,
};
const BOOTS: Item = Item {
    agi: 145
        + 6
        + if matches!(BOOTS_ENCH, SuperiorAgility) {
            16
        } else {
            0
        },
    int: 76,
    ap: 162,
    cri: 97
        + if matches!(BOOTS_ENCH, Icewalker) {
            12
        } else {
            0
        },
    arp: 60,
    haste: 0,
    hit: 0 + if matches!(BOOTS_ENCH, Icewalker) {
        12
    } else {
        0
    },
};
const RING1: Item = Item {
    agi: 102,
    int: 0,
    ap: 120,
    cri: 60 + 4,
    arp: 68,
    haste: 0,
    hit: 0,
};
const ASHEN_BAND: bool = true;
const RING2: Item = Item {
    agi: 88 + 4,
    int: 0,
    ap: 135,
    cri: 59,
    arp: 0,
    haste: 0,
    hit: 59,
};
// const ASHEN_BAND: bool = false;
// const RING2: Item = Item {
//     agi: 109 + 4,
//     int: 0,
//     ap: 145,
//     cri: 73,
//     arp: 0,
//     haste: 0,
//     hit: 57,
// };
const TRINKET1: Item = Item {
    agi: 0,
    int: 0,
    ap: 0,
    cri: 0,
    arp: 167,
    haste: 0,
    hit: 0,
};
const TRINKET2: Item = Item {
    agi: 0,
    int: 0,
    ap: 0,
    cri: 0,
    arp: 184,
    haste: 0,
    hit: 0,
};
const MELEE: Item = Item {
    agi: 179 + 8,
    int: 0,
    ap: 228
        + if matches!(MELEE_ENCH, Massacre) {
            110
        } else {
            0
        },
    cri: 114
        + if matches!(MELEE_ENCH, Accuracy) {
            25
        } else {
            0
        },
    arp: 122,
    haste: 0,
    hit: 0 + if matches!(MELEE_ENCH, Accuracy) {
        25
    } else {
        0
    },
};
// Fal hc
const RANGED: Item = Item {
    agi: 62 + 4,
    int: 0,
    ap: 66,
    cri: 41
        + if matches!(SCOPE, HeartseekerScope) {
            40
        } else {
            0
        },
    arp: 33,
    haste: 0,
    hit: 0 + if matches!(SCOPE, AccuraScope) { 30 } else { 0 },
};
// Fal nm
// const RANGED: Item = Item {
//     agi: 54 - 20,
//     int: 0,
//     ap: 72,
//     cri: 36
//         + if matches!(SCOPE, HeartseekerScope) {
//             40
//         } else {
//             0
//         },
//     arp: 36,
//     haste: 0,
//     hit: 0 + if matches!(SCOPE, AccuraScope) { 30 } else { 0 },
// };
// ZOD
// const RANGED: Item = Item {
//     agi: 22,
//     int: 0,
//     ap: 0,
//     cri: 0 + if matches!(SCOPE, HeartseekerScope) {
//         40
//     } else {
//         0
//     },
//     arp: 0,
//     haste: 0,
//     hit: 0 + if matches!(SCOPE, AccuraScope) { 30 } else { 0 },
// };

const AGI: f64 = (if RELENTLESS_ED { 21 } else { 0 }
    + if ELIXIR_OF_MIGHTY_AGILITY { 45 } else { 0 }
    + if BLACKENED_DRAGONFIN { 40 } else { 0 }
    + HEAD.agi
    + NECK.agi
    + SHOULDER.agi
    + BACK.agi
    + CHEST.agi
    + WRIST.agi
    + HANDS.agi
    + WAIST.agi
    + LEGS.agi
    + BOOTS.agi
    + RING1.agi
    + RING2.agi
    + TRINKET1.agi
    + TRINKET2.agi
    + MELEE.agi
    + RANGED.agi
    + (DEADLY_AMETRINE + GLINTING_AMETRINE) * 10
    + DELICATE_CARDINAL_RUBY * 20
    + DELICATE_DRAGONS_EYE * 34
    + if NIGHTMARE_TEAR { 10 } else { 0 }) as _;
const INT: f64 = (if ELIXIR_OF_MIGHTY_THOUGHTS { 45 } else { 0 }
    + HEAD.int
    + NECK.int
    + SHOULDER.int
    + BACK.int
    + CHEST.int
    + WRIST.int
    + HANDS.int
    + WAIST.int
    + LEGS.int
    + BOOTS.int
    + RING1.int
    + RING2.int
    + TRINKET1.int
    + TRINKET2.int
    + MELEE.int
    + RANGED.int
    + if NIGHTMARE_TEAR { 10 } else { 0 }) as _;
const AP: f64 = (if FLASK_OF_ENDLESS_RAGE { 180 } else { 0 }
    + HEAD.ap
    + NECK.ap
    + SHOULDER.ap
    + BACK.ap
    + CHEST.ap
    + WRIST.ap
    + HANDS.ap
    + WAIST.ap
    + LEGS.ap
    + BOOTS.ap
    + RING1.ap
    + RING2.ap
    + TRINKET1.ap
    + TRINKET2.ap
    + MELEE.ap
    + RANGED.ap) as _;
const CRI: f64 = (HEAD.cri
    + NECK.cri
    + SHOULDER.cri
    + BACK.cri
    + CHEST.cri
    + WRIST.cri
    + HANDS.cri
    + WAIST.cri
    + LEGS.cri
    + BOOTS.cri
    + RING1.cri
    + RING2.cri
    + TRINKET1.cri
    + TRINKET2.cri
    + MELEE.cri
    + RANGED.cri
    + DEADLY_AMETRINE * 10) as _;
// 1376.3 is the cap having war and dru debuffs
const ARP: f64 = (if ELIXIR_OF_ARMOR_PIERCING { 45 } else { 0 }
    + if HEARTY_RHINO { 40 } else { 0 }
    + HEAD.arp
    + NECK.arp
    + SHOULDER.arp
    + BACK.arp
    + CHEST.arp
    + WRIST.arp
    + HANDS.arp
    + WAIST.arp
    + LEGS.arp
    + BOOTS.arp
    + RING1.arp
    + RING2.arp
    + TRINKET1.arp
    + TRINKET2.arp
    + MELEE.arp
    + RANGED.arp
    + FRACTURED_DRAGONS_EYE * 34
    + FRACTURED_CARDINAL_RUBY * 20) as _;
const HIT: f64 = (HEAD.hit
    + NECK.hit
    + SHOULDER.hit
    + BACK.hit
    + CHEST.hit
    + WRIST.hit
    + HANDS.hit
    + WAIST.hit
    + LEGS.hit
    + BOOTS.hit
    + RING1.hit
    + RING2.hit
    + TRINKET1.hit
    + TRINKET2.hit
    + MELEE.hit
    + RANGED.hit
    + GLINTING_AMETRINE * 10
    + RIGID_DRAGONS_EYE * 34) as _;
const HASTE: f64 = (HEAD.haste
    + NECK.haste
    + SHOULDER.haste
    + BACK.haste
    + CHEST.haste
    + WRIST.haste
    + HANDS.haste
    + WAIST.haste
    + LEGS.haste
    + BOOTS.haste
    + RING1.haste
    + RING2.haste
    + TRINKET1.haste
    + TRINKET2.haste
    + MELEE.haste
    + RANGED.haste) as _;

const RACE: Race = Troll;
// const RAC_STR: f64 = match RACE {
//     Orc => 77.,
//     Troll => 75.,
// };
const RAC_AGI: f64 = match RACE {
    Orc => 177.,
    Troll => 189.,
};
const RAC_INT: f64 = match RACE {
    Orc => 86.,
    Troll => 88.,
};
const RAC_AP: f64 = match RACE {
    Orc => 160.,
    Troll => 160.,
};
const RAC_CRI: f64 = match RACE {
    Orc => 0.0087,
    Troll => 0.0074,
};

const PET_STR: f64 = 331. + if SPICED_MAMMOTH_TREATS { 30. } else { 0. };

// armor effect calculation, works for 83 lvl bosses
const BOSS_ARMOR: f64 = 10673.;
const C1: f64 = 467.5 * 83. - 22167.5;
const C2: f64 = 467.5 * 80. - 22167.5;
const BOSS_ARMOR_DEBUFFED: f64 =
    BOSS_ARMOR * (if SUNDER_ARMOR { 0.8 } else { 1. }) * (if FAERIE_FIRE { 0.95 } else { 1. });
const ARMOR_REDUCTION: f64 =
    (if ARP > 1399. { 1. } else { ARP / 1399.6 }) * (BOSS_ARMOR_DEBUFFED + C1) / 3.;
const EFFECTIVE_ARMOR: f64 = BOSS_ARMOR_DEBUFFED - ARMOR_REDUCTION;
const DAMAGE_REDUCTION: f64 = EFFECTIVE_ARMOR / (EFFECTIVE_ARMOR + C2);

// physical damage modifier of ARP
const P: f64 = if DAMAGE_REDUCTION < 0. {
    1.
} else {
    1. - DAMAGE_REDUCTION
};

// pet damage modifier
const PET: f64 = 1. - BOSS_ARMOR_DEBUFFED / (BOSS_ARMOR_DEBUFFED + C2);

// shot damage modifier
const S: f64 = match RANGED_WEAPON_SPECIALIZATION {
    0 => 1.,
    1 => 1.01,
    2 => 1.03,
    _ => 1.05,
} * (1. + 0.01 * MARKED_FOR_DEATH as f64);

// magical damage modifier
const M: f64 = (5. + 0.9 * 3. + 0.8) / 9. * if EARTH_AND_MOON { 1.13 } else { 1. };

#[allow(dead_code)]
enum MeleeEnch {
    Massacre,
    Berserking,
    Accuracy,
}

#[allow(dead_code)]
enum BootsEnch {
    Icewalker,
    SuperiorAgility,
}

#[allow(dead_code)]
enum Scope {
    HeartseekerScope,
    AccuraScope,
}

#[allow(dead_code)]
enum Potion {
    PotionOfSpeed,
    PotionOfWildMagic,
}

#[allow(dead_code)]
enum Race {
    Troll,
    Orc,
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum Ability {
    Chimera,
    Aimed,
    Free,
    Mark,
    Serpent,
    Explosive,
    Readiness,
    Kill,
    Steady,
}

#[derive(Copy, Clone)]
enum DbwProc {
    DbwCri,
    DbwAgi,
    DbwAp,
}

fn crit(cri: f64, agi: f64, chance_bonus: f64, damage_bonus: f64) -> f64 {
    1. + crit_dmg(damage_bonus) * crit_chance(cri, agi, chance_bonus)
}

fn crit_dmg(damage_bonus: f64) -> f64 {
    1. + damage_bonus + if RELENTLESS_ED { 0.03 } else { 0.0 }
}

fn crit_chance(cri: f64, agi: f64, chance_bonus: f64) -> f64 {
    RAC_CRI
        + cri / 45.9 / 100.
        + (agi - RAC_AGI) / (83. + 1. / 3.) / 100.
        + chance_bonus
        + 0.01 * MASTER_MARKSMAN as f64
        + if TOTEM_OF_WRATH { 0.03 } else { 0. }
        - 0.048 // crit conversion
}

fn chance(c: f64, rng: &mut impl Rng) -> bool {
    rng.gen::<f64>() < c
}

fn not_miss(rng: &mut impl Rng) -> bool {
    let c = 0.92 + 0.01 * (FOCUSED_AIM as f64) + HIT * 0.05 / 164.;
    if c >= 1. {
        true
    } else {
        chance(c, rng)
    }
}

fn calculate_dps(debug: bool) -> f64 {
    let mut time = 0u64; // halves of seconds just as *_cd and *_time
    let mut hdamage = 0f64;
    let mut pdamage = 0f64;
    // let mut autoshots = 0u64;
    let mut rng = rand::thread_rng();
    let mut opening = OPENING.iter();
    let mut rotation = ROTATION.iter().cycle();

    let mut cri = CRI
        + if PREPOT && matches!(POTION, PotionOfWildMagic) {
            200.
        } else {
            0.
        };

    let statch = |v| {
        (v + if MARK_OF_THE_WILD { 37. } else { 0. }
            + if IMPROVED_MARK_OF_THE_WILD { 14. } else { 0. })
            * if BLESSING_OF_KINGS { 1.1 } else { 1. }
    };
    let agiint = |v| statch(v) * (1. + 0.02 * COMBAT_EXPERIENCE as f64);

    let mut raw_agi = RAC_AGI + AGI + if HORN_OF_WINTER { 155. } else { 0. };
    let int = agiint(RAC_INT + INT + if ARCANE_INTELLECT { 60. } else { 0. });

    let mut raw_rap = RAC_AP
        + AP
        + if BLESSING_OF_MIGHT { 550. } else { 0. }
        + if IMPROVED_BLESSING_OF_MIGHT { 137. } else { 0. }
        + int * CAREFUL_AIM as f64 / 3.;
    let mut rap_mod = if ABOMINATION_MIGHT { 1.1 } else { 1. };
    let mut raw_pap = (statch(PET_STR + if HORN_OF_WINTER { 155. } else { 0. }) - 10.) * 2.
        + if BLESSING_OF_MIGHT { 550. } else { 0. }
        + if IMPROVED_BLESSING_OF_MIGHT { 137. } else { 0. };
    let mut pap_mod = if ABOMINATION_MIGHT { 1.1 } else { 1. };
    // let mut map = rap + RAC_STR + STR - 10.;

    let mut haste = HASTE
        + if PREPOT && matches!(POTION, PotionOfSpeed) {
            500.
        } else {
            0.
        };
    // without haste
    let mut ranged_speed = RANGED_SPEED / 1.15 / (if IMPROVED_MOONKIN_FORM { 1.03 } else { 1. });
    let mut ranged_done = 1.;
    let pet_speed = 3.
        / (1. + 0.15 * COBRA_REFLEXES as f64)
        / (if IMPROVED_ICY_TALONS { 1.2 } else { 1. })
        / (if IMPROVED_MOONKIN_FORM { 1.03 } else { 1. });
    let mut pet_done = 1.;

    let mut gcd = 0u16;
    let mut chimera_cd = 0u16;
    let mut silencing_cd = 0u16;
    let mut still_chimera = false;
    let mut hysteria_cd = if HYSTERIA != 0 { 0 } else { u16::MAX };
    let mut bloodlust_cd = if BLOODLUST { 0 } else { u16::MAX };
    let mut blood_fury_cd = if let Orc = RACE { 0u16 } else { u16::MAX };
    let mut berserking_cd = if let Troll = RACE { 0u16 } else { u16::MAX };
    let mut rapid_fire_cd = 0u16;
    let mut readiness_cd = 0u16;
    let mut kill_shot_cd = 0u16;
    let mut explosive_cd = 0u16;
    let mut furious_howl_cd = 0u16;
    let mut call_of_the_wild_cd = 0u16;
    let mut kill_command_cd = 35u16 * 2; // either way it gets reset
    let mut sts_cd = if STS { 0u16 } else { u16::MAX };
    let mut dbw_cd = if DBW { 0u16 } else { u16::MAX };
    let mut wfs_cd = if WFS { 0u16 } else { u16::MAX };
    let mut ashen_band_cd = if ASHEN_BAND { 0u16 } else { u16::MAX };
    let mut berserking_ench_cd = if matches!(MELEE_ENCH, Berserking) {
        0u16
    } else {
        u16::MAX
    };
    let mut potion_cd = if POT { 60 * 2 } else { u16::MAX };

    let mut dbw_proc = DbwAgi;

    let mut hysteria_time = 0u16;
    let mut bloodlust_time = 0u8;
    let mut blood_fury_time = 0u8;
    let mut berserking_time = 0u8;
    let mut rapid_fire_time = 0u8;
    let mut explosive_time1 = if PRE_EXPLOSIVE { 20 * 2 } else { 0u8 };
    let mut explosive_time2 = 0u8;
    let mut explosive_time3 = 0u8;
    let mut serpent_time = 0u8;
    let mut piercing_time = 0u8;
    let mut furious_howl_time = 0u16;
    let mut call_of_the_wild_time = 0u8;
    let mut kc = 1.6;
    let mut sts_time = 0u8;
    let mut dbw_time = 0u8;
    let mut wfs_time = 0u8;
    let mut ashen_band_time = 0u8;
    let mut berserking_ench_time = 0u8;
    let mut potion_time = if PREPOT { 15 } else { 0u8 };
    let mut improved_aspect_of_the_hawk_time = 0u8;
    let mut t10_2_time = 0u8;
    let mut t10_4_time = 0u8;
    let mut culling_the_herd_time = 0u8;

    let mut improved_steady_shot = false;

    let mut serpent_damage = 0f64;
    let mut piercing_damage = 0f64;

    let mut p = P * (if SAVAGE_COMBAT { 1.04 } else { 1. }); // physical damage modifier
    let mut t = 1.; // 2 T10 and culling the herd proc modifier

    while time <= FIGHT_TIME * 2 {
        if hysteria_time > 0 {
            hysteria_time -= 1;
            if hysteria_time == 0 {
                p /= 1.2;
            }
        }
        if hysteria_cd == 0 {
            hysteria_cd = 3 * 60 * 2;
            hysteria_time = 30 * 2 * HYSTERIA as u16;
            p *= 1.2;
        }

        if bloodlust_time > 0 {
            bloodlust_time -= 1;
            if bloodlust_time == 0 {
                ranged_speed *= 1.3;
            }
        } else if bloodlust_cd == 0 {
            bloodlust_cd = 10 * 60 * 2;
            bloodlust_time = 30 * 2;
            ranged_speed /= 1.3;
        }

        if rapid_fire_time > 0 {
            rapid_fire_time -= 1;
            if rapid_fire_time == 0 {
                ranged_speed *= 1.4;
            }
        } else if rapid_fire_cd == 0 {
            rapid_fire_cd = 5 * 60 * 2;
            rapid_fire_time = 15 * 2;
            ranged_speed /= 1.4;
        }

        if berserking_time > 0 {
            berserking_time -= 1;
            if berserking_time == 0 {
                ranged_speed *= 1.2;
            }
        } else if berserking_cd == 0 {
            berserking_cd = 180 * 2;
            berserking_time = 10 * 2;
            ranged_speed /= 1.2;
        }

        if blood_fury_time > 0 {
            blood_fury_time -= 1;
            if blood_fury_time == 0 {
                raw_rap -= 322.;
            }
        } else if blood_fury_cd == 0 {
            blood_fury_cd = 120 * 2;
            blood_fury_time = 15 * 2;
            raw_rap += 322.;
        }

        if furious_howl_time > 0 {
            furious_howl_time -= 1;
            if furious_howl_time == 0 {
                raw_rap -= 320.;
                raw_pap -= 320.;
            }
        }
        if furious_howl_cd == 0 {
            furious_howl_cd = 40 * 2;
            furious_howl_time = 20 * 2;
            raw_rap += 320.;
            raw_pap += 320.;
        }

        if call_of_the_wild_time > 0 {
            call_of_the_wild_time -= 1;
            if call_of_the_wild_time == 0 {
                rap_mod /= 1.1;
                pap_mod /= 1.1;
            }
        } else if call_of_the_wild_cd == 0 {
            call_of_the_wild_cd = 600 * 2;
            call_of_the_wild_time = 20 * 2;
            rap_mod *= 1.1;
            pap_mod *= 1.1;
        }

        if kill_command_cd == 0 {
            kc = 1.6;
            kill_command_cd = 60 * 2;
        }

        if sts_time > 0 {
            sts_time -= 1;
            if sts_time == 0 {
                raw_rap -= 1472.;
            }
        } else if sts_cd == 0 {
            sts_cd = 45 * 2;
            sts_time = 15 * 2;
            raw_rap += 1472.;
        }

        if dbw_time > 0 {
            dbw_time -= 1;
            if dbw_time == 0 {
                match dbw_proc {
                    DbwAgi => raw_agi -= 700.,
                    DbwAp => raw_rap -= 1400.,
                    DbwCri => cri -= 700.,
                }
            }
        } else if dbw_cd == 0 {
            dbw_cd = 105 * 2;
            dbw_time = 30 * 2;
            dbw_proc = *[DbwAgi, DbwAp, DbwCri].choose(&mut rng).unwrap();
            match dbw_proc {
                DbwAgi => raw_agi += 700.,
                DbwAp => raw_rap += 1400.,
                DbwCri => cri += 700.,
            }
        }

        if wfs_time > 0 {
            wfs_time -= 1;
            if wfs_time == 0 {
                raw_rap -= 1250.;
            }
        } else if wfs_cd == 0 {
            wfs_cd = 45 * 2;
            wfs_time = 15 * 2;
            raw_rap += 1250.;
        }

        if ashen_band_time > 0 {
            ashen_band_time -= 1;
            if ashen_band_time == 0 {
                raw_rap -= 480.;
            }
        } else if ashen_band_cd == 0 {
            ashen_band_cd = 60 * 2;
            ashen_band_time = 10 * 2;
            raw_rap += 480.;
        }

        if berserking_ench_time > 0 {
            berserking_ench_time -= 1;
            if berserking_ench_time == 0 {
                raw_rap -= 400.;
            }
        } else if berserking_ench_cd == 0 {
            berserking_ench_cd = 60 * 2;
            berserking_ench_time = 15 * 2;
            raw_rap += 400.;
        }

        if potion_time > 0 {
            potion_time -= 1;
            if potion_time == 0 {
                match POTION {
                    PotionOfSpeed => haste -= 500.,
                    PotionOfWildMagic => cri -= 200.,
                }
            }
        } else if potion_cd == 0 {
            potion_cd = u16::MAX;
            potion_time = 15 * 2;
            match POTION {
                PotionOfSpeed => haste += 500.,
                PotionOfWildMagic => cri += 200.,
            }
        }

        if improved_aspect_of_the_hawk_time > 0 {
            improved_aspect_of_the_hawk_time -= 1;
            if improved_aspect_of_the_hawk_time == 0 {
                ranged_speed *= 1.
                    + (if GLYPH_OF_THE_HAWK { 0.042 } else { 0.03 })
                        * IMPROVED_ASPECT_OF_THE_HAWK as f64;
            }
        }

        if t10_2_time > 0 {
            t10_2_time -= 1;
            if t10_2_time == 0 {
                t /= 1.15;
            }
        }

        if t10_4_time > 0 {
            t10_4_time -= 1;
            if t10_4_time == 0 {
                rap_mod /= 1.2;
            }
        }

        if culling_the_herd_time > 0 {
            culling_the_herd_time -= 1;
            if culling_the_herd_time == 0 {
                t /= 1. + 0.01 * CULLING_THE_HERD as f64
            }
        }

        if serpent_time > 0 {
            serpent_time -= 1;
            if serpent_time % (3 * 2) == 0 {
                hdamage += serpent_damage;
                if T10_4 && chance(0.05, &mut rng) {
                    if t10_4_time == 0 {
                        rap_mod *= 1.2;
                    }
                    t10_4_time = 10 * 2;
                }
            }
        }

        if piercing_time > 0 {
            piercing_time -= 1;
            if piercing_time % (1 * 2) == 0 {
                hdamage += (if MANGLE { 1.3 } else { 1. })
                    * piercing_damage
                    * (0.1 * PIERCING_SHOTS as f64)
                    / 8.;
            }
        }

        let agi = agiint(raw_agi);
        let rap = rap_mod * (raw_rap + agi - 10.);
        let pap = pap_mod * (raw_pap + rap * 0.22);

        for explosive_time in [
            &mut explosive_time1,
            &mut explosive_time2,
            &mut explosive_time3,
        ] {
            if *explosive_time > 0 {
                *explosive_time -= 1;
                if *explosive_time % (2 * 2) == 0 {
                    let base_dmg = 90. + 0.1 * rap;
                    hdamage += t
                        * M
                        * if GLYPH_OF_EXPLOSIVE_TRAP {
                            crit(cri, agi, 0., 0.)
                        } else {
                            1.
                        }
                        * (1. + (TRAP_MASTERY as f64) * 0.1)
                        * base_dmg;
                }
            }
        }

        if pet_done >= 1. {
            pet_done -= 1.;
            let dmg = PET * (1. - 0.075 * COBRA_REFLEXES as f64) * (0.15 * pap + 68.);
            let cc = 0.05 - 0.048
                + 0.03 * SPIDERS_BITE as f64
                + if RAMPAGE { 0.05 } else { 0. }
                + if TOTEM_OF_WRATH { 0.03 } else { 0. };
            let gc = 0.24;
            pdamage += dmg * (gc * 0.7 + cc * 2. + (1. - cc - gc));
        }

        if ranged_done >= 1. {
            // autoshots += 1;
            ranged_done -= 1.;
            if not_miss(&mut rng) {
                let cb = 0.01 * LETHAL_SHOTS as f64 + if RAMPAGE { 0.05 } else { 0. };
                let base_dmg = RANGED_DAMAGE + (AMMO_DAMAGE + rap / 14.) * RANGED_SPEED;
                let dmg = t * p * S * crit(cri, agi, cb, 0.) * base_dmg;
                hdamage += dmg;
                if T10_2 && chance(0.05, &mut rng) {
                    t10_2_time = 10 * 2;
                    t *= 1.15;
                }
                if ZOD && chance(0.05, &mut rng) && not_miss(&mut rng) {
                    hdamage += dmg * 0.5;
                }
                if chance(0.1, &mut rng) {
                    if improved_aspect_of_the_hawk_time == 0 {
                        ranged_speed /= 1.
                            + (if GLYPH_OF_THE_HAWK { 0.042 } else { 0.03 })
                                * IMPROVED_ASPECT_OF_THE_HAWK as f64;
                    }
                    improved_aspect_of_the_hawk_time = 12 * 2;
                }
                if chance(0.04 * WILD_QUIVER as f64, &mut rng) {
                    hdamage += t * M * S * 0.8 * base_dmg;
                }
            }
        }

        if gcd == 0 {
            gcd = 3;

            {
                // bite
                let dmg = PET * kc * (pap / 14. + 150.) * (1. + 0.01 * MARKED_FOR_DEATH as f64);
                let cc = 0.05 - 0.048
                    + 0.03 * SPIDERS_BITE as f64
                    + if RAMPAGE { 0.05 } else { 0. }
                    + if TOTEM_OF_WRATH { 0.03 } else { 0. }
                    + if kc > 1. {
                        0.1 * FOCUSED_FIRE as f64
                    } else {
                        0.
                    };
                pdamage += dmg * (cc * 2. + (1. - cc));
                kc = (kc - 0.2).max(1.);
                if chance(cc, &mut rng) {
                    if culling_the_herd_time == 0 {
                        t *= 1. + 0.01 * CULLING_THE_HERD as f64;
                    }
                    culling_the_herd_time = 10 * 2;
                }
            }

            let mut ability = Chimera;

            if !still_chimera {
                let mut op = opening.next();
                match op {
                    None => op = rotation.next(),
                    Some(Mark) => {
                        raw_rap += MARK_RAP;
                        if !START_WITH_MARK {
                            op = opening.next()
                        }
                    }
                    _ => {}
                }
                ability = *op.unwrap();
            }

            if let Free = ability {
                if kill_shot_cd == 0 && time >= ((FIGHT_TIME * 2) as f64 * (1. - BELOW_20)) as u64 {
                    ability = Kill;
                } else if explosive_cd == 0 {
                    ability = Explosive;
                } else {
                    ability = Steady;
                }
            }

            if ZOD
                && [Serpent, Chimera, Aimed, Kill, Steady].contains(&ability)
                && chance(0.05, &mut rng)
                && not_miss(&mut rng)
                && not_miss(&mut rng)
            {
                let cb = 0.01 * LETHAL_SHOTS as f64 + if RAMPAGE { 0.05 } else { 0. };
                let base_dmg = RANGED_DAMAGE + (AMMO_DAMAGE + rap / 14.) * RANGED_SPEED;
                let dmg = t * p * S * crit(cri, agi, cb, 0.) * base_dmg;
                hdamage += dmg * 0.5;
            }

            match ability {
                Serpent => {
                    // Serpent
                    if not_miss(&mut rng) {
                        serpent_damage = t * M * S * (0.04 * rap + 242.);
                        serpent_time = 2 * if GLYPH_OF_SERPENT_STING { 21 } else { 15 };
                    }
                }
                Chimera => {
                    if chimera_cd == 0 {
                        chimera_cd = 10 * 2;
                        still_chimera = false;
                        // Silencing (for simplicity it's only here)
                        if silencing_cd == 0 {
                            silencing_cd = 20 * 2;
                            if not_miss(&mut rng) {
                                let cb =
                                    0.01 * LETHAL_SHOTS as f64 + if RAMPAGE { 0.05 } else { 0. };
                                let db = 0.06 * MORTAL_SHOTS as f64;
                                // for some reason hunter's mark affects it twice
                                let base_dmg = 0.5
                                    * (RANGED_DAMAGE
                                        + (AMMO_DAMAGE + (rap + MARK_RAP) / 14.) * RANGED_SPEED);
                                hdamage += t * p * S * crit(cri, agi, cb, db) * base_dmg;
                                if ZOD && chance(0.05, &mut rng) && not_miss(&mut rng) {
                                    let cb = 0.01 * LETHAL_SHOTS as f64
                                        + if RAMPAGE { 0.05 } else { 0. };
                                    let base_dmg =
                                        RANGED_DAMAGE + (AMMO_DAMAGE + rap / 14.) * RANGED_SPEED;
                                    let dmg = t * p * S * crit(cri, agi, cb, 0.) * base_dmg;
                                    hdamage += dmg * 0.5;
                                }
                            }
                        }
                        if not_miss(&mut rng) {
                            // Chimera
                            // for some reason it takes only 400 of hunter's mark
                            let base_dmg = 1.25
                                * (RANGED_DAMAGE
                                    + (AMMO_DAMAGE + (rap - MARK_RAP / 5.) / 14.) * 2.8);
                            let dmg =
                                t * M * S * base_dmg * if improved_steady_shot { 1.15 } else { 1. };
                            let sdmg = serpent_damage
                                * if GLYPH_OF_SERPENT_STING { 21. } else { 15. }
                                / 3.
                                * 0.4;
                            let cb = 0.01 * LETHAL_SHOTS as f64;
                            let db = 0.06 * MORTAL_SHOTS as f64 + 0.02 * MARKED_FOR_DEATH as f64;
                            hdamage += crit(cri, agi, cb, db) * (dmg + sdmg);
                            if chance(crit_chance(cri, agi, cb), &mut rng) {
                                piercing_time = 8 * 2;
                                piercing_damage = dmg * (1. + crit_dmg(db));
                            }
                            serpent_damage = t * M * S * (0.04 * rap + 242.);
                            serpent_time = 2 * if GLYPH_OF_SERPENT_STING { 21 } else { 15 };
                            improved_steady_shot = false;
                        }
                    } else {
                        still_chimera = true;
                        gcd = 0;
                    }
                }
                Aimed => {
                    if not_miss(&mut rng) {
                        let base_dmg = RANGED_DAMAGE + (AMMO_DAMAGE + rap / 14.) * 2.8 + 408.;
                        let dmg = t
                            * p
                            * S
                            * if improved_steady_shot { 1.15 } else { 1. }
                            * (1. + 0.04 * BARRAGE as f64)
                            * base_dmg;
                        let cb = 0.01 * LETHAL_SHOTS as f64
                            + 0.04 * IMPROVED_BARRAGE as f64
                            + if RAMPAGE { 0.05 } else { 0. };
                        let db = 0.06 * MORTAL_SHOTS as f64 + 0.02 * MARKED_FOR_DEATH as f64;
                        hdamage += crit(cri, agi, cb, db) * dmg;
                        if chance(crit_chance(cri, agi, cb), &mut rng) {
                            piercing_time = 8 * 2;
                            piercing_damage = dmg * (1. + crit_dmg(db));
                        }
                        improved_steady_shot = false;
                    }
                }
                Explosive => {
                    // Explosive Trap
                    let base_dmg = 0.1 * rap + (523. + 671.) / 2.;
                    hdamage += t * M * crit(cri, agi, 0., 0.) * base_dmg;
                    for explosive_time in [
                        &mut explosive_time1,
                        &mut explosive_time2,
                        &mut explosive_time3,
                    ] {
                        if *explosive_time == 0 {
                            *explosive_time = 20 * 2;
                            break;
                        }
                    }
                    explosive_cd = 30 * 2;
                }
                Readiness => {
                    gcd = 1 * 2;
                    chimera_cd = 0;
                    silencing_cd = 0;
                    rapid_fire_cd = 0;
                    kill_shot_cd = 0;
                    explosive_cd = 0;
                    kill_command_cd = 0;
                }
                Kill => {
                    kill_shot_cd = (if GLYPH_OF_KILL_SHOT { 9 } else { 15 }) * 2;
                    if not_miss(&mut rng) {
                        let cb = 0.01 * LETHAL_SHOTS as f64 + if RAMPAGE { 0.05 } else { 0. };
                        let db = 0.06 * MORTAL_SHOTS as f64 + 0.02 * MARKED_FOR_DEATH as f64;
                        let base_dmg = 2. * RANGED_DAMAGE + rap * 0.4 + 650.;
                        hdamage += t * p * S * crit(cri, agi, cb, db) * base_dmg;
                    }
                }
                Steady => {
                    if not_miss(&mut rng) {
                        let base_dmg =
                            RANGED_DAMAGE + AMMO_DAMAGE * RANGED_SPEED + 0.1 * rap + 252.;
                        let dmg =
                            t * p * S * if GLYPH_OF_STEADY_SHOT { 1.1 } else { 1. } * base_dmg;
                        let cb = 0.01 * LETHAL_SHOTS as f64
                            + 0.02 * SURVIVAL_INSTINCTS as f64
                            + if RAMPAGE { 0.05 } else { 0. };
                        let db = 0.06 * MORTAL_SHOTS as f64 + 0.02 * MARKED_FOR_DEATH as f64;
                        hdamage += crit(cri, agi, cb, db) * dmg;
                        if chance(crit_chance(cri, agi, cb), &mut rng) {
                            piercing_time = 8 * 2;
                            piercing_damage = dmg * (1. + crit_dmg(db));
                        }
                        if chance(0.05 * (IMPROVED_STEADY_SHOT as f64), &mut rng) {
                            improved_steady_shot = true;
                        }
                    }
                }
                _ => {}
            }
        }
        for cd in [
            &mut gcd,
            &mut chimera_cd,
            &mut silencing_cd,
            &mut hysteria_cd,
            &mut bloodlust_cd,
            &mut blood_fury_cd,
            &mut berserking_cd,
            &mut rapid_fire_cd,
            &mut readiness_cd,
            &mut kill_shot_cd,
            &mut explosive_cd,
            &mut furious_howl_cd,
            &mut call_of_the_wild_cd,
            &mut kill_command_cd,
            &mut sts_cd,
            &mut dbw_cd,
            &mut wfs_cd,
            &mut ashen_band_cd,
            &mut berserking_ench_cd,
            &mut potion_cd,
        ] {
            if *cd != 0 {
                *cd -= 1;
            }
        }
        time += 1;
        ranged_done += 0.5 / (ranged_speed / (1. + haste / 32.79 / 100.));
        if debug {
            // println!("ranged_speed: {}", ranged_speed);
            // println!("pet speed: {}", pet_speed);
            // println!("t: {}", t);
        }
        pet_done += 0.5 / pet_speed;
    }
    let mut damage =
        hdamage * (1. + 0.01 * IMPROVED_TRACKING as f64) * (1. + 0.02 * FOCUSED_FIRE as f64)
            + pdamage
                * (1. + 0.03 * SPIKED_COLLAR as f64)
                * (1. + 0.03 * SHARK_ATTACK as f64)
                * (if let Orc = RACE { 1.05 } else { 1. })
                * 1.25 // happiness
                ;
    if debug {
        // println!("pet damage: {}%", pdamage / (pdamage + hdamage) * 100.);
    }
    damage *= (if HELLSCREAM_WARSONG { 1.3 } else { 1. })
        * (if SANCTIFIED_RETRIBUTION { 1.03 } else { 1. });
    if debug {
        // println!("total damage: {}", damage);
        // println!("DPS: {}", damage / (FIGHT_TIME as f64));
        // println!("P: {}", P);
        // println!("PET: {}", PET);
        println!("ARP: {}", ARP);
        println!("HIT: {}", HIT);
        // println!("autoshots: {}", autoshots);
    }
    damage / (FIGHT_TIME as f64)
}

fn main() {
    assert_eq!(
        DELICATE_CARDINAL_RUBY
            + DELICATE_DRAGONS_EYE
            + FRACTURED_CARDINAL_RUBY
            + FRACTURED_DRAGONS_EYE,
        if BLACKSMITHING { 19 } else { 17 }
    );
    assert_eq!(GLINTING_AMETRINE + DEADLY_AMETRINE + RIGID_DRAGONS_EYE, 4);
    assert_eq!(
        DELICATE_DRAGONS_EYE + FRACTURED_DRAGONS_EYE + RIGID_DRAGONS_EYE,
        if JEWELCRAFTING { 3 } else { 0 }
    );
    let iterations = 1000000u64;
    let mut total = calculate_dps(true) / iterations as f64;
    for _ in 1..iterations {
        total += calculate_dps(false) / iterations as f64;
    }
    println!("\nDPS per {} iterations: {}", iterations, total);
}
