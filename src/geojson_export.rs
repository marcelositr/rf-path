//! GeoJSON export of analysed link paths.

use std::fs;
use std::path::Path;

use geojson::{Feature, FeatureCollection, GeoJson, Geometry, Value};
use serde_json::{Map, Value as JsonValue};

use crate::analysis::LinkAnalysis;
use crate::error::Result;

/// Exports the analysed path and its sampled profile as a GeoJSON FeatureCollection.
///
/// The collection contains one LineString for the path and one Point for each
/// sampled profile row. All numeric properties come directly from `LinkAnalysis`.
pub fn export_geojson(analysis: &LinkAnalysis, output_path: impl AsRef<Path>) -> Result<()> {
    let coordinates = analysis
        .samples
        .iter()
        .map(|sample| vec![sample.position.lon_deg, sample.position.lat_deg])
        .collect::<Vec<_>>();

    let mut features = Vec::with_capacity(analysis.samples.len() + 1);
    features.push(Feature {
        bbox: None,
        geometry: Some(Geometry::new(Value::LineString(coordinates))),
        id: None,
        properties: Some(path_properties(analysis)),
        foreign_members: None,
    });

    for sample in &analysis.samples {
        let mut properties = Map::new();
        properties.insert("distance_m".into(), JsonValue::from(sample.distance_m));
        properties.insert("terrain_m".into(), JsonValue::from(sample.terrain_m));
        properties.insert("los_m".into(), JsonValue::from(sample.los_m));
        properties.insert("earth_bulge_m".into(), JsonValue::from(sample.earth_bulge_m));
        properties.insert(
            "fresnel_radius_m".into(),
            JsonValue::from(sample.fresnel_radius_m),
        );
        properties.insert("clearance_m".into(), JsonValue::from(sample.clearance_m));
        properties.insert(
            "clearance_ratio".into(),
            JsonValue::from(sample.clearance_ratio),
        );
        properties.insert(
            "status".into(),
            JsonValue::from(status_label(sample.status)),
        );

        features.push(Feature {
            bbox: None,
            geometry: Some(Geometry::new(Value::Point(vec![
                sample.position.lon_deg,
                sample.position.lat_deg,
            ]))),
            id: None,
            properties: Some(properties),
            foreign_members: None,
        });
    }

    let collection = FeatureCollection {
        bbox: None,
        features,
        foreign_members: None,
    };
    let json = GeoJson::FeatureCollection(collection).to_string();
    fs::write(output_path, json)?;
    Ok(())
}

fn path_properties(analysis: &LinkAnalysis) -> Map<String, JsonValue> {
    let mut properties = Map::new();
    properties.insert("distance_m".into(), JsonValue::from(analysis.distance_m));
    properties.insert("frequency_hz".into(), JsonValue::from(analysis.frequency_hz));
    properties.insert("wavelength_m".into(), JsonValue::from(analysis.wavelength_m));
    properties.insert("fspl_db".into(), JsonValue::from(analysis.fspl_db));
    properties.insert("los_blocked".into(), JsonValue::from(analysis.los_blocked));
    properties.insert(
        "fresnel_60_blocked".into(),
        JsonValue::from(analysis.fresnel_60_blocked),
    );
    properties
}

fn status_label(status: crate::analysis::ClearanceStatus) -> &'static str {
    match status {
        crate::analysis::ClearanceStatus::Clear => "CLEAR",
        crate::analysis::ClearanceStatus::FresnelPartial => "FRESNEL_PARTIAL",
        crate::analysis::ClearanceStatus::LineOfSightBlocked => "LOS_BLOCKED",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::analysis::{ClearanceStatus, LinkAnalysis, ProfileSample};
    use crate::geo::{AntennaPoint, GeoPoint};

    fn sample_analysis() -> LinkAnalysis {
        LinkAnalysis {
            distance_m: 100.0,
            frequency_hz: 2.4e9,
            wavelength_m: 0.1,
            tx: AntennaPoint {
                position: GeoPoint::new(0.0, 0.0).unwrap(),
                antenna_height_m: 10.0,
            },
            rx: AntennaPoint {
                position: GeoPoint::new(0.0, 0.001).unwrap(),
                antenna_height_m: 10.0,
            },
            tx_ground_m: 0.0,
            rx_ground_m: 0.0,
            tx_altitude_m: 10.0,
            rx_altitude_m: 10.0,
            fspl_db: 80.0,
            min_clearance_m: -1.0,
            min_clearance_ratio: -0.5,
            los_blocked: true,
            fresnel_60_blocked: true,
            worst_point: None,
            samples: vec![
                ProfileSample {
                    distance_m: 0.0,
                    position: GeoPoint::new(0.0, 0.0).unwrap(),
                    terrain_m: 0.0,
                    los_m: 10.0,
                    earth_bulge_m: 0.0,
                    fresnel_radius_m: 0.0,
                    clearance_m: 10.0,
                    clearance_ratio: 1.0,
                    status: ClearanceStatus::Clear,
                },
                ProfileSample {
                    distance_m: 100.0,
                    position: GeoPoint::new(0.0, 0.001).unwrap(),
                    terrain_m: 9.0,
                    los_m: 10.0,
                    earth_bulge_m: 0.0,
                    fresnel_radius_m: 0.0,
                    clearance_m: 1.0,
                    clearance_ratio: 1.0,
                    status: ClearanceStatus::Clear,
                },
            ],
        }
    }

    #[test]
    fn exports_valid_feature_collection() {
        let analysis = sample_analysis();
        let path = std::env::temp_dir().join(format!(
            "rf_path_geojson_test_{}.geojson",
            std::process::id()
        ));

        export_geojson(&analysis, &path).unwrap();
        let content = std::fs::read_to_string(&path).unwrap();
        let parsed: GeoJson = content.parse().unwrap();
        let collection = match parsed {
            GeoJson::FeatureCollection(collection) => collection,
            _ => panic!("expected FeatureCollection"),
        };

        assert_eq!(collection.features.len(), 3);
        assert!(matches!(
            collection.features[0].geometry.as_ref().unwrap().value,
            Value::LineString(_)
        ));
        assert!(matches!(
            collection.features[1].geometry.as_ref().unwrap().value,
            Value::Point(_)
        ));
        assert_eq!(
            collection.features[1]
                .properties
                .as_ref()
                .unwrap()["status"],
            JsonValue::from("CLEAR")
        );

        std::fs::remove_file(path).unwrap();
    }
}
