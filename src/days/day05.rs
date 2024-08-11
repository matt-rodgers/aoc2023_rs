use crate::util::answer::*;
use crate::util::input;
use anyhow::Result;

#[derive(Debug, Clone)]
struct Mapping {
    dest_start: i64,
    source_start: i64,
    length: i64,
}

pub fn run() -> Result<Answer> {
    let data = input::get_all("inputs/05.in")?;

    /* Parse input */
    let parts: Vec<&str> = data.split("\n\n").collect();

    let seeds: Vec<i64> = parts[0]
        .split(' ')
        .skip(1)
        .map(|item| item.parse().expect("could not parse int"))
        .collect();

    let maps = parts[1..]
        .iter()
        .map(|m| {
            m.split('\n')
                .skip(1)
                .filter(|item| !item.is_empty())
                .map(|item| {
                    let v = item
                        .split(' ')
                        .filter(|item| !item.is_empty())
                        .map(|num| num.parse::<i64>().expect("could not parse int"))
                        .collect::<Vec<i64>>();
                    Mapping {
                        dest_start: v[0],
                        source_start: v[1],
                        length: v[2],
                    }
                })
                .collect::<Vec<Mapping>>()
        })
        .collect::<Vec<Vec<Mapping>>>();

    let pt1: i64 = seeds
        .iter()
        .map(|seed| {
            /* For each seed (starting point) */
            let mut source = *seed;
            for mapping_list in maps.iter() {
                /* Apply each mapping step in turn */
                for mapping in mapping_list {
                    /* For each mapping step, check if source value is within source range, and if it
                     * is apply offset accordingly, otherwise keep it the same
                     */
                    let source_end = mapping.source_start + mapping.length;
                    if (mapping.source_start..source_end).contains(&source) {
                        let offset = mapping.dest_start - mapping.source_start;
                        source = source + offset;
                        break;
                    }
                }
            }
            /* Once all maps have been applied, return the result, which is the location */
            source
        })
        .min()
        .expect("could not find minimum");

    /* Part 2: convert seeds into pairs of (start, end) */
    let seed_ranges: Vec<(i64, i64)> = seeds
        .iter()
        .step_by(2)
        .zip(seeds.iter().skip(1).step_by(2))
        .map(|(i, j)| (*i, *i + *j))
        .collect();

    let mut final_locations: Vec<(i64, i64)> = Vec::new();
    for (start, end) in seed_ranges.iter() {
        /* results holds ranges that have been mapped to a new range */
        let mut results: Vec<(i64, i64)> = Vec::new();

        /* Create vector of ranges, which starts with just the initial seed range but will have
         * elements added and removed as we go through the mapping
         */
        let mut ranges: Vec<(i64, i64)> = vec![(*start, *end)];

        for m in maps.iter() {
            /* For each stage of mapping, keep going while there are ranges which have not yet had
             * all map entries applied
             */
            while let Some((mut s, mut e)) = ranges.pop() {
                /* Get a single range */
                let mut overlap_found = false;

                /* and apply all map entries to it */
                for Mapping {
                    dest_start,
                    source_start,
                    length,
                } in m
                {
                    let source_end = *source_start + *length;
                    let delta = dest_start - source_start;

                    if source_end <= s || *source_start >= e {
                        /* No overlap for this map entry */
                        continue;
                    }

                    overlap_found = true;

                    if s < *source_start {
                        /* Add non-overlapping region at start */
                        ranges.push((s, *source_start));

                        /* Update range to account for split */
                        s = *source_start;
                    }

                    if e > source_end {
                        /* Add non-overlapping region at end */
                        ranges.push((source_end, e));

                        /* Update range to account for split */
                        e = source_end;
                    }

                    /* Add overlapping region to results */
                    results.push((s + delta, e + delta));
                }

                if !overlap_found {
                    /* Add any leftover un-mapped ranges to results */
                    results.push((s, e));
                }
            }

            /* Set up for next round */
            ranges = results.clone();
            results.clear();
        }

        final_locations.extend(ranges);
    }

    let pt2 = final_locations
        .iter()
        .map(|(l, _)| l)
        .min()
        .expect("could not find minimum");

    Ok(Answer {
        pt1: pt1 as u64,
        pt2: *pt2 as u64,
    })
}
