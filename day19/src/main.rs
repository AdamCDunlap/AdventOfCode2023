use std::{collections::HashMap, str::FromStr};

use regex::Regex;

#[derive(Debug)]
enum Category {
    X,
    M,
    A,
    S,
}

impl FromStr for Category {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Category::*;
        Ok(match s {
            "x" => X,
            "m" => M,
            "a" => A,
            "s" => S,
            _ => return Err(()),
        })
    }
}

#[derive(Debug, Clone, Copy)]
enum Inequality {
    Less,
    Greater,
}

impl FromStr for Inequality {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            ">" => Inequality::Greater,
            "<" => Inequality::Less,
            _ => return Err(()),
        })
    }
}

#[derive(Debug)]
struct RuleCondition {
    category: Category,
    inequality: Inequality,
    compare_val: i64,
}

impl FromStr for RuleCondition {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let category = s[0..1].parse()?;
        let inequality = s[1..2].parse()?;
        let compare_val = s[2..].parse().unwrap();

        Ok(Self {
            category,
            inequality,
            compare_val,
        })
    }
}

impl RuleCondition {
    fn is_applicable(&self, part: &Part) -> bool {
        let op = |n: i64| match self.inequality {
            Inequality::Greater => n > self.compare_val,
            Inequality::Less => n < self.compare_val,
        };

        match self.category {
            Category::X => op(part.x),
            Category::M => op(part.m),
            Category::A => op(part.a),
            Category::S => op(part.s),
        }
    }

    fn get_relevant_num_range_mut<'a>(&'a self, part_range: &'a mut PartRange) -> &'a mut Range {
        match self.category {
            Category::X => &mut part_range.x,
            Category::M => &mut part_range.m,
            Category::A => &mut part_range.a,
            Category::S => &mut part_range.s,
        }
    }
    fn get_relevant_num_range<'a>(&'a self, part_range: &'a PartRange) -> &'a Range {
        match self.category {
            Category::X => &part_range.x,
            Category::M => &part_range.m,
            Category::A => &part_range.a,
            Category::S => &part_range.s,
        }
    }

    fn split_range(&self, part_range: &PartRange) -> (Option<PartRange>, Option<PartRange>) {
        let relevant_num_range = self.get_relevant_num_range(&part_range);

        let (matching_range, nonmatching_range) = match self.inequality {
            Inequality::Less => (
                Range::try_new(
                    relevant_num_range.min,
                    std::cmp::min(self.compare_val - 1, relevant_num_range.max),
                ),
                Range::try_new(
                    std::cmp::max(self.compare_val, relevant_num_range.min),
                    relevant_num_range.max,
                ),
            ),
            Inequality::Greater => (
                Range::try_new(
                    std::cmp::max(self.compare_val + 1, relevant_num_range.min),
                    relevant_num_range.max,
                ),
                Range::try_new(
                    relevant_num_range.min,
                    std::cmp::min(self.compare_val, relevant_num_range.max),
                ),
            ),
        };

        let matching_part_range = if let Some(matching_range) = matching_range {
            let mut matching_part_range = part_range.clone();
            *self.get_relevant_num_range_mut(&mut matching_part_range) = matching_range;
            Some(matching_part_range)
        } else {
            None
        };

        let nonmatching_part_range = if let Some(nonmatching_range) = nonmatching_range {
            let mut nonmatching_part_range = part_range.clone();
            *self.get_relevant_num_range_mut(&mut nonmatching_part_range) = nonmatching_range;
            Some(nonmatching_part_range)
        } else {
            None
        };

        (matching_part_range, nonmatching_part_range)
    }
}

