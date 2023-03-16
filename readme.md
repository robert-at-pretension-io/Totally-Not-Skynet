Project Overview

This project aims to create an intelligent system that can complete an overarching goal through the use of agents with different abilities. The system revolves around an event loop, where agents interact with each other and the environment to complete the goal. The project is built using a modular approach, with various agents representing different capabilities, and a central agent that selects the most appropriate action based on the current log.
Current Implementation

    An overarching goal that the system aims to achieve
    An event loop that drives the progress towards the goal
    A set of agents with distinct abilities (represented as prompts for the language model)
    A central agent that chooses the best action to take given the current log

Future Enhancements

    Staleness Meta-Agent: Develop a meta-agent that assesses the staleness of the current log. The agent will determine if the log has reached a state where no further progress can be made with the available agents and actions.

    Context Creation Meta-Agent: Implement a meta-agent that can create new contexts by generating new event logs with different combinations of agents. This will allow the system to explore alternative paths and strategies for achieving the goal.

    Agent Creation Meta-Agent: Design a meta-agent capable of creating new agents with abilities beyond those of the existing agents. This will enable the system to adapt and expand its capabilities as needed to overcome challenges and achieve the goal.

    Contextualization: Enhance the system by introducing the concept of contexts. A context will define the set of available actions and will be controlled by a configuration that groups actions into specific contexts. This will help manage the complexity of the system and ensure that only relevant actions are available in a given context.

Next Steps

To further develop the project, start by implementing the future enhancements listed above. Begin with the design and implementation of the Staleness Meta-Agent and then move on to the other meta-agents and contextualization features. Ensure thorough testing and documentation of each enhancement to maintain a robust and reliable system.