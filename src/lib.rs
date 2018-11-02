use std::fmt::{Debug, Display};
use std::cmp::PartialEq;

#[derive(Clone, Debug)]
pub struct Operator<T> {
    action: T,
    preconds: Vec<T>,
    add: Vec<T>,
    delete: Vec<T>,
}

#[derive(Clone, Debug)]
struct Problem<T> {
    start: Vec<T>,
    finish: Vec<T>,
    ops: Vec<Operator<T>>,
}

pub fn problem_solver<T>(
    initial_states: &mut [T],
    goal_states: &mut [T],
    operators: &mut[Operator<T>],
) -> Option<Vec<T>>
where
    T: Clone + Debug + Display + From<String> + PartialEq
{
    let prefix = String::from("Executing ");
    for operator in operators.iter_mut() {
        operator.add.push(T::from(format!("{}{}", prefix, operator.action)));
    }
    let mut goal_stack = Vec::new();
    let mut result_actions = Vec::new();
    let final_states = achieve_all(
        initial_states, operators, goal_states, &mut goal_stack, &mut result_actions);
    match final_states {
        None => None,
        // Some(f_s) => Some(f_s.iter().filter(|a| a.starts_with(&prefix)).cloned().collect())
        Some(..) => Some(result_actions),
     }
}

fn achieve_all<T>(
    states: &[T],
    ops: &[Operator<T>],
    goals: &[T],
    goal_stack: &mut[T],
    result_actions: &mut Vec<T>,
) -> Option<Vec<T>>
where
    T: Clone + Debug + Display + From<String> + PartialEq
{
    let mut st = Some((*states).to_vec());
    let mut _st = st.unwrap();
    for goal in goals {
        st = achieve(_st, ops, &goal, goal_stack, result_actions);
        match st {
            None => return None,
            Some(v) => _st = v,
        }
    }
    for goal in goals {
        if !_st.contains(goal) {
            return None;
        }
    }
    Some(_st)
}

fn achieve<T>(
    states: Vec<T>,
    operators: &[Operator<T>],
    goal: &T,
    goal_stack: &mut[T],
    result_actions: &mut Vec<T>,
) -> Option<Vec<T>>
where
    T: Clone + Debug + Display + From<String> + PartialEq
{
    // println!("achieve() : size goal stack : {:?} | Achieving {:?}", goal_stack.len(), goal);
    if states.contains(goal) {
        return Some(states);
    }
    for op in operators.iter() {
        if !op.add.contains(&goal) {
             continue;
        }
        let res = apply_operator(
            op, &states, operators, goal, goal_stack, result_actions);
        if res.is_some() {
            return res;
        }
    }
    None
}

fn apply_operator<T>(
    op: &Operator<T>,
    states: &[T],
    operators: &[Operator<T>],
    goal: &T,
    goal_stack: &mut[T],
    result_actions: &mut Vec<T>,
) -> Option<Vec<T>>
where
    T: Clone + Debug + Display + From<String> + PartialEq
{
        // println!("apply_operator() : size goal stack : {:?} | Consider {:?}", goal_stack.len(), op.action);
        let mut _gs: Vec<T> = goal_stack.to_vec();
        _gs.push(goal.clone());
        let result = achieve_all(states, operators, &op.preconds, &mut _gs, result_actions);
        match result {
            None => None,
            Some(res) => {
                // println!("apply_operator() : size goal stack : {:?} | Action {:?}", goal_stack.len(), op.action);
                result_actions.push(op.action.clone());
                let (mut add_list, del_list) = (op.add.clone(), op.delete.clone());
                let mut r: Vec<T>= res.iter()
                    .cloned()
                    .filter(|a| !del_list.contains(a))
                    .collect();
                r.append(&mut add_list);
                Some(r)
            }
        }
}

#[cfg(test)]
mod tests {
    use {Problem, Operator, problem_solver};

