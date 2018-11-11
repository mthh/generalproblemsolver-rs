#[macro_use]extern crate serde_derive;
extern crate serde;
extern crate serde_json;

use std::fmt::{Debug, Display};
use std::cmp::PartialEq;

#[derive(Clone, Debug, Deserialize)]
pub struct Operator<T> {
    action: T,
    preconds: Vec<T>,
    add: Vec<T>,
    delete: Vec<T>,
}

#[derive(Clone, Debug, Deserialize)]
struct Problem<T> {
    start: Vec<T>,
    finish: Vec<T>,
    ops: Vec<Operator<T>>,
}

pub fn problem_solver<T>(
    initial_states: &[T],
    goal_states: &[T],
    operators: &[Operator<T>],
) -> Option<Vec<T>>
where
    T: Clone + Debug + Display + From<String> + PartialEq
{
    let mut goal_stack = Vec::new();
    let mut result_actions = Vec::new();
    let final_states = achieve_all(
        initial_states, operators, goal_states, &mut goal_stack, &mut result_actions);
    match final_states {
        None => None,
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
        let mut _gs: Vec<T> = goal_stack.to_vec();
        _gs.push(goal.clone());
        let result = achieve_all(states, operators, &op.preconds, &mut _gs, result_actions);
        match result {
            None => None,
            Some(res) => {
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
    use serde_json;
    #[test]
    fn test_monkey_problem() {
        let example = Problem {
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
        let res = problem_solver(&example.start, &example.finish, &example.ops);
        println!("{:?}", example);
        assert_eq!(res, Some(vec![
            "push chair from door to middle room".to_string(),
            "climb on chair".to_string(),
            "drop ball".to_string(),
            "grasp bananas".to_string(),
            "eat bananas".to_string()]));
    }

    #[test]
    fn test_monkey_problem_from_json() {
        let str_example = r#"{
        "start": ["at door", "on floor", "has ball", "hungry", "chair at door"],
        "finish": ["not hungry"],
        "ops": [
    	{
    	    "action": "climb on chair",
    	    "preconds": ["chair at middle room", "at middle room", "on floor"],
    	    "add": ["at bananas", "on chair"],
    	    "delete": ["at middle room", "on floor"]
    	},
    	{
    	    "action": "push chair from door to middle room",
    	    "preconds": ["chair at door", "at door"],
    	    "add": ["chair at middle room", "at middle room"],
    	    "delete": ["chair at door", "at door"]
    	},
    	{
    	    "action": "walk from door to middle room",
    	    "preconds": ["at door", "on floor"],
    	    "add": ["at middle room"],
    	    "delete": ["at door"]
    	},
    	{
    	    "action": "grasp bananas",
    	    "preconds": ["at bananas", "empty handed"],
    	    "add": ["has bananas"],
    	    "delete": ["empty handed"]
    	},
    	{
    	    "action": "drop ball",
    	    "preconds": ["has ball"],
    	    "add": ["empty handed"],
    	    "delete": ["has ball"]
    	},
    	{
    	    "action": "eat bananas",
    	    "preconds": ["has bananas"],
    	    "add": ["empty handed", "not hungry"],
    	    "delete": ["has bananas", "hungry"]
    	}
    ]
}"#;

    let example: Problem<String> = serde_json::from_str(str_example).unwrap();
    let res = problem_solver(&example.start, &example.finish, &example.ops);
    assert_eq!(res, Some(vec![
        "push chair from door to middle room".to_string(),
        "climb on chair".to_string(),
        "drop ball".to_string(),
        "grasp bananas".to_string(),
        "eat bananas".to_string()]));

    }

    #[test]
    fn test_baseball_problem() {
        let str_example = r#"{
                "start": ["hand empty", "arm down"],
                "finish": ["satisfied", "baseball in air"],
                "ops": [
                {
                    "action": "raise arm",
                    "preconds": ["arm down"],
                    "add": ["arm up", "raising arm"],
                    "delete": ["arm down"]
                },
                {
                    "action": "throw baseball",
                    "preconds": ["have baseball", "arm up"],
                    "add": ["arm down", "baseball in air", "throwing baseball"],
                    "delete": ["have baseball", "arm up"]
                },
                {
                    "action": "grab baseball",
                    "preconds": ["hand empty", "arm down"],
                    "add": ["have baseball", "grabbing baseball"],
                    "delete": ["hand empty"]
                },
                {
                    "action": "drink beer",
                    "preconds": ["arm down", "hand empty"],
                    "add": ["satisfied", "drinking beer"],
                    "delete": []
                }
            ]
        }"#;
        let example: Problem<String> = serde_json::from_str(str_example).unwrap();
        let res = problem_solver(&example.start, &example.finish, &example.ops);
        assert_eq!(res, Some(vec![
            "drink beer".to_string(),
            "grab baseball".to_string(),
            "raise arm".to_string(),
            "throw baseball".to_string()]));
    }
}
