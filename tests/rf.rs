//! RF calculation integration tests.

use approx::assert_relative_eq;
use rf_path::analysis::{
    classify_clearance, earth_bulge_m, effective_earth_radius_m,
    effective_reference_path_elevation_m, reference_path_elevation_m, ClearanceStatus,
    DEFAULT_FRESNEL_CLEARANCE_RATIO, DEFAULT_K_FACTOR,
};

#[test]
fn effective_earth_radius_matches_reference_vector() {
    let radius = effective_earth_radius_m(DEFAULT_K_FACTOR).unwrap();
    assert_relative_eq!(radius, 8_494_666.666_666_666, max_relative = 1e-12);
}

#[test]
fn earth_bulge_matches_python_reference_vector() {
    let bulge = earth_bulge_m(5_000.0, 5_000.0, DEFAULT_K_FACTOR).unwrap();
    assert_relative_eq!(bulge, 1.471_511_536_650_447_4, max_relative = 1e-12);
}

#[test]
fn reference_path_and_effective_path_match_reference_vectors() {
    let linear = reference_path_elevation_m(100.0, 300.0, 5_000.0, 5_000.0).unwrap();
    let effective = effective_reference_path_elevation_m(
        100.0,
        300.0,
        5_000.0,
        5_000.0,
        DEFAULT_K_FACTOR,
    )
    .unwrap();

    assert_relative_eq!(linear, 200.0, max_relative = 1e-12);
    assert_relative_eq!(effective, 198.528_488_463_349_55, max_relative = 1e-12);
}

#[test]
fn sixty_percent_fresnel_classification_matches_specification() {
    assert_eq!(
        classify_clearance(-0.01, 10.0, DEFAULT_FRESNEL_CLEARANCE_RATIO).unwrap(),
        ClearanceStatus::LineOfSightBlocked
    );
    assert_eq!(
        classify_clearance(5.99, 10.0, DEFAULT_FRESNEL_CLEARANCE_RATIO).unwrap(),
        ClearanceStatus::FresnelPartial
    );
    assert_eq!(
        classify_clearance(6.0, 10.0, DEFAULT_FRESNEL_CLEARANCE_RATIO).unwrap(),
        ClearanceStatus::Clear
    );
}
