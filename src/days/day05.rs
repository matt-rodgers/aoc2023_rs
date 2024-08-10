use crate::util::answer::*;
use crate::util::input;
use anyhow::Result;

#[derive(Debug)]
struct Mapping {
    dest_start: i64,
    source_start: i64,
    length: i64,
}

pub fn run() -> Result<Answer> {
    let pt2 = 0;

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

    Ok(Answer {
        pt1: pt1 as u64,
        pt2: pt2 as u64,
    })
}
