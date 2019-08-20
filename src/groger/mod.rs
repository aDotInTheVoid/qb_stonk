use soup::prelude::*;
use std::collections::HashMap;

const INIT_CAP: usize = 300;

fn e2none<T, E>(r: Result<T, E>) -> Option<T> {
    match r {
        Ok(v) => Some(v),
        Err(_) => None,
    }
}

pub(super) fn parse_groger_post(post: &str) -> Option<HashMap<String, (i16, f32)>> {
    let mut ret = HashMap::with_capacity(INIT_CAP);

    let parser = Soup::new(post);
    let tab = parser.tag("table").find()?;

    let tbody = &tab
        .children()
        .filter(|d| (d.name()) == "tbody")
        .collect::<Vec<_>>()[0];

    let rows = tbody.children();

    for row in rows {
        if row.display() == "\n" {
            continue;
        }

        let elems = row
            .children()
            .filter(|child| child.is_element())
            .map(|a| a.text())
            .collect::<Vec<_>>();

        if elems[0] == "OVERALL RANK" {
            continue;
        }

        let name = elems[1].clone();
        let rank: i16 = e2none(elems[0].parse()).unwrap();
        let weight: f32 = e2none(elems[2].parse()).unwrap();
        ret.insert(name, (rank, weight));
        
    }

    Some(ret)
}
