struct Document {
    preamble: Option<Preamble>,
    iterations: Vec<Iteration>,
}

struct Preamble {
    content: String,
}

struct Iteration {
    tasks: Vec<Task>,
}

struct Task {
    status: TaskStatus,
    description: String,
}

enum TaskStatus {
    Incomplete,
    Complete,
    Cancelled,
}

#[cfg(test)]
mod tests {}
