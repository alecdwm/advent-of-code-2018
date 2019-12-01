//! --- Day 5: Alchemical Reduction ---
//!
//! You've managed to sneak in to the prototype suit manufacturing lab. The Elves are making decent progress, but are still struggling with the suit's size reduction capabilities.

/// While the very latest in 1518 alchemical technology might have solved their problem eventually, you can do better. You scan the chemical composition of the suit's material and discover that it is formed by extremely long polymers (one of which is available as your puzzle input).
///
/// The polymer is formed by smaller units which, when triggered, react with each other such that two adjacent units of the same type and opposite polarity are destroyed. Units' types are represented by letters; units' polarity is represented by capitalization. For instance, r and R are units with the same type but opposite polarity, whereas r and s are entirely different types and do not react.
///
/// For example:
///
///     In aA, a and A react, leaving nothing behind.
///     In abBA, bB destroys itself, leaving aA. As above, this then destroys itself, leaving nothing.
///     In abAB, no two adjacent units are of the same type, and so nothing happens.
///     In aabAAB, even though aa and AA are of the same type, their polarities match, and so nothing happens.
///
/// Now, consider a larger example, dabAcCaCBAcCcaDA:
///
/// dabAcCaCBAcCcaDA  The first 'cC' is removed.
/// dabAaCBAcCcaDA    This creates 'Aa', which is removed.
/// dabCBAcCcaDA      Either 'cC' or 'Cc' are removed (the result is the same).
/// dabCBAcaDA        No further actions can be taken.
///
/// After all possible reactions, the resulting polymer contains 10 units.
///
/// How many units remain after fully reacting the polymer you scanned?
pub fn part1() {
    let input = crate::common::read_stdin_to_string();

    let mut polymer: Vec<_> = input.trim().chars().collect();

    react_polymer(&mut polymer, None);

    let number_of_units = polymer.len();

    println!(
        "the number of units remaining after fully reacting the polymer you scanned: {}",
        number_of_units
    );
}

/// Time to improve the polymer.
///
/// One of the unit types is causing problems; it's preventing the polymer from collapsing as much as it should. Your goal is to figure out which unit type is causing the most problems, remove all instances of it (regardless of polarity), fully react the remaining polymer, and measure its length.
///
/// For example, again using the polymer dabAcCaCBAcCcaDA from above:
///
///     Removing all A/a units produces dbcCCBcCcD. Fully reacting this polymer produces dbCBcD, which has length 6.
///     Removing all B/b units produces daAcCaCAcCcaDA. Fully reacting this polymer produces daCAcaDA, which has length 8.
///     Removing all C/c units produces dabAaBAaDA. Fully reacting this polymer produces daDA, which has length 4.
///     Removing all D/d units produces abAcCaCBAcCcaA. Fully reacting this polymer produces abCBAc, which has length 6.
///
/// In this example, removing all C/c units was best, producing the answer 4.
///
/// What is the length of the shortest polymer you can produce by removing all units of exactly one type and fully reacting the result?
pub fn part2() {
    let input = crate::common::read_stdin_to_string();

    let polymer: Vec<_> = input.trim().chars().collect();
    let mut shortest_polymer = polymer.len();

    let drop_units = [
        ('a', 'A'),
        ('b', 'B'),
        ('c', 'C'),
        ('d', 'D'),
        ('e', 'E'),
        ('f', 'F'),
        ('g', 'G'),
        ('h', 'H'),
        ('i', 'I'),
        ('j', 'J'),
        ('k', 'K'),
        ('l', 'L'),
        ('m', 'M'),
        ('n', 'N'),
        ('o', 'O'),
        ('p', 'P'),
        ('q', 'Q'),
        ('r', 'R'),
        ('s', 'S'),
        ('t', 'T'),
        ('u', 'U'),
        ('v', 'V'),
        ('w', 'W'),
        ('x', 'X'),
        ('y', 'Y'),
        ('z', 'Z'),
    ];

    for drop_unit in drop_units.iter() {
        let mut polymer = polymer.to_vec();
        react_polymer(&mut polymer, *drop_unit);
        if polymer.len() < shortest_polymer {
            shortest_polymer = polymer.len();
        }
    }

    println!("the length of the shortest polymer: {}", shortest_polymer);
}

fn react_polymer<T: Into<Option<(char, char)>>>(polymer: &mut Vec<char>, drop_unit: T) {
    let mut i = 0;
    let drop_unit = drop_unit.into();

    while i < polymer.len() - 1 {
        let unit = polymer[i];
        let next_unit = polymer[i + 1];

        if let Some(drop_unit) = drop_unit {
            if unit == drop_unit.0 || unit == drop_unit.1 {
                polymer.remove(i);

                if i != 0 {
                    i -= 1;
                }
                continue;
            }
            if next_unit == drop_unit.0 || next_unit == drop_unit.1 {
                polymer.remove(i + 1);
                continue;
            }
        }

        if test_unit_reaction(unit, next_unit) {
            polymer.remove(i);
            polymer.remove(i);

            if i != 0 {
                i -= 1;
            }
            continue;
        }

        i += 1;
    }
}

fn test_unit_reaction(a: char, b: char) -> bool {
    match (a.is_lowercase(), b.is_lowercase()) {
        (true, true) | (false, false) => false,
        (true, false) => a == b.to_lowercase().next().unwrap(),
        (false, true) => a.to_lowercase().next().unwrap() == b,
    }
}
