extern crate battlefy;

#[test]
fn find_team() {
    let stage_id = "5d6fdb02c747ff732da36eb4";
    let name = "Feeders";
    let persistent_team_id = "5bfe1b9418ddd9114f14efb0";

    match battlefy::Team::find(stage_id, name) {
        Ok(r) => match r {
            battlefy::SearchResult::Team(t) => assert_eq!(t.pid(), persistent_team_id),
            _ => panic!("wrong search result"),
        },
        Err(e) => panic!("error grabbing teams: {}", e),
    }
}

#[test]
fn find_match() {
    let stage_id = "5d7b716bb7758c268b771f83";
    let team_id = "5bfe1b9418ddd9114f14efb0";

    match battlefy::Match::find(stage_id, team_id, 1) {
        Ok(r) => match r {
            Some(_) => (),
            None => panic!("no match found"),
        },
        Err(e) => panic!(e),
    }
}
