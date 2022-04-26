use crate::*;

#[test]
fn monster_eta() {
    // no target -> -1
    let monster = Monster::new(
        0,
        Vec2 { x: 100.0, y: 100.0 },
        0,
        false,
        10,
        Vec2 {
            x: -100.0,
            y: -100.0,
        },
        None,
    );
    dbg!(monster.eta);
    assert!(monster.eta == i32::MAX);

    // Close
    let monster = Monster::new(
        0,
        Vec2 { x: 100.0, y: 100.0 },
        0,
        false,
        10,
        Vec2 {
            x: -100.0,
            y: -100.0,
        },
        Some(Vec2 { x: 0.0, y: 0.0 }),
    );
    dbg!(monster.eta);
    assert!(monster.eta == 0);

    // one turns
    let monster = Monster::new(
        0,
        Vec2 { x: 300.0, y: 300.0 },
        0,
        false,
        10,
        Vec2 {
            x: -100.0,
            y: -100.0,
        },
        Some(Vec2 { x: 0.0, y: 0.0 }),
    );
    dbg!(monster.eta);
    assert!(monster.eta == 1);

    // many turns
    let monster = Monster::new(
        0,
        Vec2 {
            x: 2358.0,
            y: 5223.0,
        },
        0,
        false,
        10,
        Vec2 {
            x: -189.0,
            y: -352.0,
        },
        Some(Vec2 { x: 0.0, y: 0.0 }),
    );
    dbg!(monster.eta);
    assert!(monster.eta == 14);
}

#[test]
fn patrol_nodes() {
    let mut patrol = Patrol::new(Vec2::MAX, 8200.0, 10.0);
    dbg!(&patrol.points);
    for _ in 0..27 {
        let p = patrol.get_next();
        dbg!((p, patrol.i, patrol.d));
    }
}