#[test]
fn test_split_range() {
    let range = Range { min: 1, max: 100 };
    let part_range = PartRange {
        workflow_name: "foo".to_string(),
        x: range.clone(),
        m: range.clone(),
        a: range.clone(),
        s: range.clone(),
    };
    assert_eq!(
        "m<5"
            .parse::<RuleCondition>()
            .unwrap()
            .split_range(&part_range),
        (
            Some(PartRange {
                m: Range { min: 1, max: 4 },
                ..part_range.clone()
            }),
            Some(PartRange {
                m: Range { min: 5, max: 100 },
                ..part_range.clone()
            })
        )
    );

    assert_eq!(
        "m<1"
            .parse::<RuleCondition>()
            .unwrap()
            .split_range(&part_range),
        (
            None,
            Some(PartRange {
                m: Range { min: 1, max: 100 },
                ..part_range.clone()
            })
        )
    );
    assert_eq!(
        "m<0"
            .parse::<RuleCondition>()
            .unwrap()
            .split_range(&part_range),
        (
            None,
            Some(PartRange {
                m: Range { min: 1, max: 100 },
                ..part_range.clone()
            })
        )
    );
    assert_eq!(
        "m>0"
            .parse::<RuleCondition>()
            .unwrap()
            .split_range(&part_range),
        (
            Some(PartRange {
                m: Range { min: 1, max: 100 },
                ..part_range.clone()
            }),
            None,
        )
    );
    assert_eq!(
        "m>1"
            .parse::<RuleCondition>()
            .unwrap()
            .split_range(&part_range),
        (
            Some(PartRange {
                m: Range { min: 2, max: 100 },
                ..part_range.clone()
            }),
            Some(PartRange {
                m: Range { min: 1, max: 1 },
                ..part_range.clone()
            })
        )
    );
    assert_eq!(
        "m>99"
            .parse::<RuleCondition>()
            .unwrap()
            .split_range(&part_range),
        (
            Some(PartRange {
                m: Range { min: 100, max: 100 },
                ..part_range.clone()
            }),
            Some(PartRange {
                m: Range { min: 1, max: 99 },
                ..part_range.clone()
            })
        )
    );
    assert_eq!(
        "m>100"
            .parse::<RuleCondition>()
            .unwrap()
            .split_range(&part_range),
        (
            None,
            Some(PartRange {
                m: Range { min: 1, max: 100 },
                ..part_range.clone()
            })
        )
    );
}

#[derive(Debug)]
enum Action {
    Accept,
    Reject,
    NextWorkflow(String),
}

impl FromStr for Action {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "R" => Action::Reject,
            "A" => Action::Accept,
            s => Action::NextWorkflow(s.to_string()),
        })
    }
}

#[derive(Debug)]
struct Rule {
    condition: Option<RuleCondition>,
    action: Action,
}

impl FromStr for Rule {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(':');
        let first = split.next().unwrap();
        let (condition, action) = if let Some(action) = split.next() {
            (Some(first.parse().unwrap()), action.parse().unwrap())
        } else {
            (None, first.parse().unwrap())
        };

        Ok(Self { condition, action })
    }
}

#[derive(Debug)]
struct Part {
    x: i64,
    m: i64,
    a: i64,
    s: i64,
}

impl FromStr for Part {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"^\{x=(\d*),m=(\d*),a=(\d*),s=(\d*)\}$").unwrap();
        let Some((_, [x, m, a, s])) = re.captures(input).map(|c| c.extract()) else {
            return Err(());
        };
        Ok(Self {
            x: x.parse().map_err(|_| ())?,
            m: m.parse().map_err(|_| ())?,
            a: a.parse().map_err(|_| ())?,
            s: s.parse().map_err(|_| ())?,
        })
    }
}

impl Part {
    fn get_rating(&self) -> i64 {
        self.x + self.m + self.a + self.s
    }
}

#[derive(Debug)]
struct Workflows(HashMap<String, Vec<Rule>>);

impl FromStr for Workflows {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.lines()
                .map(|workflow_str| {
                    let mut split = workflow_str.split('{');
                    let workflow_name = split.next().expect("Workflow name");
                    let rules_str = split.next().expect("Rules");
                    assert!(split.next().is_none(), "Only 1 {{");

                    // Strip of trailing }
                    let rules_str = &rules_str[..rules_str.len() - 1];
                    let rules = rules_str.split(',').map(|r| r.parse().unwrap()).collect();

                    (workflow_name.to_string(), rules)
                })
                .collect(),
        ))
    }
}

