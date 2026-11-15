use std::collections::{BTreeMap, BTreeSet};

use crate::ets2::achievement_definitions::{ACHIEVEMENTS, ACHIEVEMENT_GROUPS};
use crate::ets2::cities::{city_by_id, City, CITIES};
use crate::ets2::generated::cargo_metadata::CARGOS;
use crate::ets2::save::{
    cargo_id, company_city, company_id, truck_brand, BusJob, DeliveryLogEntry, Garage, GarageSize,
    SaveGame,
};
use crate::ets2::CargoMetadata;

pub const EXPERIENCE_BEATS_ALL_CATEGORIES: [&str; 8] = [
    "Machinery",
    "ADR cargo",
    "Container",
    "Refrigerated",
    "Liquid cargo",
    "Fragile cargo",
    "Construction",
    "Bulk cargo",
];

const TEST_DRIVE_LIMITED_TARGET_BRANDS: u32 = 5;
const TEST_DRIVE_LIMITED_TARGET_DISTANCE_KM: u64 = 999;
const OWNED_TRUCK_JOB_TYPES: [&str; 4] = ["cargo", "external", "compn", "on_compn"];
const ALL_IS_POSSIBLE_TARGET_CARGOS: u32 = 30;
const RELIABLE_CONTRACTOR_TARGET_COMPANIES: u32 = 15;
const LONG_HAULER_TARGET_DISTANCE_KM: u32 = 2000;
const PROFIT_HUNTER_TARGET_REVENUE: f64 = 130_000.0;
const PROFIT_HUNTER_TARGET_DISTANCE_KM: u32 = 2200;
const SUMMARY_EVIDENCE_LIMIT: usize = 30;

/// `job_type` of Special Transport (oversize) jobs in the delivery log.
const OVERSIZE_JOB_TYPE: &str = "spec_oversize";
const THE_BIGGER_THE_BETTER_TARGET_CARGOS: u32 = 10;
const MASS_TO_DON_TARGET_TONNES: u32 = 195;
const DRIVER_EXCEPTIONNEL_TARGET_ROUTES: u32 = 10;
const FREE_ROAM_JOB_TYPE: &str = "freerm";
/// Special Transport DLC cargo: ID, in-game name, mass in tonnes.
const SPECIAL_TRANSPORT_CARGOES: [(&str, &str, u32); 11] = [
    ("boiler_parts", "Boiler Part", 45),
    ("condensator", "Industrial Condenser", 60),
    ("dumper", "Haul Truck Chassis", 55),
    ("dumper_tire", "Huge Tyres", 20),
    ("ex_bucket", "Excavator Bucket", 25),
    ("heat_exch", "Heat Exchanger", 70),
    ("lattice", "Construction Staircase", 50),
    ("mystery_box", "High-Tech Device / Massive Tech Part", 44),
    ("mystery_cyl", "Massive Tech Part / High-Tech Device", 45),
    ("pilot_boat", "Service Boat", 25),
    ("silo", "Giant Silo", 40),
];
/// Base-game Special Transport routes (`route_data.<id>` in the save's
/// `oversize_offer_ctrl`). The only Munich route in the save is
/// `munch_inns1`; it stands for the Munich route in the base-game list.
const BASE_GAME_SPECIAL_ROUTES: [(&str, &str); 17] = [
    ("lux_lieg1", "Luxembourg – Liège"),
    ("shef_bir1", "Sheffield – Birmingham"),
    ("bre_tra1", "Bremen – Travemünde"),
    ("ber_szcz1", "Berlin – Szczecin"),
    ("reim_stra1", "Reims – Strasbourg"),
    ("dort_hann1", "Dortmund – Hannover"),
    ("prag_nurn1", "Praha – Nürnberg"),
    ("glas_aber1", "Glasgow – Aberdeen"),
    ("munch_inns1", "Munich – Salzburg"),
    ("koln_amster1", "Cologne – Amsterdam"),
    ("brus_amster1", "Brussel – Amsterdam"),
    ("lin_br1", "Linz – Brno"),
    ("lon_fel1", "London – Felixstowe"),
    ("dij_gen1", "Dijon – Genève"),
    ("car_bir1", "Cardiff – Birmingham"),
    ("ply_dov1", "Plymouth – Dover"),
    ("brat_graz1", "Bratislava – Graz"),
];
const TURKISH_DELIGHT_TARGET_DELIVERIES: u32 = 3;
const TURKISH_DELIGHT_MIN_DISTANCE_KM: u32 = 2500;

