# Project Overview

This project is designed to create an intelligent system capable of accomplishing a comprehensive goal via the utilization of multiple agents possessing varied abilities. The system operates around an event loop, where agents interact with the environment and each other in order to reach the defined goal. Built with a modular approach, our project incorporates multiple agents to represent different functionalities, and a central agent that selects the most suitable action based on the current log.

![Architecture](https://i.imgur.com/UHsnRJ8.png)

# Current Implementation

The current implementation of the system includes:

- An overarching goal which the system endeavors to reach
- An event loop propelling progress towards the said goal
- A set of distinct agents with abilities, symbolized as prompts for the language model
- A central agent responsible for choosing the most optimal action in accordance with the current log

# To-Do List:

The features are listed in order of their importance and usefulness:

- [ ] Implement a memory construct:

  - [ ] Allow metaprocess to add to memory context, influencing all prompts
  - [ ] Introduce a runtime state

- [ ] Enhance self-reflection capabilities to avoid issues like infinite loops

  - [ ] Implement a metaprocess that maintains a list of summarized actions that have occurred in relation to the goal

- [ ] Implement separate Docker containers for each connection

- [ ] Develop a web browsing process

- [ ] Allow the style of nodes to be altered based on node type

- [ ] Decide on the appropriate data structure and algorithm for storing the execution of the node-graph

- [ ] Integrate the runtime/execution of processes with an external, real-time (human-scale) workflow engine

- [ ] Implement digital system features such as search, parse, store, execute, retrieve, order, sort, filter, etc.

- [ ] Introduce various node types including flow control, store data, subprocesses, memory, context, conditionals

- [ ] Develop a method to suspend and schedule processes

- [ ] Create a system to describe the components of the software project so the system can recognize its components

- [ ] Save execution context along with the ID of actions (with semantic versioning) to facilitate version control

- [ ] Implement a global variable store to eliminate the need for variable nodes

- [ ] Introduce semantic versioning to actions so that processes can maintain stable dependencies

# Example Usages

Currently, we're in the process of implementing these examples. They will be updated soon.

We appreciate your patience and encourage you to contribute to this project in the meantime.

# Contributions

We welcome contributions! If you're interested in contributing, please see our [Contributing Guidelines](CONTRIBUTING.md).

# License

This project is licensed under the [MIT License](LICENSE).
