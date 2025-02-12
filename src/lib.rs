use std::fmt::{Display, Write};
use std::str::FromStr;

use anyhow::{Error, Result};
use pest::Parser;
use pest_derive::Parser;

#[derive(Clone, Parser, Debug, PartialEq)]
#[grammar = "./mtd.pest"]
pub struct Document {
    pub preamble: Option<Preamble>,
    pub iterations: Vec<(usize, Iteration)>,
}

// impl Document {
//     pub fn new(preamble: Option<Preamble>, iterations: Vec<(Iteration>) -> Self {
//         Self {
//             preamble,
//             iterations,
//         }
//     }
// }

impl FromStr for Document {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut pairs = Document::parse(Rule::Document, s)?;

        let first_pair = pairs
            .next()
            .expect("should be first pair if document parsed");
        let (iterations_pair, preamble) = match first_pair.as_rule() {
            // There is a preamble
            Rule::Preamble => {
                let content = first_pair.as_str().to_string();
                // use the next pair to extract iterations
                // and return the found preamble
                (pairs.next().unwrap(), Some(Preamble { content }))
            }
            // There was no preamble
            Rule::Iterations => {
                // use this pair to extract iterations, and there is no preamble
                (first_pair, None)
            }
            // the first pair should only be a preamble or iterations by
            // definition in the grammar.
            // If this has gone wrong, check ./src/mtd.pest
            _ => unreachable!("document parse should have errored"),
        };

        let mut iterations = Vec::new();
        for iteration in iterations_pair.into_inner() {
            // If this has gone wrong, check ./src/mtd.pest
            debug_assert_eq!(
                iteration.as_rule(),
                Rule::Iteration,
                "grammar definition mismatched with FromStr definition"
            );

            let mut iteration_inner = iteration.into_inner();
            // the first pair within an iteration is the header
            let iteration_num: usize = iteration_inner
                .next()
                .expect("should have a header")
                .as_str()
                .parse()
                // if this goes wrong, check the pair rule is a number
                .expect("should parse to usize due to grammar rules");

            let mut tasks = Vec::new();
            // the rest of the pairs in an iteration are tasks
            for task in iteration_inner {
                debug_assert_eq!(
                    task.as_rule(),
                    Rule::Task,
                    "grammar definition mismatched with FromStr definition"
                );
                let mut task_inner = task.into_inner();

                let task_num: usize = task_inner
                    .next()
                    .expect("should be three pairs")
                    .as_str()
                    .parse()
                    .expect("Number in grammar should parse to usize");
                let status_rule = task_inner.next().expect("should be three pairs").as_rule();
                let status = match status_rule {
                    Rule::Incomplete => TaskStatus::Incomplete,
                    Rule::Complete => TaskStatus::Complete,
                    Rule::Cancelled => TaskStatus::Cancelled,
                    _ => unreachable!("document parse should have errored"),
                };

                let description = task_inner
                    .next()
                    .expect("should be three pairs")
                    .as_str()
                    .to_string();

                tasks.push((
                    task_num,
                    Task {
                        status,
                        description,
                    },
                ))
            }

            iterations.push((iteration_num, Iteration { tasks }))
        }

        Ok(Document {
            preamble,
            iterations,
        })
    }
}

impl Display for Document {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(Preamble { content }) = &self.preamble {
            write!(f, "{}\n\n", content)?;
        };

        // ignore original iteration numbering
        for (iter_num, (_, Iteration { tasks })) in self.iterations.iter().enumerate() {
            if iter_num > 0 {
                write!(f, "\n\n")?;
            }
            write!(f, "# {}", iter_num)?;
            if !tasks.is_empty() {
                f.write_char('\n')?;
            }
            for (
                task_num,
                (
                    _, // ignore what it was originally numbered as
                    Task {
                        status,
                        description,
                    },
                ),
            ) in tasks.iter().enumerate().map(|(num, task)| (num + 1, task))
            {
                write!(f, "\n{}. [{}] {}", task_num, status, description)?
            }
        }
        write!(f, "\n")?;
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Preamble {
    pub content: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Iteration {
    pub tasks: Vec<(usize, Task)>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Task {
    pub status: TaskStatus,
    pub description: String,
}

#[derive(Clone, Debug, PartialEq)]
pub enum TaskStatus {
    Incomplete,
    Complete,
    Cancelled,
}

impl Display for TaskStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            TaskStatus::Incomplete => ' ',
            TaskStatus::Complete => 'x',
            TaskStatus::Cancelled => 'c',
        };
        f.write_char(c)
    }
}

#[cfg(test)]
mod tests {
    use crate::{Document, Iteration, Preamble, Task, TaskStatus};

