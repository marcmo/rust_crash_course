# Examples using encode/decode it within the workspace/project
cargo r --bin decode resources/messages.bin
cargo r --bin encode resources/messages.jsonl
cargo r --bin encode resources/messages.jsonl | cargo r --bin decode

# Example encode a message format stream received from a network connection




