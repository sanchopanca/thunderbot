use dashmap::DashMap;
use lazy_static::lazy_static;
use rand::seq::SliceRandom;
use rand::thread_rng;

lazy_static! {
    static ref BOT_RULES: DashMap<String, Vec<String>> = {
        let rules = DashMap::new();
        let kpop = vec![
            String::from("https://youtu.be/9bZkp7q19f0"),
            String::from("https://youtu.be/POe9SOEKotk"),
            String::from("https://youtu.be/5UdoUmvu_n8"),
            String::from("https://youtu.be/2e-Q7GfCGbA"),
            String::from("https://youtu.be/id6q2EP2UqE"),
            String::from("https://youtu.be/8dJyRm2jJ-U"),
            String::from("https://youtu.be/JQGRg8XBnB4"),
            String::from("https://youtu.be/Hbb5GPxXF1w"),
            String::from("https://youtu.be/p1bjnyDqI9k"),
            String::from("https://youtu.be/k6jqx9kZgPM"),
            String::from("https://youtu.be/z8Eu-HU0sWQ"),
            String::from("https://youtu.be/eH8jn4W8Bqc"),
            String::from("https://youtu.be/IHNzOHi8sJs"),
            String::from("https://youtu.be/WPdWvnAAurg"),
            String::from("https://youtu.be/gdZLi9oWNZg"),
            String::from("https://youtu.be/H8kqPkEXP_E"),
            String::from("https://youtu.be/awkkyBH2zEo"),
            String::from("https://youtu.be/z3szNvgQxHo"),
            String::from("https://youtu.be/i0p1bmr0EmE"),
            String::from("https://youtu.be/WyiIGEHQP8o"),
            String::from("https://youtu.be/lcRV2gyEfMo"),
        ];
        rules.insert(String::from("kpop time"), kpop.clone());
        rules.insert(String::from("k p o p   t i m e"), kpop.clone());
        rules.insert(String::from("kpop tijd"), kpop);
        rules.insert(
            String::from("hat a week huh"),
            vec![String::from("https://whataweek.eu")],
        );
        rules.insert(
            String::from("hat a week huh"),
            vec![String::from("https://whataweek.eu")],
        );
        rules.insert(
            String::from("(╯°□°)╯︵ ┻━┻"),
            vec![String::from("┬─┬ノ(º_ºノ)")],
        );
        rules
    };
}

#[allow(dead_code)]
fn match_message(message: &str, patterns: &[&str]) -> bool {
    patterns.iter().any(|p| message.contains(p))
}

fn random_choice(v: &[String]) -> &str {
    v.choose(&mut thread_rng()).unwrap() // todo: empty vector
}

pub fn respond(message: &str) -> Option<String> {
    for entry in BOT_RULES.iter() {
        let prompt = entry.key();
        let responses = entry.value();
        if message.contains(prompt) {
            return Some(String::from(random_choice(responses)));
        }
    }
    None
}

#[allow(dead_code)]
fn save_rule(pattern: String, mut responses: Vec<String>) {
    match BOT_RULES.get_mut(&pattern) {
        None => {
            BOT_RULES.insert(pattern, responses);
        }
        Some(mut entry) => {
            let value_ptr = entry.value_mut();
            (*value_ptr).append(&mut responses);
        }
    }
}

#[allow(dead_code)]
fn save_rule_single_response(pattern: String, response: String) {
    save_rule(pattern, vec![response]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn match_message_works() {
        let patterns = &["kpop time", "kpop tijd"];
        assert!(match_message("Is it kpop time yet", patterns));
        assert!(match_message("Is het al kpop tijd?", patterns));
        assert!(!match_message("It's Britney time", patterns));
    }
}
