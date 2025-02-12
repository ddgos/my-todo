use std::str::FromStr;

use anyhow::{Error, Result};
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser, Debug, PartialEq)]
#[grammar = "./mtd.pest"]
struct Document {
    preamble: Option<Preamble>,
    iterations: Vec<Iteration>,
}

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
            assert_eq!(
                iteration.as_rule(),
                Rule::Iteration,
                "grammar definition mismatched with FromStr definition"
            );

            let mut tasks = Vec::new();
            for task in iteration.into_inner() {
                assert_eq!(
                    task.as_rule(),
                    Rule::Task,
                    "grammar definition mismatched with FromStr definition"
                );
                let mut task_inner = task.into_inner();

                let status_rule = task_inner.next().expect("should be two pairs").as_rule();
                let status = match status_rule {
                    Rule::Incomplete => TaskStatus::Incomplete,
                    Rule::Complete => TaskStatus::Complete,
                    Rule::Cancelled => TaskStatus::Cancelled,
                    _ => unreachable!("document parse should have errored"),
                };

                let description = task_inner
                    .next()
                    .expect("should be two pairs")
                    .as_str()
                    .to_string();

                tasks.push(Task {
                    status,
                    description,
                })
            }

            iterations.push(Iteration { tasks })
        }

        Ok(Document {
            preamble,
            iterations,
        })
    }
}

#[derive(Debug, PartialEq)]
struct Preamble {
    content: String,
}

#[derive(Debug, PartialEq)]
struct Iteration {
    tasks: Vec<Task>,
}

#[derive(Debug, PartialEq)]
struct Task {
    status: TaskStatus,
    description: String,
}

#[derive(Debug, PartialEq)]
enum TaskStatus {
    Incomplete,
    Complete,
    Cancelled,
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
                Task {
                    status: TaskStatus::Incomplete,
                    description: "unstarted".to_string(),
                },
                Task {
                    status: TaskStatus::Complete,
                    description: "complete".to_string(),
                },
                Task {
                    status: TaskStatus::Cancelled,
                    description: "cancelled".to_string(),
                },
            ],
        };
        let expected_second_iteration = Iteration {
            tasks: vec![Task {
                status: TaskStatus::Incomplete,
                description: "next iteration".to_string(),
            }],
        };
        let expected_iterations = vec![expected_first_iteration, expected_second_iteration];
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
                Task {
                    status: TaskStatus::Incomplete,
                    description: "unstarted".to_string(),
                },
                Task {
                    status: TaskStatus::Complete,
                    description: "complete".to_string(),
                },
                Task {
                    status: TaskStatus::Cancelled,
                    description: "cancelled".to_string(),
                },
            ],
        };
        let expected_second_iteration = Iteration { tasks: Vec::new() };
        let expected_iterations = vec![expected_first_iteration, expected_second_iteration];
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
}
