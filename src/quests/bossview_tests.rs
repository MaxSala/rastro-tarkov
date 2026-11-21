use super::*;

fn boss(name: &str, chance: f64, time: Option<u32>) -> Boss {
    Boss {
        id: "x".into(),
        name: name.into(),
        map: "lighthouse".into(),
        chance,
        time,
        triggered: false,
        escorts: vec![],
        zones: vec![],
    }
}

#[test]
fn the_central_point_is_a_real_spawn() {
    let pts = [[0.0, 0.0, 0.0], [10.0, 0.0, 0.0], [11.0, 0.0, 0.0], [12.0, 0.0, 0.0], [13.0, 0.0, 0.0]];
    assert_eq!(central(&pts), Some([11.0, 0.0, 0.0]));
    assert_eq!(central(&[]), None);
}

#[test]
fn the_list_has_one_line_per_boss() {
    let (a, b, c) = (boss("Rogue", 0.5, None), boss("Rogue", 0.8, None), boss("Partisan", 0.1, Some(900)));
    let l = lines(&[&a, &b, &c]);
    assert_eq!(l.len(), 2);
    assert_eq!((l[0].name.as_str(), l[0].chance), ("Rogue", 0.8), "likeliest first");
    assert_eq!(l[1].time, Some(900));
}