    #[test]
    fn it_works() {
        let mut example = Problem {
            start: vec!["at door".to_string(), "on floor".to_string(), "has ball".to_string(), "hungry".to_string(), "chair at door".to_string()],
            finish: vec!["not hungry".to_string()],
            ops: vec![
            	Operator {
            	    action: "climb on chair".to_string(),
            	    preconds: vec!["chair at middle room".to_string(), "at middle room".to_string(), "on floor".to_string()],
            	    add: vec!["at bananas".to_string(), "on chair".to_string()],
            	    delete: vec!["at middle room".to_string(), "on floor".to_string()]
            	},
            	Operator {
            	    action: "push chair from door to middle room".to_string(),
            	    preconds: vec!["chair at door".to_string(), "at door".to_string()],
            	    add: vec!["chair at middle room".to_string(), "at middle room".to_string()],
            	    delete: vec!["chair at door".to_string(), "at door".to_string()]
            	},
            	Operator {
            	    action: "walk from door to middle room".to_string(),
            	    preconds: vec!["at door".to_string(), "on floor".to_string()],
            	    add: vec!["at middle room".to_string()],
            	    delete: vec!["at door".to_string()]
            	},
            	Operator {
            	    action: "grasp bananas".to_string(),
            	    preconds: vec!["at bananas".to_string(), "empty handed".to_string()],
            	    add: vec!["has bananas".to_string()],
            	    delete: vec!["empty handed".to_string()]
            	},
            	Operator {
            	    action: "drop ball".to_string(),
            	    preconds: vec!["has ball".to_string()],
            	    add: vec!["empty handed".to_string()],
            	    delete: vec!["has ball".to_string()]
            	},
            	Operator {
            	    action: "eat bananas".to_string(),
            	    preconds: vec!["has bananas".to_string()],
            	    add: vec!["empty handed".to_string(), "not hungry".to_string()],
            	    delete: vec!["has bananas".to_string(), "hungry".to_string()]
            	}
            ]
        };
        let res = problem_solver(&mut example.start, &mut example.finish, &mut example.ops);
        assert_eq!(res, Some(vec![
            "push chair from door to middle room".to_string(),
            "climb on chair".to_string(),
            "drop ball".to_string(),
            "grasp bananas".to_string(),
            "eat bananas".to_string()]));
    }
}


// #[derive(Clone, Debug)]
// pub struct Operator {
//     action: String,
//     preconds: Vec<String>,
//     add: Vec<String>,
//     delete: Vec<String>,
// }
//
// #[derive(Clone, Debug)]
// struct Problem {
//     start: Vec<String>,
//     finish: Vec<String>,
//     ops: Vec<Operator>,
// }
//
// pub fn gps(initial_states: &mut [String], goal_states: &mut [String], operators: &mut[Operator]) -> Option<Vec<String>>{
//     let prefix = String::from("Executing ");
//     for operator in operators.iter_mut() {
//         operator.add.push(format!("{}{}", prefix, operator.action));
//     }
//     let mut goal_stack = Vec::new();
//     let final_states = achieve_all(initial_states, operators, goal_states, &mut goal_stack);
//     println!("{:?}", operators);
//     match final_states {
//         None => None,
//         Some(f_s) => {
//             println!("{:?}", f_s);
//             Some(f_s.iter().filter(|a| a.starts_with(&prefix)).cloned().collect())
//         }
//      }
// }
//
// fn achieve_all(states: &[String], ops: &[Operator], goals: &[String], goal_stack: &mut[String]) -> Option<Vec<String>> {
//     let mut st = Some((*states).to_vec());
//     let mut _st = st.unwrap();
//     for goal in goals {
//         st = achieve(_st, ops, &goal, goal_stack);
//         match st {
//             None => return None,
//             Some(v) => _st = v,
//         }
//     }
//     for goal in goals {
//         if !_st.contains(goal) {
//             return None;
//         }
//     }
//     Some(_st)
// }
//
// fn achieve(states: Vec<String>, operators: &[Operator], goal: &String, goal_stack: &mut[String]) -> Option<Vec<String>> {
//     println!("achieve() : size goal stack : {:?} | Achieving {:?}", goal_stack.len(), goal);
//     if states.contains(goal) {
//         return Some(states);
//     }
//     for op in operators.iter() {
//         if !op.add.contains(goal) {
//              continue;
//         }
//         let res = apply_operator(op, &states, operators, goal, goal_stack);
//         if res.is_some() {
//             return res;
//         }
//     }
//     None
// }
//
// fn apply_operator(op: &Operator, states: &[String], operators: &[Operator], goal: &str, goal_stack: &mut[String]) -> Option<Vec<String>> {
//         println!("apply_operator() : size goal stack : {:?} | Consider {:?}", goal_stack.len(), op.action);
//         let mut _gs: Vec<String> = goal_stack.to_vec();
//         _gs.push(goal.to_string());
//         let result = achieve_all(states, operators, &op.preconds, &mut _gs);
//         match result {
//             None => None,
//             Some(res) => {
//                 println!("apply_operator() : size goal stack : {:?} | Action {:?}", goal_stack.len(), op.action);
//                 let (mut add_list, delete_list) = (op.add.clone(), op.delete.clone());
//                 let mut r: Vec<String>= res.iter().cloned().filter(|a| !delete_list.contains(a)).collect();
//                 r.append(&mut add_list);
//                 Some(r)
//             }
//         }
// }