impl Workflows {
    fn check_part(&self, part: &Part) -> bool {
        // println!("Checking {part:?}");
        let mut workflow_name = "in";
        loop {
            // println!("In workflow {workflow_name}");
            let workflow = self
                .0
                .get(workflow_name)
                .expect("No workflow with name {workflow_name}");

            match &workflow
                .iter()
                .find(|rule| {
                    rule.condition
                        .as_ref()
                        .map(|r| r.is_applicable(part))
                        .unwrap_or(true)
                })
                .expect("Part doesn't match any rule in workflow")
                .action
            {
                Action::Accept => return true,
                Action::Reject => return false,
                Action::NextWorkflow(next_name) => workflow_name = next_name,
            }
        }
    }

    fn solve_part2(&self) -> i64 {
        let mut part_ranges = vec![PartRange {
            workflow_name: "in".to_string(),
            x: Range { min: 1, max: 4000 },
            m: Range { min: 1, max: 4000 },
            a: Range { min: 1, max: 4000 },
            s: Range { min: 1, max: 4000 },
        }];

        let mut num_accepted = 0;
        while let Some(mut part_range) = part_ranges.pop() {
            let workflow = self
                .0
                .get(&part_range.workflow_name)
                .expect("No workflow with name {workflow_name}");
            for rule in workflow {
                if let Some(condition) = &rule.condition {
                    let (matching_part_range, nonmatching_part_range) =
                        condition.split_range(&part_range);

                    if let Some(mut matching_part_range) = matching_part_range {
                        match &rule.action {
                            Action::Accept => {
                                num_accepted += matching_part_range.num_distinct_parts()
                            }
                            Action::Reject => (),
                            Action::NextWorkflow(next_name) => {
                                matching_part_range.workflow_name = next_name.clone();
                                part_ranges.push(matching_part_range);
                            }
                        }
                    }

                    if let Some(nonmatching_part_range) = nonmatching_part_range {
                        part_range = nonmatching_part_range;
                    } else {
                        break;
                    }
                } else {
                    match &rule.action {
                        Action::Accept => num_accepted += part_range.num_distinct_parts(),
                        Action::Reject => (),
                        Action::NextWorkflow(next_name) => {
                            part_range.workflow_name = next_name.clone();
                            part_ranges.push(part_range);
                        }
                    }
                    break;
                }
            }
        }

        num_accepted
    }
}

#[derive(Debug)]
struct Puzzle {
    workflows: Workflows,
    parts: Vec<Part>,
}

impl FromStr for Puzzle {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split("\n\n");
        let workflows_str = split.next().expect("Workflows");
        let parts_str = split.next().expect("parts");
        assert!(split.next().is_none(), "Only rules and parts");

        let workflows = workflows_str.parse()?;

        let parts = parts_str
            .lines()
            .map(|part| part.parse().unwrap())
            .collect();

        Ok(Puzzle { workflows, parts })
    }
}

impl Puzzle {
    fn solve_part1(&self) -> i64 {
        self.parts
            .iter()
            .filter(|part| self.workflows.check_part(part))
            .map(Part::get_rating)
            .sum()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Range {
    min: i64,
    max: i64,
}

impl Range {
    fn len(&self) -> i64 {
        assert!(self.max >= self.min);
        self.max - self.min + 1
    }

    fn try_new(min: i64, max: i64) -> Option<Self> {
        if max >= min {
            Some(Self { min, max })
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct PartRange {
    workflow_name: String,
    x: Range,
    m: Range,
    a: Range,
    s: Range,
}

impl PartRange {
    fn num_distinct_parts(&self) -> i64 {
        self.x.len() * self.m.len() * self.a.len() * self.s.len()
    }
}

fn part1(input: &str) -> i64 {
    let puzzle: Puzzle = input.parse().unwrap();
    puzzle.solve_part1()
}

#[test]
fn test_part1() {
    assert_eq!(part1(TEST_INPUT), 19114);
}

fn part2(input: &str) -> i64 {
    let puzzle: Puzzle = input.parse().unwrap();
    puzzle.workflows.solve_part2()
}

#[test]
fn test_part2() {
    assert_eq!(part2(TEST_INPUT), 167409079868000);
}

fn main() {
    let input = &std::fs::read_to_string("input.txt").expect("input.txt should exist");
    println!("part 1: {}", part1(input));
    println!("part 2: {}", part2(input));
}

const TEST_INPUT: &str = r"px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";
