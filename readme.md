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

1. **Implement a memory construct:** This is fundamental for storing and retrieving information that's used throughout your system.

   - Allow metaprocess to add to memory context, influencing all prompts
   - Introduce a runtime state

2. **Decide on the appropriate data structure and algorithm for storing the execution of the node-graph:** This step involves the underlying architecture of how your system will organize and process data, which is critical before other functionalities can be built.

3. **Implement digital system features such as search, parse, store, execute, retrieve, order, sort, filter, etc.:** These features are core to your system's operation and should be addressed early on.

4. **Introduce various node types including flow control, store data, subprocesses, memory, context, conditionals:** Different node types will enable you to build more complex and flexible functionality.

5. **Create a system to describe the components of the software project so the system can recognize its components:** This will enable your system to understand its own structure, which will help with later development stages.

6. **Develop a web browsing process:** This can be done once the fundamental structures and features of the system are implemented.

7. **Allow the style of nodes to be altered based on node type:** This is important for visual distinction and user interaction, but can be done after the core functionality is established.

8. **Enhance self-reflection capabilities to avoid issues like infinite loops:** This is a complex task that requires a functioning system to test and improve upon.

   - Implement a metaprocess that maintains a list of summarized actions that have occurred in relation to the goal

9. **Save execution context along with the ID of actions (with semantic versioning) to facilitate version control:** This will help with tracking changes and maintaining the stability of your system.

10. **Implement separate Docker containers for each connection:** This involves containerization and can be done once the system's core features are stable.

11. **Implement a global variable store to eliminate the need for variable nodes:** This optimization can improve efficiency but isn't essential in the early stages of the system.

12. **Introduce semantic versioning to actions so that processes can maintain stable dependencies:** Semantic versioning is important for maintaining backward compatibility and should be done when the system has a set of stable features.

13. **Develop a method to suspend and schedule processes:** This feature is important for controlling execution flow and can be developed after the main features are implemented.

14. **Integrate the runtime/execution of processes with an external, real-time (human-scale) workflow engine:** This can be a later step once the system is functioning correctly and the integration will provide additional functionality.

# Example Usages

Currently, we're in the process of implementing these examples. They will be updated soon.

We appreciate your patience and encourage you to contribute to this project in the meantime.

# Contributions

We welcome contributions! If you're interested in contributing, please see our [Contributing Guidelines](CONTRIBUTING.md).

# License

This project is licensed under the [MIT License](LICENSE).
