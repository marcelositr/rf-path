use approx::assert_relative_eq;
use rf_path::geo::{
    great_circle_distance_m, great_circle_interpolate, sample_great_circle, GeoPoint,
};

#[test]
fn project_endpoints_have_expected_distance() {
    let tx = GeoPoint::new(-20.2831, -47.7812).unwrap();
    let rx = GeoPoint::new(-20.3541, -47.8523).unwrap();

    assert_relative_eq!(
        great_circle_distance_m(tx, rx),
        10_830.336596,
        max_relative = 1e-9
    );
}

#[test]
fn sampling_has_uniform_fractions_and_exact_endpoints() {
    let a = GeoPoint::new(-20.0, -47.0).unwrap();
    let b = GeoPoint::new(-21.0, -48.0).unwrap();
    let points = sample_great_circle(a, b, 5).unwrap();

    assert_eq!(points.len(), 5);
    assert_eq!(points[0], a);
    assert_eq!(points[4], b);

    let midpoint = great_circle_interpolate(a, b, 0.5).unwrap();
    assert_relative_eq!(points[2].lat_deg, midpoint.lat_deg, max_relative = 1e-12);
    assert_relative_eq!(points[2].lon_deg, midpoint.lon_deg, max_relative = 1e-12);
}

#[test]
fn invalid_sample_count_is_rejected() {
    let a = GeoPoint::new(0.0, 0.0).unwrap();
    let b = GeoPoint::new(1.0, 1.0).unwrap();
    assert!(sample_great_circle(a, b, 1).is_err());
}
