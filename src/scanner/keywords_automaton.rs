use std::collections::HashMap;


// struct KeywordsAutomaton {
//     transitions: HashMap<(&'static str, char), &'static str>,
//     initial_state: &'static str
// }

// impl KeywordsAutomaton {
//     fn transition(&self, state: &'static str, input: char) -> Option<&str> {
//        self.transitions.get(&(state, input)).cloned()
//     }
// }

pub fn check_keywords(symbols: &Vec<char>, index: &mut usize, n: &usize) -> Option<String> {

    let transits: HashMap<(&'static str, char), &'static str> = HashMap::from([
        (("q0", 'A'), "q1"),
        (("q1", 'N'), "q2"),
        (("q2", 'D'), "qf"),

        (("q0", 'C'), "q4"),
        (("q4", 'L'), "q5"),
        (("q5", 'A'), "q6"),
        (("q6", 'S'), "q7"),
        (("q7", 'S'), "qf"),

        (("q0", 'E'), "q10"),
        (("q10", 'L'), "q11"),
        (("q11", 'S'), "q12"),
        (("q12", 'E'), "qf"),

        (("q0", 'F'), "q14"),
        (("q14", 'A'), "q15"),
        (("q15", 'L'), "q16"),
        (("q16", 'S'), "q17"),
        (("q17", 'E'), "qf"),

        (("q14", 'O'), "q19"),
        (("q19", 'R'), "qf"),
        (("q20", ' '), "qf"),

        (("q14", 'U'), "q22"),
        (("q22", 'N'), "qf"),

        (("q0", 'I'), "q24"),
        (("q24", 'F'), "qf"),

        (("q0", 'N'), "q26"),
        (("q26", 'I'), "q27"),
        (("q27", 'L'), "qf"),

        (("q0", 'O'), "q30"),
        (("q30", 'R'), "qf"),

        (("q0", 'P'), "q32"),
        (("q32", 'R'), "q33"),
        (("q33", 'I'), "q34"),
        (("q34", 'N'), "q35"),
        (("q35", 'T'), "qf"),
    
        (("q0", 'R'), "q37"),
        (("q37", 'E'), "q39"),
        (("q39", 'T'), "q40"),
        (("q40", 'U'), "q41"),
        (("q41", 'R'), "q42"),
        (("q42", 'N'), "qf"),

        (("q0", 'S'), "q38"),
        (("q38", 'U'), "q44"),
        (("q44", 'P'), "q45"),
        (("q45", 'E'), "q46"),
        (("q46", 'R'), "qf"),

        (("q0", 'T'), "q48"),
        (("q48", 'R'), "q49"),
        (("q49", 'U'), "q50"),
        (("q50", 'E'), "qf"),

        (("q48", 'H'), "q53"),
        (("q53", 'I'), "q54"),
        (("q54", 'S'), "qf"),


        (("q0", 'V'), "q56"),
        (("q56", 'A'), "q57"),
        (("q57", 'R'), "qf"),


        (("q0", 'W'), "q59"),
        (("q59", 'H'), "q60"),
        (("q60", 'I'), "q61"),
        (("q61", 'L'), "q62"),
        (("q62", 'E'), "qf"),

    ]);

    let mut keyword = String::new();
    // let automaton = KeywordsAutomaton{
    //     transitions: transits,
    //     initial_state: "q0"
    // };
    let mut next_state = "q0";
    
    while *index < *n  && next_state != "qf" {
        let c = symbols[*index];
        match transits.get(&(next_state, c)) {
            Some(new_state) => {
                next_state = *new_state;
                keyword.push(c);
                *index += 1;
            },
            None => {
                break;
            }
        }; 
    }
    
    if next_state != "qf" || (*index < *n && symbols[*index].is_alphanumeric()) {
        return None;
    }
    Some(keyword)
}