// City IDs below follow the game's `company.volatile.<company>.<city>` naming,
// which uses local spellings (e.g. `munchen`, `wien`, `bucuresti`).
const ISTANBUL: &str = "istanbul";
const ORIENT_EXPRESS_LEGS: [(&str, &str); 6] = [
    ("paris", "strasbourg"),
    ("strasbourg", "munchen"),
    ("munchen", "wien"),
    ("wien", "budapest"),
    ("budapest", "bucuresti"),
    ("bucuresti", ISTANBUL),
];
const IBERIAN_PILGRIMAGE_LEGS: [(&str, &str); 3] = [
    ("lisboa", "a_coruna"),
    ("sevilla", "a_coruna"),
    ("pamplona", "a_coruna"),
];
const I_AM_A_GPS_TARGET_KM: u32 = 20_000;
const PATHFINDER_TARGET_KM: u32 = 40_000;
const SUCCESSFULLY_DOCKED_TARGET_PORTS: u32 = 5;
const SWIMMING_IN_SUCCESS_TARGET_PROFIT: u32 = 450_000;
const WORKING_WITH_THE_ELITE_TARGET_GARAGES: u32 = 5;
const PERFORMANCE_OPTIMIZER_TARGET_GARAGES: u32 = 10;
const PERFORMANCE_OPTIMIZER_TARGET_AVERAGE: f64 = 0.75;
/// Garage productivity as the game shows it: profit over the last 7 full
/// days (today excluded) against 300,000 EUR, the 100% mark for a large
/// garage. Matches the in-game percentages; `garage.productivity` does not.
const PROFIT_WEEK_DAYS: u32 = 7;
const FULL_PRODUCTIVITY_WEEK_PROFIT: i64 = 300_000;
const SMOOTH_START_TARGET_SATISFACTION: f64 = 0.9;
const SEASONED_TRAVELLER_TARGET_KM: u32 = 5_000;
const RULE_OF_THREES_TARGET_JOBS: u32 = 3;
const RULE_OF_THREES_MIN_DISTANCE_KM: u32 = 300;
/// Satisfaction is an f32 fraction; 1.0 is 100%.
const FULL_SATISFACTION: f64 = 0.9999;
/// Coach jobs done in your own coach (quick jobs provide one).
const OWN_COACH_JOB_TYPE: &str = "company_job";
const CORSICA_CITIES: [&str; 6] = [
    "ajaccio",
    "bastia",
    "bonifacio",
    "calvi",
    "lile_rousse",
    "porto_vecchi",
];
const SARDINIA_CITIES: [&str; 3] = ["cagliari", "olbia", "sassari"];
/// `dlc` of Scandinavia DLC cities in the city list (`dlc_north` in cities_by_dlc.json).
const SCANDINAVIA_DLC: &str = "scandinavia";
const CATTLE_CARGO: &str = "live_cattle";
/// The seven Scandinavia DLC quarries (`company_def` per city). `ms_stein` also
/// has a base-game quarry in Glasgow, which does not count.
const SCANDINAVIA_QUARRIES: [(&str, &str); 7] = [
    ("ms_stein", "kristiansand"),
    ("ms_stein", "oslo"),
    ("ms_stein", "stavanger"),
    ("nord_sten", "jonkoping"),
    ("nord_sten", "kalmar"),
    ("nord_sten", "linkoping"),
    ("nord_sten", "vaxjo"),
];
const MINER_TARGET_QUARRIES: u32 = 3;
const HEAD_HUNTER_TARGET_AGENCIES: u32 = 40;
const PROPERTY_MAGNATE_TARGET_GARAGES: u32 = 30;
// Company/city IDs below were checked against a 1.5x save's company list.
const FRENCH_NUCLEAR_PLANT_CITIES: [&str; 5] = ["alban", "civaux", "golfech", "laurent", "paluel"];
const SCANDINAVIA_CONTAINER_PORT_CITIES: [&str; 2] = ["esbjerg", "oslo"];

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct AchievementRegistry {
    pub groups: Vec<AchievementGroup>,
    pub achievements: Vec<Achievement>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Achievement {
    pub id: &'static str,
    pub group: &'static str,
    pub steam_api_name: &'static str,
    pub display_name: &'static str,
    pub description: &'static str,
    pub status: AchievementStatus,
    pub progress: AchievementProgress,
    pub evidence: Vec<AchievementEvidence>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum AchievementStatus {
    Complete,
    InProgress,
    /// The save data this achievement depends on is not read by the tracker yet.
    NotTracked,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct AchievementProgress {
    pub current: u32,
    pub target: u32,
    pub unit: &'static str,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct AchievementEvidence {
    pub label: String,
    pub value: String,
    pub items: Vec<String>,
    pub complete: bool,
}

/// One Steam achievement set: the base game, a DLC, or another special set.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct AchievementGroup {
    pub id: &'static str,
    pub name: &'static str,
    /// The set's `id` in Steam's achievement schema.
    pub steam_group_id: u32,
    /// The DLC's Steam app id, when the set belongs to one DLC.
    pub steam_dlc_app_id: Option<u32>,
}

/// Static description of one Steam achievement and how the tracker evaluates it.
#[derive(Debug, Clone, Copy)]
pub struct AchievementDefinition {
    pub id: &'static str,
    /// The `id` of the [`AchievementGroup`] this achievement belongs to.
    pub group: &'static str,
    /// The achievement's `api_name` on Steam (app 227300).
    pub steam_api_name: &'static str,
    pub display_name: &'static str,
    pub description: &'static str,
    pub tracking: AchievementTracking,
}

#[derive(Debug, Clone, Copy)]
pub enum AchievementTracking {
    /// Evaluated from the save game's delivery log.
    DeliveryLog(fn(&SaveGame) -> AchievementEvaluation),
    /// Listed for completeness; `reason` says what the tracker is missing.
    NotTracked { reason: &'static str },
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct AchievementEvaluation {
    pub status: AchievementStatus,
    pub progress: AchievementProgress,
    pub evidence: Vec<AchievementEvidence>,
}

pub fn evaluate_achievements(save: &SaveGame) -> AchievementRegistry {
    AchievementRegistry {
        groups: ACHIEVEMENT_GROUPS.to_vec(),
        achievements: ACHIEVEMENTS
            .iter()
            .map(|definition| evaluate_definition(definition, save))
            .collect(),
    }
}

fn evaluate_definition(definition: &AchievementDefinition, save: &SaveGame) -> Achievement {
    let evaluation = match definition.tracking {
        AchievementTracking::DeliveryLog(evaluate) => evaluate(save),
        AchievementTracking::NotTracked { reason } => AchievementEvaluation {
            status: AchievementStatus::NotTracked,
            progress: AchievementProgress {
                current: 0,
                target: 0,
                unit: "",
            },
            evidence: vec![AchievementEvidence {
                label: "Not tracked yet".to_string(),
                value: reason.to_string(),
                items: Vec::new(),
                complete: false,
            }],
        },
    };

    Achievement {
        id: definition.id,
        group: definition.group,
        steam_api_name: definition.steam_api_name,
        display_name: definition.display_name,
        description: definition.description,
        status: evaluation.status,
        progress: evaluation.progress,
        evidence: evaluation.evidence,
    }
}

pub(crate) fn experience_beats_all(save: &SaveGame) -> AchievementEvaluation {
    let mut evidence_by_category = EXPERIENCE_BEATS_ALL_CATEGORIES
        .into_iter()
        .map(|category| (category, BTreeSet::new()))
        .collect::<BTreeMap<_, _>>();

    for entry in &save.delivery_log.entries {
        for category in achievement_categories_for_cargo(&entry.cargo) {
            if let Some(cargos) = evidence_by_category.get_mut(category) {
                cargos.insert(cargo_id(&entry.cargo).to_string());
            }
        }
    }

    let evidence = EXPERIENCE_BEATS_ALL_CATEGORIES
        .into_iter()
        .map(|category| {
            let cargos = &evidence_by_category[category];
            AchievementEvidence {
                label: category.to_string(),
                value: summarize_set(cargos, SUMMARY_EVIDENCE_LIMIT),
                items: set_items(cargos),
                complete: !cargos.is_empty(),
            }
        })
        .collect::<Vec<_>>();
    let current = evidence.iter().filter(|item| item.complete).count() as u32;
    let target = EXPERIENCE_BEATS_ALL_CATEGORIES.len() as u32;

    AchievementEvaluation {
        status: status_from_progress(current, target),
        progress: AchievementProgress {
            current,
            target,
            unit: "categories",
        },
        evidence,
    }
}

pub(crate) fn test_drive_limited(save: &SaveGame) -> AchievementEvaluation {
    let mut distance_by_brand = BTreeMap::<String, u64>::new();
    for entry in save
        .delivery_log
        .entries
        .iter()
        .filter(|entry| is_owned_truck_job_type(&entry.job_type))
    {
        *distance_by_brand
            .entry(display_truck_brand(truck_brand(&entry.truck)).to_string())
            .or_insert(0) += u64::from(entry.distance_km);
    }

    let mut brands = distance_by_brand.into_iter().collect::<Vec<_>>();
    brands.sort_by(|left, right| right.1.cmp(&left.1).then_with(|| left.0.cmp(&right.0)));
    let evidence = brands
        .iter()
        .map(|(brand, distance)| AchievementEvidence {
            label: brand.clone(),
            value: format!("{distance} km"),
            items: Vec::new(),
            complete: *distance >= TEST_DRIVE_LIMITED_TARGET_DISTANCE_KM,
        })
        .collect::<Vec<_>>();
    let current = evidence.iter().filter(|item| item.complete).count() as u32;

    AchievementEvaluation {
        status: status_from_progress(current, TEST_DRIVE_LIMITED_TARGET_BRANDS),
        progress: AchievementProgress {
            current,
            target: TEST_DRIVE_LIMITED_TARGET_BRANDS,
            unit: "brands",
        },
        evidence,
    }
}

pub(crate) fn all_is_possible(save: &SaveGame) -> AchievementEvaluation {
    let cargos = save
        .delivery_log
        .unique_cargos()
        .into_iter()
        .filter(|cargo| !cargo.is_empty())
        .collect::<BTreeSet<_>>();
    let current = cargos.len() as u32;

    AchievementEvaluation {
        status: status_from_progress(current, ALL_IS_POSSIBLE_TARGET_CARGOS),
        progress: AchievementProgress {
            current,
            target: ALL_IS_POSSIBLE_TARGET_CARGOS,
            unit: "cargoes",
        },
        evidence: vec![AchievementEvidence {
            label: "Completed cargoes".to_string(),
            value: summarize_set(&cargos, SUMMARY_EVIDENCE_LIMIT),
            items: set_items(&cargos),
            complete: current >= ALL_IS_POSSIBLE_TARGET_CARGOS,
        }],
    }
}

pub(crate) fn reliable_contractor(save: &SaveGame) -> AchievementEvaluation {
    let companies = save
        .delivery_log
        .entries
        .iter()
        .flat_map(|entry| {
            [
                company_id(&entry.source_company).to_string(),
                company_id(&entry.destination_company).to_string(),
            ]
        })
        .filter(|company| !company.is_empty())
        .collect::<BTreeSet<_>>();
    let current = companies.len() as u32;

    AchievementEvaluation {
        status: status_from_progress(current, RELIABLE_CONTRACTOR_TARGET_COMPANIES),
        progress: AchievementProgress {
            current,
            target: RELIABLE_CONTRACTOR_TARGET_COMPANIES,
            unit: "companies",
        },
        evidence: vec![AchievementEvidence {
            label: "Companies".to_string(),
            value: summarize_set(&companies, SUMMARY_EVIDENCE_LIMIT),
            items: set_items(&companies),
            complete: current >= RELIABLE_CONTRACTOR_TARGET_COMPANIES,
        }],
    }
}

pub(crate) fn long_hauler(save: &SaveGame) -> AchievementEvaluation {
    let longest = save
        .delivery_log
        .entries
        .iter()
        .max_by_key(|entry| entry.distance_km);
    let current = longest.map_or(0, |entry| entry.distance_km);
    let complete = current > LONG_HAULER_TARGET_DISTANCE_KM;

    AchievementEvaluation {
        status: if complete {
            AchievementStatus::Complete
        } else {
            AchievementStatus::InProgress
        },
        progress: AchievementProgress {
            current,
            target: LONG_HAULER_TARGET_DISTANCE_KM,
            unit: "km",
        },
        evidence: vec![AchievementEvidence {
            label: "Longest delivery".to_string(),
            value: longest
                .map(delivery_summary)
                .unwrap_or_else(|| "missing".to_string()),
            items: Vec::new(),
            complete,
        }],
    }
}

pub(crate) fn profit_hunter(save: &SaveGame) -> AchievementEvaluation {
    let qualifying = save
        .delivery_log
        .entries
        .iter()
        .filter(|entry| is_profit_hunter_delivery(entry))
        .max_by(|left, right| left.revenue.total_cmp(&right.revenue));
    let evidence_entry = qualifying.or_else(|| {
        save.delivery_log
            .entries
            .iter()
            .max_by(|left, right| left.revenue.total_cmp(&right.revenue))
    });
    let current = evidence_entry.map_or(0, profit_hunter_criteria_met);
    let complete = qualifying.is_some();

    AchievementEvaluation {
        status: if complete {
            AchievementStatus::Complete
        } else {
            AchievementStatus::InProgress
        },
        progress: AchievementProgress {
            current,
            target: 2,
            unit: "requirements",
        },
        evidence: vec![AchievementEvidence {
            label: if complete {
                "Qualifying job".to_string()
            } else {
                "Best revenue job".to_string()
            },
            value: evidence_entry
                .map(delivery_summary)
                .unwrap_or_else(|| "missing".to_string()),
            items: Vec::new(),
            complete,
        }],
    }
}

pub(crate) fn i_am_a_gps(save: &SaveGame) -> AchievementEvaluation {
    discovered_roads(save, I_AM_A_GPS_TARGET_KM)
}

pub(crate) fn pathfinder(save: &SaveGame) -> AchievementEvaluation {
    discovered_roads(save, PATHFINDER_TARGET_KM)
}

/// Uses `player.discovary_distance`, read as km of discovered road.
fn discovered_roads(save: &SaveGame, target_km: u32) -> AchievementEvaluation {
    let km = save
        .career
        .discovered_road_km
        .map(|km| km.max(0.0).floor() as u32);
    counter(km, target_km, "km", "Discovered roads")
}

// ---- City discovery -------------------------------------------------------

enum CityScope {
    Country(&'static str),
    Dlc(&'static str),
    Ids(&'static [&'static str]),
}

impl CityScope {
    fn contains(&self, city: &City) -> bool {
        match self {
            CityScope::Country(country) => city.country == *country,
            CityScope::Dlc(dlc) => city.dlc == *dlc,
            CityScope::Ids(ids) => ids.contains(&city.id),
        }
    }
}

/// Counts `economy.visited_cities` that fall in `scope`, using the city list
/// from `cities_by_dlc.json`.
fn discovered_cities(save: &SaveGame, scope: CityScope, target: u32) -> AchievementEvaluation {
    let Some(visited) = &save.career.visited_cities else {
        return missing_data(target, "cities", "Discovered cities");
    };
    let in_scope = CITIES
        .iter()
        .filter(|city| scope.contains(city))
        .copied()
        .collect::<Vec<_>>();
    let discovered = in_scope
        .iter()
        .filter(|city| visited.iter().any(|id| id == city.id))
        .map(city_label)
        .collect::<BTreeSet<_>>();
    let missing = in_scope
        .iter()
        .filter(|city| !visited.iter().any(|id| id == city.id))
        .map(city_label)
        .collect::<BTreeSet<_>>();
    let mut evaluation = set_achievement(&discovered, target, "cities", "Discovered");
    evaluation.evidence.push(AchievementEvidence {
        label: "Not discovered yet".to_string(),
        value: if missing.is_empty() {
            "none".to_string()
        } else {
            summarize_set(&missing, SUMMARY_EVIDENCE_LIMIT)
        },
        items: set_items(&missing),
        complete: missing.is_empty(),
    });
    evaluation
}

pub(crate) fn odyssean_voyager(save: &SaveGame) -> AchievementEvaluation {
    discovered_cities(save, CityScope::Country("greece"), 12)
}

pub(crate) fn bon_voyage(save: &SaveGame) -> AchievementEvaluation {
    discovered_cities(save, CityScope::Country("france"), 20)
}

pub(crate) fn all_around_the_isle_of_beauty(save: &SaveGame) -> AchievementEvaluation {
    discovered_cities(save, CityScope::Ids(&CORSICA_CITIES), 4)
}

pub(crate) fn balkan_explorer(save: &SaveGame) -> AchievementEvaluation {
    discovered_cities(save, CityScope::Dlc("road_to_the_black_sea"), 20)
}

pub(crate) fn sightseer(save: &SaveGame) -> AchievementEvaluation {
    discovered_cities(save, CityScope::Dlc("scandinavia"), 15)
}

pub(crate) fn baltic_tourist(save: &SaveGame) -> AchievementEvaluation {
    discovered_cities(save, CityScope::Dlc("beyond_the_baltic_sea"), 20)
}

pub(crate) fn conquistador(save: &SaveGame) -> AchievementEvaluation {
    discovered_cities(save, CityScope::Dlc("iberia"), 30)
}

pub(crate) fn imperator(save: &SaveGame) -> AchievementEvaluation {
    discovered_cities(save, CityScope::Country("italy"), 15)
}

pub(crate) fn all_around_the_blue_island(save: &SaveGame) -> AchievementEvaluation {
    discovered_cities(save, CityScope::Ids(&SARDINIA_CITIES), 2)
}

pub(crate) fn road_to_the_adriatics(save: &SaveGame) -> AchievementEvaluation {
    discovered_cities(save, CityScope::Dlc("west_balkans"), 20)
}

// ---- Scandinavia deliveries -------------------------------------------------

pub(crate) fn cattle_drive(save: &SaveGame) -> AchievementEvaluation {
    let deliveries = save
        .delivery_log
        .entries
        .iter()
        .filter(|entry| cargo_id(&entry.cargo) == CATTLE_CARGO)
        .filter(|entry| {
            city_by_id(company_city(&entry.destination_company))
                .is_some_and(|city| city.dlc == SCANDINAVIA_DLC)
        })
        .map(|entry| {
            format!(
                "{} → {}, {} km",
                city_name(company_city(&entry.source_company)),
                city_name(company_city(&entry.destination_company)),
                entry.distance_km
            )
        })
        .collect::<Vec<_>>();
    let current = deliveries.len() as u32;

    AchievementEvaluation {
        status: status_from_progress(current, 1),
        progress: AchievementProgress {
            current,
            target: 1,
            unit: "deliveries",
        },
        evidence: vec![AchievementEvidence {
            label: "Cattle to Scandinavia".to_string(),
            value: summarize_list(&deliveries),
            items: deliveries.clone(),
            complete: current >= 1,
        }],
    }
}

pub(crate) fn miner(save: &SaveGame) -> AchievementEvaluation {
    let delivered = save
        .delivery_log
        .entries
        .iter()
        .filter_map(|entry| {
            let company = company_id(&entry.destination_company);
            let city = company_city(&entry.destination_company);
            SCANDINAVIA_QUARRIES
                .contains(&(company, city))
                .then(|| quarry_label(company, city))
        })
        .collect::<BTreeSet<_>>();
    let missing = SCANDINAVIA_QUARRIES
        .iter()
        .map(|(company, city)| quarry_label(company, city))
        .filter(|quarry| !delivered.contains(quarry))
        .collect::<BTreeSet<_>>();
    let mut evaluation = set_achievement(
        &delivered,
        MINER_TARGET_QUARRIES,
        "quarries",
        "Delivered to",
    );
    evaluation.evidence.push(AchievementEvidence {
        label: "Not delivered to yet".to_string(),
        value: if missing.is_empty() {
            "none".to_string()
        } else {
            summarize_set(&missing, SUMMARY_EVIDENCE_LIMIT)
        },
        items: set_items(&missing),
        complete: missing.is_empty(),
    });
    evaluation
}

fn quarry_label(company: &str, city: &str) -> String {
    let name = match company {
        "ms_stein" => "MS Stein",
        "nord_sten" => "Nordic Stenbrott",
        other => other,
    };
    format!("{name} ({})", city_name(city))
}

// ---- Garages and company profit -------------------------------------------

pub(crate) fn national_company(save: &SaveGame) -> AchievementEvaluation {
    let Some(garages) = &save.career.garages else {
        return missing_data(1, "large garages", "Garages");
    };
    let Some(hq) = save.career.hq_city.as_deref() else {
        return missing_data(1, "large garages", "Headquarters");
    };
    let Some(hq_city) = city_by_id(hq) else {
        return missing_data(1, "large garages", &format!("Country of HQ city {hq}"));
    };
    let in_country = garages
        .iter()
        .filter(|garage| garage.size == GarageSize::Large)
        .filter(|garage| {
            city_by_id(&garage.city).is_some_and(|city| city.country == hq_city.country)
        })
        .map(garage_name)
        .collect::<BTreeSet<_>>();
    let mut evaluation = set_achievement(&in_country, 1, "large garages", "Large garages there");
    evaluation.evidence.insert(
        0,
        AchievementEvidence {
            label: "Headquarters".to_string(),
            value: format!("{} ({})", hq_city.name, hq_city.country),
            items: Vec::new(),
            complete: true,
        },
    );
    evaluation
}

pub(crate) fn swimming_in_success(save: &SaveGame) -> AchievementEvaluation {
    let Some(garages) = &save.career.garages else {
        return missing_data(SWIMMING_IN_SUCCESS_TARGET_PROFIT, "EUR", "Profit log");
    };
    // Garage profit logs together cover every driver, the player included.
    let mut by_day = std::collections::BTreeMap::<u32, i64>::new();
    for entry in garages.iter().flat_map(|garage| &garage.daily_profit) {
        *by_day.entry(entry.day).or_insert(0) += entry.profit;
    }
    let (Some(first_day), Some(last_day)) = (
        by_day.keys().next().copied(),
        by_day.keys().next_back().copied(),
    ) else {
        return missing_data(SWIMMING_IN_SUCCESS_TARGET_PROFIT, "EUR", "Profit log");
    };
    let (best_day, best_profit) = by_day
        .iter()
        .max_by_key(|(_, profit)| **profit)
        .map(|(day, profit)| (*day, *profit))
        .unwrap_or((first_day, 0));
    let average = by_day.values().sum::<i64>() / by_day.len() as i64;
    let current = u32::try_from(best_profit.max(0)).unwrap_or(u32::MAX);
    let days_ago = |day: u32| match save
        .career
        .current_day
        .map(|today| today.saturating_sub(day))
    {
        Some(0) => "today".to_string(),
        Some(1) => "yesterday".to_string(),
        Some(days) => format!("{days} days ago"),
        None => format!("day {day}"),
    };

    AchievementEvaluation {
        status: status_from_progress(current, SWIMMING_IN_SUCCESS_TARGET_PROFIT),
        progress: AchievementProgress {
            current,
            target: SWIMMING_IN_SUCCESS_TARGET_PROFIT,
            unit: "EUR",
        },
        evidence: vec![
            AchievementEvidence {
                label: "Best day".to_string(),
                value: format!("{best_profit} EUR ({})", days_ago(best_day)),
                items: Vec::new(),
                complete: current >= SWIMMING_IN_SUCCESS_TARGET_PROFIT,
            },
            AchievementEvidence {
                label: format!("Average of {} days", by_day.len()),
                value: format!("{average} EUR"),
                items: Vec::new(),
                complete: average >= i64::from(SWIMMING_IN_SUCCESS_TARGET_PROFIT),
            },
            AchievementEvidence {
                label: "Note".to_string(),
                value: format!(
                    "The save only keeps the last {} days of profit ({} to {}), so older days are not checked",
                    last_day - first_day + 1,
                    days_ago(first_day),
                    days_ago(last_day)
                ),
                items: Vec::new(),
                complete: false,
            },
        ],
    }
}

pub(crate) fn working_with_the_elite(save: &SaveGame) -> AchievementEvaluation {
    let Some(large) = large_garage_weeks(save) else {
        return missing_data(WORKING_WITH_THE_ELITE_TARGET_GARAGES, "garages", "Garages");
    };
    let full = large
        .iter()
        .filter(|week| week.profit >= FULL_PRODUCTIVITY_WEEK_PROFIT)
        .count() as u32;
    let summaries = large.iter().map(GarageWeek::summary).collect::<Vec<_>>();

    AchievementEvaluation {
        status: status_from_progress(full, WORKING_WITH_THE_ELITE_TARGET_GARAGES),
        progress: AchievementProgress {
            current: full,
            target: WORKING_WITH_THE_ELITE_TARGET_GARAGES,
            unit: "large garages at 100%",
        },
        evidence: vec![AchievementEvidence {
            label: "Large garages".to_string(),
            value: summarize_list(&summaries),
            items: summaries.clone(),
            complete: full >= WORKING_WITH_THE_ELITE_TARGET_GARAGES,
        }],
    }
}

pub(crate) fn performance_optimizer(save: &SaveGame) -> AchievementEvaluation {
    let Some(large) = large_garage_weeks(save) else {
        return missing_data(PERFORMANCE_OPTIMIZER_TARGET_GARAGES, "garages", "Garages");
    };
    let count = large.len() as u32;
    // Average over all large garages, so extra low-productivity ones count against it.
    let average = if large.is_empty() {
        0.0
    } else {
        large.iter().map(GarageWeek::productivity).sum::<f64>() / large.len() as f64
    };
    let enough = count >= PERFORMANCE_OPTIMIZER_TARGET_GARAGES;
    let average_met = average >= PERFORMANCE_OPTIMIZER_TARGET_AVERAGE;
    let summaries = large.iter().map(GarageWeek::summary).collect::<Vec<_>>();

    AchievementEvaluation {
        status: if enough && average_met {
            AchievementStatus::Complete
        } else {
            AchievementStatus::InProgress
        },
        // The bar follows the average; the garage count is its own row.
        progress: AchievementProgress {
            current: (average * 100.0).floor() as u32,
            target: (PERFORMANCE_OPTIMIZER_TARGET_AVERAGE * 100.0) as u32,
            unit: "% average productivity",
        },
        evidence: vec![
            AchievementEvidence {
                label: "Large garages owned".to_string(),
                value: format!("{count} (needs {PERFORMANCE_OPTIMIZER_TARGET_GARAGES})"),
                items: Vec::new(),
                complete: enough,
            },
            AchievementEvidence {
                label: "Large garages".to_string(),
                value: summarize_list(&summaries),
                items: summaries.clone(),
                complete: enough,
            },
            AchievementEvidence {
                label: "Average productivity".to_string(),
                value: format!("{:.0}% (needs 75%)", average * 100.0),
                items: Vec::new(),
                complete: average_met,
            },
        ],
    }
}

/// A large garage's profit over the last 7 full days.
struct GarageWeek {
    name: String,
    profit: i64,
}

impl GarageWeek {
    fn productivity(&self) -> f64 {
        self.profit as f64 / FULL_PRODUCTIVITY_WEEK_PROFIT as f64
    }

    /// "Stockholm 63% · 190296 EUR profit in 7 days"
    fn summary(&self) -> String {
        format!(
            "{} {:.0}% · {} EUR profit in {PROFIT_WEEK_DAYS} days",
            self.name,
            self.productivity() * 100.0,
            self.profit
        )
    }
}

/// Large garages with their last-7-days profit; `None` without garages or
/// without the in-game day.
fn large_garage_weeks(save: &SaveGame) -> Option<Vec<GarageWeek>> {
    let garages = save.career.garages.as_ref()?;
    let today = save.career.current_day?;
    let first_day = today.saturating_sub(PROFIT_WEEK_DAYS);
    Some(
        garages
            .iter()
            .filter(|garage| garage.size == GarageSize::Large)
            .map(|garage| GarageWeek {
                name: garage_name(garage),
                profit: garage
                    .daily_profit
                    .iter()
                    .filter(|entry| (first_day..today).contains(&entry.day))
                    .map(|entry| entry.profit)
                    .sum(),
            })
            .collect(),
    )
}

fn garage_name(garage: &Garage) -> String {
    city_name(&garage.city)
}

/// Display name for a city ID, falling back to the ID itself.
fn city_name(id: &str) -> String {
    city_by_id(id).map_or_else(|| id.to_string(), city_label)
}

/// The city's name; names in Greek or Cyrillic script also get the city ID,
/// so they stay readable ("Αθήνα (athens)").
fn city_label(city: &City) -> String {
    let non_latin = city
        .name
        .chars()
        .any(|c| ('\u{0370}'..='\u{03FF}').contains(&c) || ('\u{0400}'..='\u{04FF}').contains(&c));
    if non_latin {
        format!("{} ({})", city.name, city.id)
    } else {
        city.name.to_string()
    }
}

// ---- Coaches ---------------------------------------------------------------

pub(crate) fn smooth_start(save: &SaveGame) -> AchievementEvaluation {
    let Some(jobs) = &save.career.bus_jobs else {
        return missing_data(1, "coach jobs", "Coach jobs");
    };
    let qualifying = jobs
        .iter()
        .filter(|job| job.satisfaction >= SMOOTH_START_TARGET_SATISFACTION)
        .map(bus_job_summary)
        .collect::<Vec<_>>();
    let best = jobs
        .iter()
        .map(|job| job.satisfaction)
        .fold(None, |best: Option<f64>, value| {
            Some(best.map_or(value, |b| b.max(value)))
        });
    let current = qualifying.len() as u32;

    AchievementEvaluation {
        status: status_from_progress(current, 1),
        progress: AchievementProgress {
            current,
            target: 1,
            unit: "coach jobs",
        },
        evidence: vec![
            AchievementEvidence {
                label: "Best satisfaction".to_string(),
                value: best.map_or_else(
                    || "missing".to_string(),
                    |value| format!("{:.0}% (needs 90%)", value * 100.0),
                ),
                items: Vec::new(),
                complete: current >= 1,
            },
            AchievementEvidence {
                label: "Jobs at 90% or more".to_string(),
                value: summarize_list(&qualifying),
                items: qualifying.clone(),
                complete: current >= 1,
            },
        ],
    }
}

pub(crate) fn seasoned_traveller(save: &SaveGame) -> AchievementEvaluation {
    let Some(jobs) = &save.career.bus_jobs else {
        return missing_data(SEASONED_TRAVELLER_TARGET_KM, "km", "Coach jobs");
    };
    let own = jobs
        .iter()
        .filter(|job| job.kind == OWN_COACH_JOB_TYPE)
        .collect::<Vec<_>>();
    let current = own.iter().map(|job| job.distance_km).sum::<u32>();
    let summaries = own
        .iter()
        .map(|job| bus_job_summary(job))
        .collect::<Vec<_>>();

    AchievementEvaluation {
        status: status_from_progress(current, SEASONED_TRAVELLER_TARGET_KM),
        progress: AchievementProgress {
            current,
            target: SEASONED_TRAVELLER_TARGET_KM,
            unit: "km",
        },
        evidence: vec![AchievementEvidence {
            label: "Own-coach jobs".to_string(),
            value: summarize_list(&summaries),
            items: summaries.clone(),
            complete: current >= SEASONED_TRAVELLER_TARGET_KM,
        }],
    }
}

pub(crate) fn multitalented_driver(save: &SaveGame) -> AchievementEvaluation {
    let Some(skills) = &save.career.coach_skills else {
        return missing_data(skills_target(), "coach skills", "Coach skills");
    };
    let evidence = skills
        .iter()
        .map(|(label, points)| AchievementEvidence {
            label: (*label).to_string(),
            value: format!("{points} points"),
            items: Vec::new(),
            complete: *points >= 1,
        })
        .collect::<Vec<_>>();
    let current = evidence.iter().filter(|item| item.complete).count() as u32;
    let target = skills.len() as u32;

    AchievementEvaluation {
        status: status_from_progress(current, target),
        progress: AchievementProgress {
            current,
            target,
            unit: "coach skills",
        },
        evidence,
    }
}

fn skills_target() -> u32 {
    crate::ets2::save::COACH_SKILLS.len() as u32
}

pub(crate) fn rule_of_threes(save: &SaveGame) -> AchievementEvaluation {
    let Some(jobs) = &save.career.bus_jobs else {
        return missing_data(RULE_OF_THREES_TARGET_JOBS, "jobs in a row", "Coach jobs");
    };
    let qualifies = |job: &BusJob| {
        job.distance_km >= RULE_OF_THREES_MIN_DISTANCE_KM && job.satisfaction >= FULL_SATISFACTION
    };
    let (mut best, mut streak) = (0_u32, 0_u32);
    for job in jobs {
        streak = if qualifies(job) { streak + 1 } else { 0 };
        best = best.max(streak);
    }
    let qualifying = jobs
        .iter()
        .filter(|job| qualifies(job))
        .map(bus_job_summary)
        .collect::<Vec<_>>();

    AchievementEvaluation {
        status: status_from_progress(best, RULE_OF_THREES_TARGET_JOBS),
        progress: AchievementProgress {
            current: best,
            target: RULE_OF_THREES_TARGET_JOBS,
            unit: "jobs in a row",
        },
        evidence: vec![AchievementEvidence {
            label: "300 km+ jobs at 100%".to_string(),
            value: summarize_list(&qualifying),
            items: qualifying.clone(),
            complete: best >= RULE_OF_THREES_TARGET_JOBS,
        }],
    }
}

fn bus_job_summary(job: &BusJob) -> String {
    format!(
        "{} → {}, {} km, {:.0}%",
        city_name(&job.from),
        city_name(&job.to),
        job.distance_km,
        job.satisfaction * 100.0
    )
}

// ---- Ferries ---------------------------------------------------------------

pub(crate) fn successfully_docked(save: &SaveGame) -> AchievementEvaluation {
    let Some(crossings) = &save.career.ferry_crossings else {
        return missing_data(SUCCESSFULLY_DOCKED_TARGET_PORTS, "ports", "Ferry log");
    };
    // The Channel Tunnel (tunnel_c, tunnel_d) is a train, not a port.
    let ports = crossings
        .iter()
        .filter(|crossing| !crossing.is_train())
        .flat_map(|crossing| [crossing.from.clone(), crossing.to.clone()])
        .filter(|port| !port.is_empty())
        .collect::<BTreeSet<_>>();
    let mut evaluation = set_achievement(
        &ports,
        SUCCESSFULLY_DOCKED_TARGET_PORTS,
        "ports",
        "Ports used",
    );
    evaluation.evidence.push(AchievementEvidence {
        label: "Note".to_string(),
        value: "Road to the Black Sea river ferries are in the ferry log too; whether Steam counts them for this achievement is not confirmed".to_string(),
        items: Vec::new(),
        complete: false,
    });
    evaluation
}

pub(crate) fn choo_choo(save: &SaveGame) -> AchievementEvaluation {
    ferry_crossings(save, true)
}

pub(crate) fn sardine(save: &SaveGame) -> AchievementEvaluation {
    ferry_crossings(save, false)
}

fn ferry_crossings(save: &SaveGame, train: bool) -> AchievementEvaluation {
    let Some(crossings) = &save.career.ferry_crossings else {
        return missing_data(1, "crossings", "Ferry log");
    };
    let used = crossings
        .iter()
        .filter(|crossing| crossing.is_train() == train)
        .map(|crossing| {
            format!(
                "{} → {} ({}×)",
                crossing.from, crossing.to, crossing.use_count
            )
        })
        .collect::<Vec<_>>();
    let current = used.len() as u32;

    AchievementEvaluation {
        status: status_from_progress(current, 1),
        progress: AchievementProgress {
            current,
            target: 1,
            unit: "crossings",
        },
        evidence: vec![AchievementEvidence {
            label: if train {
                "Channel Tunnel crossings".to_string()
            } else {
                "Ferry crossings".to_string()
            },
            value: summarize_list(&used),
            items: used.clone(),
            complete: current >= 1,
        }],
    }
}

pub(crate) fn head_hunter(save: &SaveGame) -> AchievementEvaluation {
    let Some(agencies) = &save.career.unlocked_recruitments else {
        return missing_data(
            HEAD_HUNTER_TARGET_AGENCIES,
            "agencies",
            "Recruitment agencies",
        );
    };
    let agencies = agencies.iter().cloned().collect::<BTreeSet<_>>();
    set_achievement(
        &agencies,
        HEAD_HUNTER_TARGET_AGENCIES,
        "agencies",
        "Discovered agencies",
    )
}

pub(crate) fn from_the_comfort_of_your_home(save: &SaveGame) -> AchievementEvaluation {
    counter(
        save.career.trucks_bought_online.map(saturating_u32),
        1,
        "trucks",
        "Trucks bought online",
    )
}

pub(crate) fn diesel_no_petrol(save: &SaveGame) -> AchievementEvaluation {
    counter(
        save.career.gas_station_visits.map(saturating_u32),
        1,
        "visits",
        "Filling station visits",
    )
}

pub(crate) fn property_magnate(save: &SaveGame) -> AchievementEvaluation {
    let Some(garages) = &save.career.garages else {
        return missing_data(PROPERTY_MAGNATE_TARGET_GARAGES, "garages", "Garages");
    };
    let owned = garages
        .iter()
        .filter(|garage| garage.size.is_owned())
        .map(|garage| garage.city.clone())
        .collect::<BTreeSet<_>>();
    set_achievement(
        &owned,
        PROPERTY_MAGNATE_TARGET_GARAGES,
        "garages",
        "Owned garages",
    )
}

pub(crate) fn go_nuclear(save: &SaveGame) -> AchievementEvaluation {
    deliveries_to_company(
        save,
        "nucleon",
        &FRENCH_NUCLEAR_PLANT_CITIES,
        3,
        "nuclear plants",
    )
}

pub(crate) fn whatever_floats_your_boat(save: &SaveGame) -> AchievementEvaluation {
    deliveries_to_company(
        save,
        "cont_port",
        &SCANDINAVIA_CONTAINER_PORT_CITIES,
        1,
        "container ports",
    )
}

pub(crate) fn captain(save: &SaveGame) -> AchievementEvaluation {
    deliveries_to_company(save, "c_navale", &[], 1, "shipyards")
}

/// Distinct destination cities of deliveries to `company` (optionally only in
/// `cities`).
fn deliveries_to_company(
    save: &SaveGame,
    company: &str,
    cities: &[&str],
    target: u32,
    unit: &'static str,
) -> AchievementEvaluation {
    let reached = save
        .delivery_log
        .entries
        .iter()
        .filter(|entry| company_id(&entry.destination_company) == company)
        .map(|entry| company_city(&entry.destination_company).to_string())
        .filter(|city| cities.is_empty() || cities.contains(&city.as_str()))
        .collect::<BTreeSet<_>>();
    let mut evaluation = set_achievement(&reached, target, unit, "Delivered to");
    if !cities.is_empty() {
        let missing = cities
            .iter()
            .filter(|city| !reached.contains(**city))
            .map(|city| (*city).to_string())
            .collect::<BTreeSet<_>>();
        evaluation.evidence.push(AchievementEvidence {
            label: "Not yet".to_string(),
            value: if missing.is_empty() {
                "none".to_string()
            } else {
                summarize_set(&missing, SUMMARY_EVIDENCE_LIMIT)
            },
            items: Vec::new(),
            complete: missing.is_empty(),
        });
    }
    evaluation
}

/// Progress towards `target` distinct values, listing them as evidence.
fn set_achievement(
    values: &BTreeSet<String>,
    target: u32,
    unit: &'static str,
    label: &str,
) -> AchievementEvaluation {
    let current = values.len() as u32;
    AchievementEvaluation {
        status: status_from_progress(current, target),
        progress: AchievementProgress {
            current,
            target,
            unit,
        },
        evidence: vec![AchievementEvidence {
            label: label.to_string(),
            value: summarize_set(values, SUMMARY_EVIDENCE_LIMIT),
            items: set_items(values),
            complete: current >= target,
        }],
    }
}

/// Progress of a single save counter towards `target`.
fn counter(
    value: Option<u32>,
    target: u32,
    unit: &'static str,
    label: &str,
) -> AchievementEvaluation {
    let Some(current) = value else {
        return missing_data(target, unit, label);
    };
    AchievementEvaluation {
        status: status_from_progress(current, target),
        progress: AchievementProgress {
            current,
            target,
            unit,
        },
        evidence: vec![AchievementEvidence {
            label: label.to_string(),
            value: format!("{current} {unit}"),
            items: Vec::new(),
            complete: current >= target,
        }],
    }
}

fn missing_data(target: u32, unit: &'static str, label: &str) -> AchievementEvaluation {
    AchievementEvaluation {
        status: AchievementStatus::InProgress,
        progress: AchievementProgress {
            current: 0,
            target,
            unit,
        },
        evidence: vec![AchievementEvidence {
            label: label.to_string(),
            value: "not found in this save".to_string(),
            items: Vec::new(),
            complete: false,
        }],
    }
}

fn saturating_u32(value: u64) -> u32 {
    u32::try_from(value).unwrap_or(u32::MAX)
}

pub(crate) fn size_matters(save: &SaveGame) -> AchievementEvaluation {
    let deliveries = oversize_deliveries(save).count() as u32;
    let cargos = oversize_cargos(save);

    AchievementEvaluation {
        status: status_from_progress(deliveries, 1),
        progress: AchievementProgress {
            current: deliveries,
            target: 1,
            unit: "oversize deliveries",
        },
        evidence: vec![AchievementEvidence {
            label: "Oversize cargoes".to_string(),
            value: summarize_set(&cargos, SUMMARY_EVIDENCE_LIMIT),
            items: set_items(&cargos),
            complete: deliveries >= 1,
        }],
    }
}

pub(crate) fn the_bigger_the_better(save: &SaveGame) -> AchievementEvaluation {
    let cargos = oversize_cargos(save);
    let current = cargos.len() as u32;

    AchievementEvaluation {
        status: status_from_progress(current, THE_BIGGER_THE_BETTER_TARGET_CARGOS),
        progress: AchievementProgress {
            current,
            target: THE_BIGGER_THE_BETTER_TARGET_CARGOS,
            unit: "oversize cargoes",
        },
        evidence: vec![AchievementEvidence {
            label: "Oversize cargoes".to_string(),
            value: summarize_set(&cargos, SUMMARY_EVIDENCE_LIMIT),
            items: set_items(&cargos),
            complete: current >= THE_BIGGER_THE_BETTER_TARGET_CARGOS,
        }],
    }
}

pub(crate) fn giant(save: &SaveGame) -> AchievementEvaluation {
    oversize_cargo_delivered(save, "silo")
}

pub(crate) fn not_a_canoe(save: &SaveGame) -> AchievementEvaluation {
    oversize_cargo_delivered(save, "pilot_boat")
}

pub(crate) fn big_brother(save: &SaveGame) -> AchievementEvaluation {
    oversize_cargo_delivered(save, "dumper")
}

pub(crate) fn mass_to_don(save: &SaveGame) -> AchievementEvaluation {
    // Consecutive deliveries in the whole log (free roam is not a delivery),
    // so any other job in between breaks the run.
    let deliveries = save
        .delivery_log
        .entries
        .iter()
        .filter(|entry| entry.job_type != FREE_ROAM_JOB_TYPE)
        .collect::<Vec<_>>();
    let best = deliveries
        .windows(3)
        .filter(|window| {
            window
                .iter()
                .all(|entry| entry.job_type == OVERSIZE_JOB_TYPE)
        })
        .map(|window| {
            let tonnes = window
                .iter()
                .map(|entry| oversize_tonnes(entry))
                .sum::<f64>();
            let cargos = window
                .iter()
                .map(|entry| special_cargo_name(cargo_id(&entry.cargo)))
                .collect::<Vec<_>>()
                .join(" + ");
            (tonnes, cargos)
        })
        .max_by(|left, right| left.0.total_cmp(&right.0));
    let current = best.as_ref().map_or(0, |(tonnes, _)| tonnes.floor() as u32);

    AchievementEvaluation {
        status: status_from_progress(current, MASS_TO_DON_TARGET_TONNES),
        progress: AchievementProgress {
            current,
            target: MASS_TO_DON_TARGET_TONNES,
            unit: "t",
        },
        evidence: vec![AchievementEvidence {
            label: "Heaviest 3 in a row".to_string(),
            value: best.map_or_else(
                || "no 3 Special Transport deliveries in a row yet".to_string(),
                |(tonnes, cargos)| format!("{tonnes:.0} t ({cargos})"),
            ),
            items: Vec::new(),
            complete: current >= MASS_TO_DON_TARGET_TONNES,
        }],
    }
}

pub(crate) fn driver_exceptionnel(save: &SaveGame) -> AchievementEvaluation {
    let driven = oversize_deliveries(save)
        .map(|entry| entry.special_route.as_str())
        .filter(|route| !route.is_empty())
        .collect::<BTreeSet<_>>();
    let base = BASE_GAME_SPECIAL_ROUTES
        .iter()
        .filter(|(id, _)| driven.contains(id))
        .map(|(_, name)| (*name).to_string())
        .collect::<BTreeSet<_>>();
    let missing = BASE_GAME_SPECIAL_ROUTES
        .iter()
        .filter(|(id, _)| !driven.contains(id))
        .map(|(_, name)| (*name).to_string())
        .collect::<BTreeSet<_>>();
    let other = driven
        .iter()
        .filter(|route| !BASE_GAME_SPECIAL_ROUTES.iter().any(|(id, _)| id == *route))
        .map(|route| (*route).to_string())
        .collect::<BTreeSet<_>>();
    let mut evaluation = set_achievement(
        &base,
        DRIVER_EXCEPTIONNEL_TARGET_ROUTES,
        "base game routes",
        "Driven",
    );
    evaluation.evidence.push(AchievementEvidence {
        label: "Not driven yet".to_string(),
        value: if missing.is_empty() {
            "none".to_string()
        } else {
            summarize_set(&missing, SUMMARY_EVIDENCE_LIMIT)
        },
        items: set_items(&missing),
        complete: missing.is_empty(),
    });
    if !other.is_empty() {
        evaluation.evidence.push(AchievementEvidence {
            label: "DLC routes (not counted)".to_string(),
            value: summarize_set(&other, SUMMARY_EVIDENCE_LIMIT),
            items: Vec::new(),
            complete: false,
        });
    }
    evaluation
}

pub(crate) fn volvo_trucks_lover(save: &SaveGame) -> AchievementEvaluation {
    cargo_delivered(save, "volvo_tr")
}

pub(crate) fn scania_trucks_lover(save: &SaveGame) -> AchievementEvaluation {
    cargo_delivered(save, "scania_tr")
}

pub(crate) fn turkish_delight(save: &SaveGame) -> AchievementEvaluation {
    let qualifying = save
        .delivery_log
        .entries
        .iter()
        .filter(|entry| {
            company_city(&entry.source_company) == ISTANBUL
                && entry.distance_km >= TURKISH_DELIGHT_MIN_DISTANCE_KM
        })
        .map(route_summary)
        .collect::<Vec<_>>();
    let current = qualifying.len() as u32;

    AchievementEvaluation {
        status: status_from_progress(current, TURKISH_DELIGHT_TARGET_DELIVERIES),
        progress: AchievementProgress {
            current,
            target: TURKISH_DELIGHT_TARGET_DELIVERIES,
            unit: "deliveries",
        },
        evidence: vec![AchievementEvidence {
            label: "From Istanbul, 2,500 km or more".to_string(),
            value: summarize_list(&qualifying),
            items: qualifying.clone(),
            complete: current >= TURKISH_DELIGHT_TARGET_DELIVERIES,
        }],
    }
}

pub(crate) fn orient_express(save: &SaveGame) -> AchievementEvaluation {
    route_legs(save, &ORIENT_EXPRESS_LEGS, true)
}

pub(crate) fn iberian_pilgrimage(save: &SaveGame) -> AchievementEvaluation {
    route_legs(save, &IBERIAN_PILGRIMAGE_LEGS, false)
}

fn oversize_deliveries(save: &SaveGame) -> impl Iterator<Item = &DeliveryLogEntry> {
    save.delivery_log
        .entries
        .iter()
        .filter(|entry| entry.job_type == OVERSIZE_JOB_TYPE)
}

/// Delivered Special Transport cargoes, by readable name.
fn oversize_cargos(save: &SaveGame) -> BTreeSet<String> {
    oversize_deliveries(save)
        .map(|entry| cargo_id(&entry.cargo))
        .filter(|cargo| !cargo.is_empty())
        .map(special_cargo_label)
        .collect()
}

/// "Giant Silo (silo)"; unknown cargo IDs are shown as they are.
fn special_cargo_label(cargo: &str) -> String {
    match special_cargo(cargo) {
        Some((id, name, _)) => format!("{name} ({id})"),
        None => cargo.to_string(),
    }
}

fn special_cargo_name(cargo: &str) -> String {
    special_cargo(cargo).map_or_else(|| cargo.to_string(), |(_, name, _)| (*name).to_string())
}

fn special_cargo(cargo: &str) -> Option<&'static (&'static str, &'static str, u32)> {
    SPECIAL_TRANSPORT_CARGOES
        .iter()
        .find(|(id, _, _)| *id == cargo)
}

/// Cargo mass from the Special Transport cargo list, falling back to the
/// delivery log's mass for cargo not in it.
fn oversize_tonnes(entry: &DeliveryLogEntry) -> f64 {
    special_cargo(cargo_id(&entry.cargo)).map_or_else(
        || entry.cargo_mass_kg.unwrap_or(0.0) / 1000.0,
        |(_, _, tonnes)| f64::from(*tonnes),
    )
}

fn oversize_cargo_delivered(save: &SaveGame, cargo: &str) -> AchievementEvaluation {
    let cargos = oversize_cargos(save);
    let deliveries = oversize_deliveries(save)
        .filter(|entry| cargo_id(&entry.cargo) == cargo)
        .count() as u32;

    let complete = deliveries >= 1;

    AchievementEvaluation {
        status: status_from_progress(deliveries, 1),
        progress: AchievementProgress {
            current: deliveries,
            target: 1,
            unit: "deliveries",
        },
        evidence: vec![
            AchievementEvidence {
                label: special_cargo_label(cargo),
                value: if complete {
                    format!("{deliveries} deliveries")
                } else {
                    "missing".to_string()
                },
                items: Vec::new(),
                complete,
            },
            AchievementEvidence {
                label: "All oversize cargoes".to_string(),
                value: summarize_set(&cargos, SUMMARY_EVIDENCE_LIMIT),
                items: set_items(&cargos),
                complete,
            },
        ],
    }
}

fn cargo_delivered(save: &SaveGame, cargo: &str) -> AchievementEvaluation {
    let deliveries = save
        .delivery_log
        .entries
        .iter()
        .filter(|entry| cargo_id(&entry.cargo) == cargo)
        .map(route_summary)
        .collect::<Vec<_>>();
    let current = deliveries.len() as u32;

    AchievementEvaluation {
        status: status_from_progress(current, 1),
        progress: AchievementProgress {
            current,
            target: 1,
            unit: "deliveries",
        },
        evidence: vec![AchievementEvidence {
            label: format!("{cargo} deliveries"),
            value: summarize_list(&deliveries),
            items: deliveries.clone(),
            complete: current >= 1,
        }],
    }
}

/// Checks which `(source city, destination city)` legs appear in the delivery
/// log, in the given direction. When `in_order` is set, each leg only counts
/// if it was delivered after the previous one (the log is oldest first).
fn route_legs(
    save: &SaveGame,
    legs: &[(&'static str, &'static str)],
    in_order: bool,
) -> AchievementEvaluation {
    let entries = &save.delivery_log.entries;
    let is_leg = |entry: &DeliveryLogEntry, from: &str, to: &str| {
        company_city(&entry.source_company) == from
            && company_city(&entry.destination_company) == to
    };
    let mut next_index = 0;
    let mut chain_broken = false;
    let evidence = legs
        .iter()
        .map(|(from, to)| {
            let complete = if in_order {
                let found = (!chain_broken)
                    .then(|| {
                        entries[next_index..]
                            .iter()
                            .position(|entry| is_leg(entry, from, to))
                    })
                    .flatten();
                match found {
                    Some(offset) => {
                        next_index += offset + 1;
                        true
                    }
                    None => {
                        chain_broken = true;
                        false
                    }
                }
            } else {
                entries.iter().any(|entry| is_leg(entry, from, to))
            };
            let count = entries
                .iter()
                .filter(|entry| is_leg(entry, from, to))
                .count();
            AchievementEvidence {
                label: format!("{from} → {to}"),
                value: match (count, complete) {
                    (0, _) => "missing".to_string(),
                    (_, true) => format!("{count} deliveries"),
                    (_, false) => format!("{count} deliveries, but not after the previous leg"),
                },
                items: Vec::new(),
                complete,
            }
        })
        .collect::<Vec<_>>();
    let current = evidence.iter().filter(|item| item.complete).count() as u32;
    let target = legs.len() as u32;

    AchievementEvaluation {
        status: status_from_progress(current, target),
        progress: AchievementProgress {
            current,
            target,
            unit: "routes",
        },
        evidence,
    }
}

fn route_summary(entry: &DeliveryLogEntry) -> String {
    format!(
        "{} → {}, {}, {} km",
        company_city(&entry.source_company),
        company_city(&entry.destination_company),
        cargo_id(&entry.cargo),
        entry.distance_km
    )
}

fn summarize_list(values: &[String]) -> String {
    if values.is_empty() {
        return "missing".to_string();
    }
    let mut summary = values
        .iter()
        .take(SUMMARY_EVIDENCE_LIMIT)
        .cloned()
        .collect::<Vec<_>>();
    if values.len() > SUMMARY_EVIDENCE_LIMIT {
        summary.push(format!("+{} more", values.len() - SUMMARY_EVIDENCE_LIMIT));
    }
    summary.join("; ")
}

fn status_from_progress(current: u32, target: u32) -> AchievementStatus {
    if current >= target {
        AchievementStatus::Complete
    } else {
        AchievementStatus::InProgress
    }
}

pub fn is_owned_truck_job_type(job_type: &str) -> bool {
    OWNED_TRUCK_JOB_TYPES.contains(&job_type)
}

pub fn achievement_categories_for_cargo(cargo: &str) -> BTreeSet<&'static str> {
    let Some(metadata) = CARGOS
        .iter()
        .find(|metadata| metadata.id == cargo_id(cargo))
    else {
        return BTreeSet::new();
    };
    achievement_categories_for_metadata(metadata)
}

fn achievement_categories_for_metadata(metadata: &CargoMetadata) -> BTreeSet<&'static str> {
    let mut categories = BTreeSet::new();
    if metadata.groups.contains(&"machinery") {
        categories.insert("Machinery");
    }
    if metadata.adr_class.is_some() || metadata.groups.contains(&"adr") {
        categories.insert("ADR cargo");
    }
    if metadata.groups.contains(&"containers") || metadata.trailer_categories.contains(&"container")
    {
        categories.insert("Container");
    }
    if metadata.groups.contains(&"refrigerated")
        || metadata.trailer_categories.contains(&"refrigerated")
    {
        categories.insert("Refrigerated");
    }
    if metadata.groups.contains(&"liquid") || metadata.trailer_categories.contains(&"tr_tank") {
        categories.insert("Liquid cargo");
    }
    if metadata.fragility.is_some()
        || metadata.id == "glass_packed"
        || metadata.groups.contains(&"inloader")
    {
        categories.insert("Fragile cargo");
    }
    if metadata.groups.contains(&"construction") {
        categories.insert("Construction");
    }
    if metadata.groups.contains(&"bulk") || metadata.trailer_categories.contains(&"bulk") {
        categories.insert("Bulk cargo");
    }
    categories
}

fn display_truck_brand(brand: &str) -> &str {
    match brand {
        "daf" => "DAF",
        "iveco" => "Iveco",
        "man" => "MAN",
        "mercedes" => "Mercedes-Benz",
        "renault" => "Renault",
        "scania" => "Scania",
        "volvo" => "Volvo",
        "volkswagen" => "Volkswagen",
        _ => brand,
    }
}

fn summarize_set(values: &BTreeSet<String>, limit: usize) -> String {
    if values.is_empty() {
        return "missing".to_string();
    }

    let mut summary = values.iter().take(limit).cloned().collect::<Vec<_>>();
    if values.len() > limit {
        summary.push(format!("+{} more", values.len() - limit));
    }
    summary.join(", ")
}

fn set_items(values: &BTreeSet<String>) -> Vec<String> {
    values.iter().cloned().collect()
}

fn delivery_summary(entry: &DeliveryLogEntry) -> String {
    format!(
        "{}, {} km, {}",
        cargo_id(&entry.cargo),
        entry.distance_km,
        format_revenue(entry.revenue)
    )
}

fn format_revenue(revenue: f64) -> String {
    if revenue.fract() == 0.0 {
        format!("{revenue:.0} EUR")
    } else {
        format!("{revenue:.2} EUR")
    }
}

fn is_profit_hunter_delivery(entry: &DeliveryLogEntry) -> bool {
    entry.revenue > PROFIT_HUNTER_TARGET_REVENUE
        && entry.distance_km >= PROFIT_HUNTER_TARGET_DISTANCE_KM
}

fn profit_hunter_criteria_met(entry: &DeliveryLogEntry) -> u32 {
    u32::from(entry.revenue > PROFIT_HUNTER_TARGET_REVENUE)
        + u32::from(entry.distance_km >= PROFIT_HUNTER_TARGET_DISTANCE_KM)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ets2::{
        BusJob, CareerStats, DailyProfit, DeliveryLog, DeliveryLogEntry, FerryCrossing, Garage,
        GarageSize,
    };

    #[test]
    fn evaluates_experience_beats_all() {
        let save = save_with_entries(vec![
            entry("cargo.digger1000", 200, "vehicle.man.tgx", "quick"),
            entry("cargo.hydrogen", 200, "vehicle.daf.xf", "quick"),
            entry("cargo.apples_c", 200, "vehicle.iveco.hiway", "quick"),
            entry("cargo.canned_beef", 200, "vehicle.mercedes.actros", "quick"),
            entry("cargo.acid", 200, "vehicle.renault.t", "quick"),
            entry("cargo.glass_packed", 200, "vehicle.scania.r", "quick"),
            entry("cargo.bricks", 200, "vehicle.volvo.fh16", "quick"),
            entry("cargo.gravel", 200, "vehicle.man.tgx", "quick"),
        ]);

        let achievement = achievement_by_id(&save, "experience_beats_all");

        assert_eq!(achievement.status, AchievementStatus::Complete);
        assert_eq!(achievement.progress.current, 8);
        assert!(achievement
            .evidence
            .iter()
            .all(|evidence| evidence.complete));
        assert!(achievement
            .evidence
            .iter()
            .any(|evidence| evidence.label == "Bulk cargo"
                && evidence.items == vec!["gravel".to_string()]));
    }

    #[test]
    fn evaluates_test_drive_limited_owned_truck_jobs_only() {
        let save = save_with_entries(vec![
            entry("cargo.gravel", 999, "vehicle.man.tgx", "cargo"),
            entry("cargo.gravel", 1000, "vehicle.daf.xf", "external"),
            entry("cargo.gravel", 1001, "vehicle.mercedes.actros", "compn"),
            entry("cargo.gravel", 1002, "vehicle.renault.t", "on_compn"),
            entry("cargo.gravel", 1003, "vehicle.scania.r", "cargo"),
            entry("cargo.gravel", 5000, "vehicle.volvo.fh16", "quick"),
            entry("cargo.gravel", 5000, "vehicle.iveco.hiway", "freerm"),
            entry("cargo.gravel", 5000, "vehicle.iveco.hiway", "spec_oversize"),
            entry("cargo.gravel", 5000, "vehicle.iveco.hiway", "unknown"),
        ]);

        let achievement = achievement_by_id(&save, "test_drive_limited");

        assert_eq!(
            achievement.display_name,
            "Test Drive Limited (5 Truck Brands)"
        );
        assert_eq!(achievement.status, AchievementStatus::Complete);
        assert_eq!(achievement.progress.current, 5);
        assert!(achievement
            .evidence
            .iter()
            .any(|evidence| evidence.label == "Mercedes-Benz" && evidence.complete));
        assert!(!achievement
            .evidence
            .iter()
            .any(|evidence| evidence.label == "Volvo"));
        // Brands are listed by distance, longest first; 999 km is enough.
        let brands = achievement
            .evidence
            .iter()
            .map(|evidence| {
                (
                    evidence.label.as_str(),
                    evidence.value.as_str(),
                    evidence.complete,
                )
            })
            .collect::<Vec<_>>();
        assert_eq!(brands[0], ("Scania", "1003 km", true));
        assert_eq!(brands[4], ("MAN", "999 km", true));

        let short = achievement_by_id(
            &save_with_entries(vec![entry("cargo.gravel", 998, "vehicle.man.tgx", "cargo")]),
            "test_drive_limited",
        );
        assert_eq!(short.evidence[0].value, "998 km");
        assert!(!short.evidence[0].complete);
    }

    #[test]
    fn evaluates_all_is_possible() {
        let mut entries = (0..32)
            .map(|index| {
                entry(
                    &format!("cargo.cargo_{index}"),
                    200,
                    "vehicle.man.tgx",
                    "quick",
                )
            })
            .collect::<Vec<_>>();
        entries.push(entry("", 200, "vehicle.man.tgx", "quick"));
        let save = save_with_entries(entries);

        let achievement = achievement_by_id(&save, "all_is_possible");

        assert_eq!(achievement.status, AchievementStatus::Complete);
        assert_eq!(achievement.progress.current, 32);
        assert_eq!(achievement.progress.target, 30);
        assert!(achievement.evidence[0].complete);
        assert!(achievement.evidence[0].value.contains("+2 more"));
        assert_eq!(achievement.evidence[0].items.len(), 32);
        assert!(!achievement.evidence[0]
            .items
            .iter()
            .any(|item| item.starts_with('+')));
    }

    #[test]
    fn evaluates_reliable_contractor() {
        let mut entries = (0..8)
            .map(|index| DeliveryLogEntry {
                source_company: format!("company.volatile.source_{index}.amsterdam"),
                destination_company: format!("company.volatile.destination_{index}.rotterdam"),
                ..entry("cargo.gravel", 200, "vehicle.man.tgx", "quick")
            })
            .collect::<Vec<_>>();
        entries.push(DeliveryLogEntry {
            source_company: "".to_string(),
            destination_company: "company.volatile.source_0.amsterdam".to_string(),
            ..entry("cargo.gravel", 200, "vehicle.man.tgx", "quick")
        });
        let save = save_with_entries(entries);

        let achievement = achievement_by_id(&save, "reliable_contractor");

        assert_eq!(achievement.status, AchievementStatus::Complete);
        assert_eq!(achievement.progress.current, 16);
        assert_eq!(achievement.progress.target, 15);
        assert!(achievement.evidence[0].value.contains("source_0"));
        assert_eq!(achievement.evidence[0].items.len(), 16);
        assert!(achievement.evidence[0]
            .items
            .iter()
            .any(|item| item == "source_0"));
    }

    #[test]
    fn evaluates_long_hauler() {
        let save = save_with_entries(vec![
            entry("cargo.gravel", 2000, "vehicle.man.tgx", "quick"),
            entry("cargo.canned_beef", 2001, "vehicle.scania.r", "quick"),
        ]);

        let achievement = achievement_by_id(&save, "long_hauler");

        assert_eq!(achievement.status, AchievementStatus::Complete);
        assert_eq!(achievement.progress.current, 2001);
        assert_eq!(achievement.progress.target, 2000);
        assert_eq!(
            achievement.evidence[0].value,
            "canned_beef, 2001 km, 1000 EUR"
        );
    }

    #[test]
    fn evaluates_profit_hunter_as_single_qualifying_job() {
        let save = save_with_entries(vec![
            revenue_entry("cargo.gravel", 2300, 130_001.0),
            revenue_entry("cargo.canned_beef", 3000, 100_000.0),
            revenue_entry("cargo.bricks", 1000, 180_000.0),
        ]);

        let achievement = achievement_by_id(&save, "profit_hunter");

        assert_eq!(achievement.status, AchievementStatus::Complete);
        assert_eq!(achievement.progress.current, 2);
        assert_eq!(achievement.evidence[0].label, "Qualifying job");
        assert_eq!(achievement.evidence[0].value, "gravel, 2300 km, 130001 EUR");
    }

    #[test]
    fn profit_hunter_stays_in_progress_when_requirements_are_split_across_jobs() {
        let save = save_with_entries(vec![
            revenue_entry("cargo.canned_beef", 3000, 100_000.0),
            revenue_entry("cargo.bricks", 1000, 180_000.0),
        ]);

        let achievement = achievement_by_id(&save, "profit_hunter");

        assert_eq!(achievement.status, AchievementStatus::InProgress);
        assert_eq!(achievement.progress.current, 1);
        assert_eq!(achievement.evidence[0].label, "Best revenue job");
    }

    #[test]
    fn registry_lists_every_steam_achievement_grouped_by_set() {
        let registry = evaluate_achievements(&save_with_entries(Vec::new()));

        assert_eq!(registry.achievements.len(), 114);
        assert_eq!(registry.groups.len(), 16);
        assert_eq!(registry.groups[0].id, "base_game");
        assert_eq!(registry.groups[0].name, "Base Game");
        assert_eq!(registry.groups[0].steam_dlc_app_id, None);

        let count_in = |group: &str| {
            registry
                .achievements
                .iter()
                .filter(|achievement| achievement.group == group)
                .count()
        };
        assert_eq!(count_in("base_game"), 29);
        assert_eq!(count_in("special_transport"), 8);
        assert_eq!(count_in("driving_academy"), 10);
        assert_eq!(count_in("iceland"), 4);
        assert_eq!(count_in("coaches"), 4);
        let coaches = registry.groups.last().unwrap();
        assert_eq!(coaches.id, "coaches");
        assert_eq!(coaches.name, "Coaches Expansion Achievements");
        assert_eq!(coaches.steam_group_id, 15);
        assert_eq!(coaches.steam_dlc_app_id, Some(3809270));
        assert!(registry
            .achievements
            .iter()
            .filter(|achievement| achievement.group == "coaches")
            .all(|achievement| achievement.status != AchievementStatus::NotTracked));
        assert_eq!(
            registry
                .groups
                .iter()
                .map(|group| count_in(group.id))
                .sum::<usize>(),
            114
        );

        assert_eq!(registry.achievements[0].steam_api_name, "map_discover_60");
        assert_eq!(
            registry.achievements[28].steam_api_name,
            "unload_difficulty"
        );

        let ids = registry
            .achievements
            .iter()
            .map(|achievement| achievement.id)
            .collect::<BTreeSet<_>>();
        let api_names = registry
            .achievements
            .iter()
            .map(|achievement| achievement.steam_api_name)
            .collect::<BTreeSet<_>>();
        assert_eq!(ids.len(), 114);
        assert_eq!(api_names.len(), 114);
    }

    #[test]
    fn delivery_log_achievements_are_tracked_and_others_are_marked_not_tracked() {
        let registry = evaluate_achievements(&save_with_entries(Vec::new()));
        let tracked = registry
            .achievements
            .iter()
            .filter(|achievement| achievement.status != AchievementStatus::NotTracked)
            .map(|achievement| achievement.id)
            .collect::<BTreeSet<_>>();

        assert_eq!(
            tracked,
            BTreeSet::from([
                // Base game
                "all_is_possible",
                "choo_choo",
                "diesel_no_petrol",
                "experience_beats_all",
                "from_the_comfort_of_your_home",
                "head_hunter",
                "i_am_a_gps",
                "long_hauler",
                "national_company",
                "pathfinder",
                "performance_optimizer",
                "profit_hunter",
                "property_magnate",
                "reliable_contractor",
                "sardine",
                "successfully_docked",
                "swimming_in_success",
                "test_drive_limited",
                "working_with_the_elite",
                // DLC sets
                "all_around_the_blue_island",
                "all_around_the_isle_of_beauty",
                "balkan_explorer",
                "baltic_tourist",
                "big_brother",
                "bon_voyage",
                "captain",
                "cattle_drive",
                "conquistador",
                "driver_exceptionnel",
                "giant",
                "go_nuclear",
                "iberian_pilgrimage",
                "imperator",
                "mass_to_don",
                "miner",
                "multitalented_driver",
                "not_a_canoe",
                "odyssean_voyager",
                "orient_express",
                "road_to_the_adriatics",
                "rule_of_threes",
                "scania_trucks_lover",
                "seasoned_traveller",
                "sightseer",
                "size_matters",
                "smooth_start",
                "the_bigger_the_better",
                "turkish_delight",
                "volvo_trucks_lover",
                "whatever_floats_your_boat",
            ])
        );

        let untracked = achievement_by_id(&save_with_entries(Vec::new()), "polar_explorer");
        assert_eq!(untracked.display_name, "Polar Explorer");
        assert_eq!(untracked.status, AchievementStatus::NotTracked);
        assert_eq!(untracked.evidence.len(), 1);
        assert!(!untracked.evidence[0].complete);
        assert!(untracked.evidence[0]
            .value
            .contains("Nordic Horizons cities"));

        // Tracked achievements whose save data is absent stay in progress.
        let no_garages = achievement_by_id(&save_with_entries(Vec::new()), "property_magnate");
        assert_eq!(no_garages.status, AchievementStatus::InProgress);
        assert_eq!(no_garages.evidence[0].value, "not found in this save");

        let academy = achievement_by_id(&save_with_entries(Vec::new()), "double_trouble");
        assert_eq!(academy.group, "driving_academy");
        assert_eq!(academy.status, AchievementStatus::NotTracked);
    }

    #[test]
    fn evaluates_special_transport_achievements_from_oversize_jobs() {
        let oversize = |cargo: &str| entry(cargo, 300, "vehicle.volvo.fh16", "spec_oversize");
        let mut entries = vec![
            oversize("cargo.silo"),
            oversize("cargo.silo"),
            oversize("cargo.pilot_boat"),
            // A regular job never counts towards Special Transport achievements.
            entry("cargo.dumper", 300, "vehicle.volvo.fh16", "cargo"),
        ];
        entries.extend((0..8).map(|index| oversize(&format!("cargo.oversize_{index}"))));
        let save = save_with_entries(entries);

        let size_matters = achievement_by_id(&save, "size_matters");
        assert_eq!(size_matters.status, AchievementStatus::Complete);
        assert_eq!(size_matters.progress.current, 11);

        let bigger = achievement_by_id(&save, "the_bigger_the_better");
        assert_eq!(bigger.status, AchievementStatus::Complete);
        assert_eq!(bigger.progress.current, 10);
        // Known cargo shows its in-game name; unknown IDs stay as they are.
        assert!(bigger.evidence[0]
            .items
            .contains(&"Giant Silo (silo)".to_string()));
        assert!(bigger.evidence[0].items.contains(&"oversize_0".to_string()));
        assert!(!bigger.evidence[0]
            .items
            .contains(&"Haul Truck Chassis (dumper)".to_string()));

        let giant = achievement_by_id(&save, "giant");
        assert_eq!(giant.status, AchievementStatus::Complete);
        assert_eq!(giant.progress.current, 2);
        assert_eq!(giant.evidence[0].label, "Giant Silo (silo)");
        assert_eq!(giant.evidence[0].value, "2 deliveries");
        assert_eq!(
            achievement_by_id(&save, "not_a_canoe").status,
            AchievementStatus::Complete
        );
        // Big Brother needs the Haul Truck Chassis as a Special Transport job.
        let big_brother = achievement_by_id(&save, "big_brother");
        assert_eq!(big_brother.status, AchievementStatus::InProgress);
        assert_eq!(big_brother.evidence[0].label, "Haul Truck Chassis (dumper)");
        assert_eq!(big_brother.evidence[0].value, "missing");
    }

    #[test]
    fn evaluates_mass_to_don_over_three_consecutive_deliveries() {
        let oversize = |cargo: &str| entry(cargo, 300, "vehicle.volvo.fh16", "spec_oversize");
        let free_roam = || entry("", 50, "vehicle.volvo.fh16", "freerm");
        // 70 + 70 + 45 = 185 t, then a regular job breaks the run.
        let save = save_with_entries(vec![
            oversize("cargo.heat_exch"),
            oversize("cargo.heat_exch"),
            oversize("cargo.boiler_parts"),
            entry("cargo.gravel", 300, "vehicle.volvo.fh16", "cargo"),
            oversize("cargo.heat_exch"),
            oversize("cargo.condensator"),
        ]);
        let mass = achievement_by_id(&save, "mass_to_don");
        assert_eq!(mass.progress.current, 185);
        assert_eq!(mass.status, AchievementStatus::InProgress);
        assert_eq!(
            mass.evidence[0].value,
            "185 t (Heat Exchanger + Heat Exchanger + Boiler Part)"
        );

        // Free roam between deliveries does not break the run: 70 + 60 + 70.
        let mut save = save;
        save.delivery_log.entries.push(free_roam());
        save.delivery_log.entries.push(oversize("cargo.heat_exch"));
        let mass = achievement_by_id(&save, "mass_to_don");
        assert_eq!(mass.progress.current, 200);
        assert_eq!(mass.status, AchievementStatus::Complete);

        // Unknown cargo falls back to the delivery log's mass.
        let save = save_with_entries(
            (0..3)
                .map(|_| DeliveryLogEntry {
                    cargo_mass_kg: Some(66_000.0),
                    ..oversize("cargo.new_part")
                })
                .collect(),
        );
        assert_eq!(
            achievement_by_id(&save, "mass_to_don").progress.current,
            198
        );
    }

    #[test]
    fn evaluates_driver_exceptionnel_on_base_game_routes() {
        let route = |id: &str| DeliveryLogEntry {
            special_route: id.to_string(),
            ..entry(
                "cargo.heat_exch",
                300,
                "vehicle.volvo.fh16",
                "spec_oversize",
            )
        };
        let save = save_with_entries(vec![
            route("lux_lieg1"),
            route("lux_lieg1"),
            route("ber_szcz1"),
            // DLC routes do not count.
            route("nic_nic1"),
            route("stav_osl1"),
        ]);

        let driver = achievement_by_id(&save, "driver_exceptionnel");

        assert_eq!(driver.progress.current, 2);
        assert_eq!(driver.progress.target, 10);
        assert_eq!(
            driver.evidence[0].items,
            vec!["Berlin – Szczecin", "Luxembourg – Liège"]
        );
        assert_eq!(driver.evidence[1].items.len(), 15);
        assert!(driver.evidence[1]
            .items
            .contains(&"Munich – Salzburg".to_string()));
        assert_eq!(driver.evidence[2].value, "nic_nic1, stav_osl1");

        let routes = [
            "lux_lieg1",
            "shef_bir1",
            "bre_tra1",
            "ber_szcz1",
            "reim_stra1",
            "dort_hann1",
            "prag_nurn1",
            "glas_aber1",
            "munch_inns1",
            "koln_amster1",
        ];
        let save = save_with_entries(routes.iter().map(|id| route(id)).collect());
        assert_eq!(
            achievement_by_id(&save, "driver_exceptionnel").status,
            AchievementStatus::Complete
        );
    }

    #[test]
    fn evaluates_truck_factory_cargo_achievements() {
        let save = save_with_entries(vec![DeliveryLogEntry {
            source_company: "company.volatile.volvo_fac.goteborg".to_string(),
            ..entry("cargo.volvo_tr", 500, "vehicle.man.tgx", "quick")
        }]);

        let volvo = achievement_by_id(&save, "volvo_trucks_lover");
        assert_eq!(volvo.status, AchievementStatus::Complete);
        assert_eq!(
            volvo.evidence[0].items,
            vec!["goteborg → amsterdam, volvo_tr, 500 km".to_string()]
        );
        assert_eq!(
            achievement_by_id(&save, "scania_trucks_lover").status,
            AchievementStatus::InProgress
        );
    }

    #[test]
    fn evaluates_turkish_delight() {
        let save = save_with_entries(vec![
            route_entry("istanbul", "lisboa", 2500),
            route_entry("istanbul", "porto", 3100),
            route_entry("istanbul", "sofia", 2499),
            route_entry("ankara", "lisboa", 3000),
        ]);

        let achievement = achievement_by_id(&save, "turkish_delight");

        assert_eq!(achievement.status, AchievementStatus::InProgress);
        assert_eq!(achievement.progress.current, 2);
        assert_eq!(achievement.progress.target, 3);
    }

    #[test]
    fn evaluates_career_counter_achievements() {
        let mut save = save_with_entries(Vec::new());
        save.career = CareerStats {
            discovered_road_km: Some(25_000.7),
            sleeping_count: Some(0),
            hq_city: Some("toulouse".to_string()),
            unlocked_recruitments: Some((0..41).map(|index| format!("city_{index}")).collect()),
            trucks_bought_online: Some(2),
            gas_station_visits: Some(38),
            garages: None,
            ferry_crossings: Some(vec![
                crossing("tunnel_c", "tunnel_d"),
                crossing("hull", "esbjerg"),
                crossing("rostock", "trelleborg"),
                crossing("esbjerg", "hull"),
            ]),
            ..CareerStats::default()
        };

        let status = |id| achievement_by_id(&save, id).status;
        assert_eq!(status("i_am_a_gps"), AchievementStatus::Complete);
        assert_eq!(status("pathfinder"), AchievementStatus::InProgress);
        let pathfinder = achievement_by_id(&save, "pathfinder");
        assert_eq!(pathfinder.progress.current, 25_000);
        assert_eq!(pathfinder.progress.target, 40_000);
        assert_eq!(pathfinder.evidence[0].value, "25000 km");
        assert_eq!(status("head_hunter"), AchievementStatus::Complete);
        assert_eq!(
            status("from_the_comfort_of_your_home"),
            AchievementStatus::Complete
        );
        assert_eq!(status("diesel_no_petrol"), AchievementStatus::Complete);
        assert_eq!(status("choo_choo"), AchievementStatus::Complete);
        assert_eq!(status("sardine"), AchievementStatus::Complete);
        // Present in the save, but not confirmed enough to track.
        assert_eq!(status("zzzzz"), AchievementStatus::NotTracked);
        // hull, esbjerg, rostock, trelleborg: the Channel Tunnel is not a port.
        let docked = achievement_by_id(&save, "successfully_docked");
        assert_eq!(docked.progress.current, 4);
        assert_eq!(docked.status, AchievementStatus::InProgress);
        assert!(!docked.evidence[0].items.contains(&"tunnel_c".to_string()));
        assert!(docked.evidence[1].value.contains("river ferries"));

        save.career.ferry_crossings = Some(vec![crossing("hull", "esbjerg")]);
        assert_eq!(
            achievement_by_id(&save, "choo_choo").status,
            AchievementStatus::InProgress
        );
    }

    #[test]
    fn evaluates_city_discovery_by_country_dlc_and_island() {
        let mut save = save_with_entries(Vec::new());
        // "clermont" and "porto_vecchi" are save IDs that differ from cities_by_dlc.json.
        save.career.visited_cities = Some(
            [
                "paris",
                "lyon",
                "clermont",
                "toulouse",
                "ajaccio",
                "bastia",
                "calvi",
                "porto_vecchi",
                "goteborg",
                "malmo",
                "milano",
                "olbia",
            ]
            .map(String::from)
            .to_vec(),
        );
        let achievement = |id| achievement_by_id(&save, id);

        let bon_voyage = achievement("bon_voyage");
        assert_eq!(bon_voyage.progress.current, 8);
        assert_eq!(bon_voyage.progress.target, 20);
        assert!(bon_voyage.evidence[0]
            .items
            .contains(&"Clermont-Ferrand".to_string()));
        assert!(bon_voyage.evidence[1].value.contains("Le Havre"));
        // Undiscovered cities are a list too.
        assert!(bon_voyage.evidence[1]
            .items
            .contains(&"Le Havre".to_string()));
        assert_eq!(
            bon_voyage.evidence[1].items.len(),
            CITIES
                .iter()
                .filter(|city| city.country == "france")
                .count()
                - 8
        );

        let corsica = achievement("all_around_the_isle_of_beauty");
        assert_eq!(corsica.progress.current, 4);
        assert_eq!(corsica.status, AchievementStatus::Complete);
        assert!(corsica.evidence[0]
            .items
            .contains(&"Porto-Vecchio".to_string()));

        assert_eq!(achievement("sightseer").progress.current, 2);
        assert_eq!(achievement("sightseer").progress.target, 15);
        // Italian cities count from the base game too (Milano).
        assert_eq!(achievement("imperator").progress.current, 2);
        let sardinia = achievement("all_around_the_blue_island");
        assert_eq!(sardinia.progress.current, 1);
        assert_eq!(sardinia.evidence[1].value, "Cagliari, Sassari");

        // Greek names also show the city ID.
        save.career.visited_cities = Some(vec!["athens".to_string()]);
        let greece = achievement_by_id(&save, "odyssean_voyager");
        assert_eq!(greece.evidence[0].value, "Αθήνα (athens)");

        // Nordic Horizons and Iceland are not in the city list.
        for id in ["polar_explorer", "land_of_fire_and_ice"] {
            assert_eq!(
                achievement_by_id(&save, id).status,
                AchievementStatus::NotTracked,
                "{id}"
            );
        }
    }

    #[test]
    fn evaluates_cattle_drive_to_scandinavia_dlc_cities() {
        let to = |company: &str, cargo: &str| DeliveryLogEntry {
            source_company: "company.volatile.aria_fd_jnpg.jonkoping".to_string(),
            destination_company: format!("company.volatile.{company}"),
            ..entry(cargo, 345, "vehicle.man.tgx", "quick")
        };
        // Hamburg is a base-game city; Aalborg is in the Scandinavia DLC.
        let save = save_with_entries(vec![
            to("aria_fd.hamburg", "cargo.live_cattle"),
            to("aria_fd_albg.aalborg", "cargo.live_pigs"),
        ]);
        let cattle = achievement_by_id(&save, "cattle_drive");
        assert_eq!(cattle.status, AchievementStatus::InProgress);
        assert_eq!(cattle.evidence[0].value, "missing");

        let save = save_with_entries(vec![to("aria_fd_albg.aalborg", "cargo.live_cattle")]);
        let cattle = achievement_by_id(&save, "cattle_drive");
        assert_eq!(cattle.status, AchievementStatus::Complete);
        assert_eq!(
            cattle.evidence[0].items,
            vec!["Jönköping → Aalborg, 345 km"]
        );
    }

    #[test]
    fn evaluates_miner_from_scandinavia_quarries() {
        let to = |company: &str| DeliveryLogEntry {
            destination_company: format!("company.volatile.{company}"),
            ..entry("cargo.excavator", 300, "vehicle.man.tgx", "quick")
        };
        let save = save_with_entries(vec![
            to("ms_stein.oslo"),
            to("ms_stein.oslo"),
            to("nord_sten.kalmar"),
            // The Glasgow quarry is in the base game.
            to("ms_stein.glasgow"),
        ]);
        let miner = achievement_by_id(&save, "miner");
        assert_eq!(miner.progress.current, 2);
        assert_eq!(miner.progress.target, 3);
        assert_eq!(
            miner.evidence[0].items,
            vec!["MS Stein (Oslo)", "Nordic Stenbrott (Kalmar)"]
        );
        assert_eq!(miner.evidence[1].items.len(), 5);

        let mut save = save;
        save.delivery_log.entries.push(to("nord_sten.vaxjo"));
        assert_eq!(
            achievement_by_id(&save, "miner").status,
            AchievementStatus::Complete
        );
    }

    #[test]
    fn evaluates_national_company_in_headquarters_country() {
        let mut save = save_with_entries(Vec::new());
        save.career.hq_city = Some("toulouse".to_string());
        save.career.garages = Some(vec![
            garage("lyon", GarageSize::Small, 0.5),
            garage("milano", GarageSize::Large, 0.5),
        ]);
        let national = achievement_by_id(&save, "national_company");
        assert_eq!(national.status, AchievementStatus::InProgress);
        assert_eq!(national.evidence[0].value, "Toulouse (france)");

        save.career
            .garages
            .as_mut()
            .unwrap()
            .push(garage("clermont", GarageSize::Large, 0.5));
        let national = achievement_by_id(&save, "national_company");
        assert_eq!(national.status, AchievementStatus::Complete);
        assert_eq!(national.evidence[1].value, "Clermont-Ferrand");
    }

    #[test]
    fn evaluates_swimming_in_success_from_garage_profit_logs() {
        let mut save = save_with_entries(Vec::new());
        save.career.current_day = Some(5870);
        let mut stuttgart = garage("stuttgart", GarageSize::Large, 0.44);
        stuttgart.daily_profit = vec![profit(5856, 40_000), profit(5868, 300_000)];
        let mut toulouse = garage("toulouse", GarageSize::Large, 0.64);
        toulouse.daily_profit = vec![profit(5868, 61_575), profit(5870, -1_000)];
        save.career.garages = Some(vec![stuttgart, toulouse]);

        let success = achievement_by_id(&save, "swimming_in_success");

        assert_eq!(success.status, AchievementStatus::InProgress);
        assert_eq!(success.progress.current, 361_575);
        assert_eq!(success.progress.target, 450_000);
        assert_eq!(success.evidence[0].value, "361575 EUR (2 days ago)");
        assert_eq!(success.evidence[1].label, "Average of 3 days");
        assert_eq!(
            success.evidence[2].value,
            "The save only keeps the last 15 days of profit (14 days ago to today), so older days are not checked"
        );

        save.career.garages.as_mut().unwrap()[1].daily_profit[0].profit = 150_000;
        assert_eq!(
            achievement_by_id(&save, "swimming_in_success").status,
            AchievementStatus::Complete
        );
    }

    #[test]
    fn evaluates_garage_productivity_from_last_seven_full_days() {
        let mut save = save_with_entries(Vec::new());
        save.career.current_day = Some(5870);
        let week = |profit_per_day: i64| {
            (5863..5870)
                .map(|day| profit(day, profit_per_day))
                .collect()
        };
        // Stockholm in the user's save: 190,296 EUR over days 5863 to 5869 = 63%.
        let mut stockholm = garage("stockholm", GarageSize::Large, 0.51);
        stockholm.daily_profit = vec![
            // Outside the window: 8 days ago and today.
            profit(5862, 500_000),
            profit(5863, 190_296),
            profit(5870, 500_000),
        ];
        let mut garages = vec![stockholm];
        garages.extend((0..5).map(|index| {
            let mut full = garage(&format!("full_{index}"), GarageSize::Large, 0.0);
            // 300,000 EUR a week is 100%.
            full.daily_profit = week(300_000 / 7 + 1);
            full
        }));
        garages.extend((0..4).map(|index| {
            let mut half = garage(&format!("half_{index}"), GarageSize::Large, 1.0);
            half.daily_profit = week(150_000 / 7 + 1);
            half
        }));
        // Small garages never count.
        let mut zurich = garage("zurich", GarageSize::Small, 1.0);
        zurich.daily_profit = week(100_000);
        garages.push(zurich);
        save.career.garages = Some(garages);

        let elite = achievement_by_id(&save, "working_with_the_elite");
        assert_eq!(elite.progress.current, 5);
        assert_eq!(elite.status, AchievementStatus::Complete);
        assert_eq!(elite.evidence[0].items.len(), 10);
        assert_eq!(
            elite.evidence[0].items[0],
            "Stockholm 63% · 190296 EUR profit in 7 days"
        );

        // (63 + 5 × 100 + 4 × 50) / 10 = 76%.
        let optimizer = achievement_by_id(&save, "performance_optimizer");
        assert_eq!(optimizer.progress.current, 76);
        assert_eq!(optimizer.progress.target, 75);
        assert_eq!(optimizer.evidence[0].value, "10 (needs 10)");
        assert_eq!(optimizer.evidence[2].value, "76% (needs 75%)");
        assert_eq!(optimizer.status, AchievementStatus::Complete);

        // An 11th, empty large garage pulls the average below 75%.
        save.career
            .garages
            .as_mut()
            .unwrap()
            .push(garage("empty", GarageSize::Large, 1.0));
        assert_eq!(
            achievement_by_id(&save, "performance_optimizer").status,
            AchievementStatus::InProgress
        );

        // Without the in-game day the window cannot be placed.
        save.career.current_day = None;
        assert_eq!(
            achievement_by_id(&save, "working_with_the_elite").evidence[0].value,
            "not found in this save"
        );
    }

    #[test]
    fn evaluates_coach_achievements() {
        let mut save = save_with_entries(Vec::new());
        save.career.bus_jobs = Some(vec![
            bus_job("quick_job", "dortmund", 26, 0.85),
            bus_job("company_job", "toulouse", 400, 1.0),
            bus_job("company_job", "clermont", 657, 1.0),
            bus_job("company_job", "metz", 299, 1.0),
            bus_job("company_job", "lyon", 2_000, 1.0),
        ]);
        save.career.coach_skills = Some(
            crate::ets2::save::COACH_SKILLS
                .iter()
                .map(|(_, label)| (*label, 1))
                .collect(),
        );

        let smooth = achievement_by_id(&save, "smooth_start");
        assert_eq!(smooth.status, AchievementStatus::Complete);
        assert_eq!(smooth.evidence[0].value, "100% (needs 90%)");
        assert_eq!(
            smooth.evidence[1].items[0],
            "Toulouse → Clermont-Ferrand, 400 km, 100%"
        );

        // Only company jobs (own coach): 400 + 657 + 299 + 2000.
        let traveller = achievement_by_id(&save, "seasoned_traveller");
        assert_eq!(traveller.progress.current, 3_356);
        assert_eq!(traveller.status, AchievementStatus::InProgress);
        assert_eq!(traveller.evidence.len(), 1);

        assert_eq!(
            achievement_by_id(&save, "multitalented_driver").status,
            AchievementStatus::Complete
        );

        // The 299 km job breaks the streak.
        let threes = achievement_by_id(&save, "rule_of_threes");
        assert_eq!(threes.progress.current, 2);
        assert_eq!(threes.status, AchievementStatus::InProgress);

        save.career.bus_jobs.as_mut().unwrap()[3].distance_km = 300;
        assert_eq!(
            achievement_by_id(&save, "rule_of_threes").status,
            AchievementStatus::Complete
        );

        // Without Coaches data these stay in progress with a clear message.
        save.career.bus_jobs = None;
        save.career.coach_skills = None;
        let smooth = achievement_by_id(&save, "smooth_start");
        assert_eq!(smooth.status, AchievementStatus::InProgress);
        assert_eq!(smooth.evidence[0].value, "not found in this save");
    }

    #[test]
    fn evaluates_property_magnate_from_owned_garages() {
        let mut save = save_with_entries(Vec::new());
        let mut garages = vec![garage("dijon", GarageSize::Tiny, 1.0)];
        garages
            .extend((0..19).map(|index| garage(&format!("small_{index}"), GarageSize::Small, 0.5)));
        garages
            .extend((0..10).map(|index| garage(&format!("large_{index}"), GarageSize::Large, 0.5)));
        garages.push(garage("grimsby", GarageSize::NotOwned, 0.0));
        save.career.garages = Some(garages);

        let magnate = achievement_by_id(&save, "property_magnate");
        assert_eq!(magnate.progress.current, 30);
        assert_eq!(magnate.status, AchievementStatus::Complete);
        assert!(!magnate.evidence[0].items.contains(&"grimsby".to_string()));

        save.career.garages.as_mut().unwrap().remove(0);
        assert_eq!(
            achievement_by_id(&save, "property_magnate").status,
            AchievementStatus::InProgress
        );
    }

    #[test]
    fn evaluates_route_legs_by_direction_and_order() {
        let orient_express = [
            ("paris", "strasbourg"),
            ("strasbourg", "munchen"),
            ("munchen", "wien"),
            ("wien", "budapest"),
            ("budapest", "bucuresti"),
            ("bucuresti", "istanbul"),
        ];
        // The second leg is driven before the first one.
        let mut entries = vec![route_entry("strasbourg", "munchen", 360)];
        entries.extend(
            orient_express
                .iter()
                .map(|(from, to)| route_entry(from, to, 500)),
        );
        let save = save_with_entries(entries[..2].to_vec());
        let orient = achievement_by_id(&save, "orient_express");
        assert_eq!(orient.progress.current, 1);
        assert_eq!(
            orient.evidence[1].value,
            "1 deliveries, but not after the previous leg"
        );

        let orient = achievement_by_id(&save_with_entries(entries), "orient_express");
        assert_eq!(orient.status, AchievementStatus::Complete);
        assert_eq!(orient.progress.current, 6);

        let pilgrimage = achievement_by_id(
            &save_with_entries(vec![
                route_entry("sevilla", "a_coruna", 900),
                route_entry("lisboa", "a_coruna", 600),
                // Wrong direction: does not count.
                route_entry("a_coruna", "pamplona", 700),
            ]),
            "iberian_pilgrimage",
        );
        // Order does not matter for this one.
        assert_eq!(pilgrimage.progress.current, 2);
        assert_eq!(pilgrimage.status, AchievementStatus::InProgress);
    }

    #[test]
    fn evaluates_company_destination_achievements() {
        let to = |company: &str, cargo: &str| DeliveryLogEntry {
            destination_company: format!("company.volatile.{company}"),
            ..entry(cargo, 300, "vehicle.man.tgx", "cargo")
        };
        let save = save_with_entries(vec![
            to("nucleon.alban", "cargo.acid"),
            to("nucleon.golfech", "cargo.acid"),
            // Not in France.
            to("nucleon.kozloduy", "cargo.acid"),
            to("c_navale.messina", "cargo.lumber"),
            to("cont_port.esbjerg", "cargo.lumber"),
        ]);

        let nuclear = achievement_by_id(&save, "go_nuclear");
        assert_eq!(nuclear.progress.current, 2);
        assert_eq!(nuclear.status, AchievementStatus::InProgress);
        assert_eq!(nuclear.evidence[1].value, "civaux, laurent, paluel");

        assert_eq!(
            achievement_by_id(&save, "captain").status,
            AchievementStatus::Complete
        );
        let boat = achievement_by_id(&save, "whatever_floats_your_boat");
        assert_eq!(boat.status, AchievementStatus::Complete);
        assert_eq!(boat.evidence[1].value, "oslo");
    }

    #[test]
    fn owned_truck_job_type_filter_matches_v1_rules() {
        for job_type in ["cargo", "external", "compn", "on_compn"] {
            assert!(is_owned_truck_job_type(job_type));
        }
        for job_type in ["quick", "freerm", "spec_oversize", "unknown", ""] {
            assert!(!is_owned_truck_job_type(job_type));
        }
    }

    #[test]
    fn derives_achievement_categories_from_cargo_metadata() {
        assert!(achievement_categories_for_cargo("cargo.digger1000").contains("Machinery"));
        assert!(achievement_categories_for_cargo("cargo.hydrogen").contains("ADR cargo"));
        assert!(achievement_categories_for_cargo("cargo.acid").contains("ADR cargo"));
        assert!(achievement_categories_for_cargo("cargo.apples_c").contains("Container"));
        assert!(achievement_categories_for_cargo("cargo.canned_beef").contains("Refrigerated"));
        assert!(achievement_categories_for_cargo("cargo.acid").contains("Liquid cargo"));
        assert!(achievement_categories_for_cargo("cargo.acid").contains("Fragile cargo"));
        assert!(achievement_categories_for_cargo("cargo.glass_packed").contains("Fragile cargo"));
        assert!(achievement_categories_for_cargo("cargo.bricks").contains("Construction"));
        assert!(achievement_categories_for_cargo("cargo.gravel").contains("Bulk cargo"));
    }

    fn save_with_entries(entries: Vec<DeliveryLogEntry>) -> SaveGame {
        SaveGame::from_delivery_log(DeliveryLog::from_entries(entries))
    }

    fn entry(cargo: &str, distance_km: u32, truck: &str, job_type: &str) -> DeliveryLogEntry {
        DeliveryLogEntry {
            source_company: "company.volatile.lkwlog.amsterdam".to_string(),
            destination_company: "company.volatile.stokes.amsterdam".to_string(),
            cargo: cargo.to_string(),
            distance_km,
            revenue: 1000.0,
            truck: truck.to_string(),
            job_type: job_type.to_string(),
            ..DeliveryLogEntry::default()
        }
    }

    fn profit(day: u32, profit: i64) -> DailyProfit {
        DailyProfit { day, profit }
    }

    fn bus_job(kind: &str, from: &str, distance_km: u32, satisfaction: f64) -> BusJob {
        BusJob {
            kind: kind.to_string(),
            from: from.to_string(),
            to: "clermont".to_string(),
            distance_km,
            satisfaction,
            start_time: 0,
        }
    }

    fn crossing(from: &str, to: &str) -> FerryCrossing {
        FerryCrossing {
            from: from.to_string(),
            to: to.to_string(),
            use_count: 1,
        }
    }

    fn garage(city: &str, size: GarageSize, productivity: f64) -> Garage {
        Garage {
            city: city.to_string(),
            size,
            productivity,
            daily_profit: Vec::new(),
        }
    }

    fn route_entry(from_city: &str, to_city: &str, distance_km: u32) -> DeliveryLogEntry {
        DeliveryLogEntry {
            source_company: format!("company.volatile.source.{from_city}"),
            destination_company: format!("company.volatile.destination.{to_city}"),
            ..entry("cargo.gravel", distance_km, "vehicle.man.tgx", "cargo")
        }
    }

    fn revenue_entry(cargo: &str, distance_km: u32, revenue: f64) -> DeliveryLogEntry {
        DeliveryLogEntry {
            revenue,
            ..entry(cargo, distance_km, "vehicle.man.tgx", "quick")
        }
    }

    fn achievement_by_id(save: &SaveGame, id: &str) -> Achievement {
        evaluate_achievements(save)
            .achievements
            .into_iter()
            .find(|achievement| achievement.id == id)
            .unwrap()
    }
}
