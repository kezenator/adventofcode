use std::collections::HashSet;
use std::hash::Hash;
use std::fmt::Debug;

pub fn bron_kerbosch_maximal_cliques<'a, V, SIV, FN, FNIV>(
    verticies: SIV,
    neighbours: FN) -> Vec<HashSet<V>>
    where FN: Fn(&V) -> FNIV,
        SIV: Iterator<Item = &'a V>,
        FNIV: Iterator<Item = V>,
        V: 'a + Clone + Hash + PartialEq + Eq + Debug
{
    let p = verticies.cloned().collect::<HashSet<V>>();
    let mut r = HashSet::new();
    let x = HashSet::new();
    let mut results = Vec::new();

    apply(&mut r, p, x, &neighbours, &mut results);

    results
}

fn apply<'a, V, FN, FNIV>(r: &'a mut HashSet<V>, mut p: HashSet<V>, mut x: HashSet<V>, neighbours: &'a FN, results: &'a mut Vec<HashSet<V>>)
    where FN: 'a + Fn(&V) -> FNIV,
    FNIV: Iterator<Item = V>,
    V: 'a + Clone + Hash + PartialEq + Eq + Debug
{
    if p.is_empty() && x.is_empty()
    {
        results.push(r.clone());
        return;
    }

    while let Some(v) = p.iter().cloned().next()
    {
        // apply(R u {v}, P ^ N(v), X ^ N(v))
        {
            r.insert(v.clone());
            let n = neighbours(&v).collect::<HashSet<V>>();
            let sub_p = p.iter()
                .filter(|sp| n.contains(sp))
                .cloned()
                .collect();
            let sub_x  = x.iter()
                .filter(|sv| n.contains(sv))
                .cloned()
                .collect();
            apply(r, sub_p, sub_x, neighbours, results);
            r.remove(&v);
        }

        // P := P \ {v}
        p.remove(&v);
        // X := X u {v}
        x.insert(v);
    }
}