// Define a data model for the AI-powered data pipeline analyzer

use serde::{Serialize, Deserialize};

// Represents a data pipeline
#[derive(Serialize, Deserialize, Debug)]
struct Pipeline {
    id: String,
    name: String,
    description: String,
    nodes: Vec<Node>,
}

// Represents a node in the data pipeline
#[derive(Serialize, Deserialize, Debug)]
struct Node {
    id: String,
    node_type: NodeType,
    config: NodeConfig,
    inputs: Vec<NodeInput>,
    outputs: Vec<NodeOutput>,
}

// Represents the type of a node (e.g. source, processor, sink)
#[derive(Serialize, Deserialize, Debug)]
enum NodeType {
    Source,
    Processor,
    Sink,
}

// Represents the configuration of a node
#[derive(Serialize, Deserialize, Debug)]
struct NodeConfig {
    // Node-specific configuration
    props: Vec<(String, String)>,
}

// Represents an input to a node
#[derive(Serialize, Deserialize, Debug)]
struct NodeInput {
    id: String,
    data_type: String,
    // Reference to the output of another node
    upstream_node_id: Option<String>,
    upstream_output_id: Option<String>,
}

// Represents an output of a node
#[derive(Serialize, Deserialize, Debug)]
struct NodeOutput {
    id: String,
    data_type: String,
    // Reference to the input of another node
    downstream_node_id: Option<String>,
    downstream_input_id: Option<String>,
}

// Represents the analysis result of a pipeline
#[derive(Serialize, Deserialize, Debug)]
struct AnalysisResult {
    pipeline_id: String,
    warnings: Vec<Warning>,
    errors: Vec<Error>,
    // Additional analysis metrics (e.g. performance, data quality)
    metrics: Vec<(String, f64)>,
}

// Represents a warning or error found in the pipeline analysis
#[derive(Serialize, Deserialize, Debug)]
struct Warning {
    message: String,
    node_id: Option<String>,
    output_id: Option<String>,
}

// Represents an error found in the pipeline analysis
#[derive(Serialize, Deserialize, Debug)]
struct Error {
    message: String,
    node_id: Option<String>,
    output_id: Option<String>,
}