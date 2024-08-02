


#[test]
fn request() {
    use crate::osrm::request::{PathRequest, Service};
    use crate::osrm::request::Preferences;

    let service = Service::Route { alternatives: Some(2), steps: true };
    let coordinates = vec![(37.828844, -122.245773), (37.828839, -122.245779)];
    let preferences = Preferences::new(Some(42), Some(88), Some(10));
    let req = PathRequest::new(coordinates, preferences);


    println!("{:?}\n", serde_json::to_string(&req).unwrap());

    println!("{}", req.to_osrm_string());
}
