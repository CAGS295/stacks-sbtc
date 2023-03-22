# minedoc (miner doctor) - CLI Tool for Debugging Running Nodes

This CLI tool is designed to help developers debug running nodes by sourcing node information from various sources such as the node's RPC API, logs, and database.

### (Potential) Features

- Easy-to-use CLI interface: The CLI interface is user-friendly and easy to use, allowing developers to quickly get the information they need to debug their nodes.
- Simple HTML interface: In addition to the CLI, this tool provides a simple HTML interface that allows developers to easily view node information in their web browser.


### Usage

To use this tool, simply run the command with the appropriate options and arguments. For example:

```
minedoc \
  --node-url=http://localhost:8545 \
  --log-file=/path/to/node.log \
  --db-file=/path/to/db.sqlite \
  analyze
```

If you want to simplify the command it's possible to move arguments to environment variables.

```
export MINEDOC_NODE_URL=http://localhost:8545;
export MINEDOC_LOG_FILE=/path/to/node.log;
export MINEDOC_DB_FILE=/path/to/db.sqlite;

minedoc qanalyze
```

This command will get data from all sources provided and output information about any potential problems.

### Installation

To install this tool, simply run the following command:

```
cargo install --path core-eng/minedoc
```
