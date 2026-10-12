use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use subako_function_sdk::{ToolError, function, package};

#[derive(Deserialize, JsonSchema)]
struct GreetInput {
    /// Who to greet.
    name: String,
}

#[derive(Serialize, JsonSchema)]
struct Greeting {
    message: String,
}

/// Greet someone by name.
#[function]
fn greet(input: GreetInput) -> Result<Greeting, ToolError> {
    Ok(Greeting {
        message: format!("Hello, {}!", input.name),
    })
}

package! {
    name: "{{project-name}}",
    description: "Says hello.",
    functions: [greet],
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greets_by_name() {
        let greeting = greet(GreetInput {
            name: "Ada".to_string(),
        })
        .unwrap();
        assert_eq!(greeting.message, "Hello, Ada!");
    }
}