    #[test]
    fn doc_1_parses_correctly() {
        let doc_str = include_str!(r#"../test_docs/doc_1.mtd"#);
        println!("doc_1 is:\n{}", doc_str);

        let expected_preamble = Preamble {
            content: "# Preamble\n\nI am the preamble!".to_string(),
        };
        let expected_first_iteration = Iteration {
            tasks: vec![
                (
                    1,
                    Task {
                        status: TaskStatus::Incomplete,
                        description: "unstarted".to_string(),
                    },
                ),
                (
                    2,
                    Task {
                        status: TaskStatus::Complete,
                        description: "complete".to_string(),
                    },
                ),
                (
                    3,
                    Task {
                        status: TaskStatus::Cancelled,
                        description: "cancelled".to_string(),
                    },
                ),
            ],
        };
        let expected_second_iteration = Iteration {
            tasks: vec![(
                1,
                Task {
                    status: TaskStatus::Incomplete,
                    description: "next iteration".to_string(),
                },
            )],
        };
        let expected_iterations = vec![
            (0, expected_first_iteration),
            (1, expected_second_iteration),
        ];
        let expected_document = Document {
            preamble: Some(expected_preamble),
            iterations: expected_iterations,
        };

        let parsed_document: Document = doc_str.parse().unwrap();

        if expected_document != parsed_document {
            assert_eq!(expected_document.preamble, parsed_document.preamble);
        }
        assert_eq!(expected_document, parsed_document);
    }

    #[test]
    fn doc_2_parses_correctly() {
        let doc_str = include_str!(r#"../test_docs/doc_2.mtd"#);
        println!("doc_2 is:\n{}", doc_str);

        let expected_first_iteration = Iteration {
            tasks: vec![
                (
                    1,
                    Task {
                        status: TaskStatus::Incomplete,
                        description: "unstarted".to_string(),
                    },
                ),
                (
                    2,
                    Task {
                        status: TaskStatus::Complete,
                        description: "complete".to_string(),
                    },
                ),
                (
                    3,
                    Task {
                        status: TaskStatus::Cancelled,
                        description: "cancelled".to_string(),
                    },
                ),
            ],
        };
        let expected_second_iteration = Iteration { tasks: Vec::new() };
        let expected_iterations = vec![
            (0, expected_first_iteration),
            (1, expected_second_iteration),
        ];
        let expected_document = Document {
            preamble: None,
            iterations: expected_iterations,
        };

        let parsed_document: Document = doc_str.parse().unwrap();

        if expected_document != parsed_document {
            assert_eq!(expected_document.preamble, parsed_document.preamble);
        }
        assert_eq!(expected_document, parsed_document);
    }

    #[test]
    fn doc_1_formats_correctly() {
        let doc_str = include_str!(r#"../test_docs/doc_1.mtd"#);
        println!("doc_1 is:\n{}", doc_str);

        let preamble = Preamble {
            content: "# Preamble\n\nI am the preamble!".to_string(),
        };
        let first_iteration = Iteration {
            tasks: vec![
                (
                    1,
                    Task {
                        status: TaskStatus::Incomplete,
                        description: "unstarted".to_string(),
                    },
                ),
                (
                    2,
                    Task {
                        status: TaskStatus::Complete,
                        description: "complete".to_string(),
                    },
                ),
                (
                    3,
                    Task {
                        status: TaskStatus::Cancelled,
                        description: "cancelled".to_string(),
                    },
                ),
            ],
        };
        let second_iteration = Iteration {
            tasks: vec![(
                1,
                Task {
                    status: TaskStatus::Incomplete,
                    description: "next iteration".to_string(),
                },
            )],
        };
        let iterations = vec![(0, first_iteration), (1, second_iteration)];
        let document = Document {
            preamble: Some(preamble),
            iterations,
        };

        assert_eq!(doc_str, format!("{}", document))
    }

    #[test]
    fn doc_2_formats_correctly() {
        let doc_str = include_str!(r#"../test_docs/doc_2.mtd"#);
        println!("doc_2 is:\n{}", doc_str);

        let first_iteration = Iteration {
            tasks: vec![
                (
                    1,
                    Task {
                        status: TaskStatus::Incomplete,
                        description: "unstarted".to_string(),
                    },
                ),
                (
                    1,
                    Task {
                        status: TaskStatus::Complete,
                        description: "complete".to_string(),
                    },
                ),
                (
                    1,
                    Task {
                        status: TaskStatus::Cancelled,
                        description: "cancelled".to_string(),
                    },
                ),
            ],
        };
        let second_iteration = Iteration { tasks: Vec::new() };
        let iterations = vec![(0, first_iteration), (1, second_iteration)];
        let document = Document {
            preamble: None,
            iterations,
        };

        assert_eq!(doc_str, format!("{}", document))
    }
}
