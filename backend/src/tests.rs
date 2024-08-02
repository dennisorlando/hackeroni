

#[test]
fn request() {
    use crate::osrm::request::{Request, Service, Profile};
    let service = Service::Route { alternatives: Some(2), steps: true };
    let coordinates = vec![(37.828844, -122.245773), (37.828839, -122.245779)];
    let req = Request::new(service, coordinates, Profile::Foot);

    println!("{}", req.to_osrm_string());
}
