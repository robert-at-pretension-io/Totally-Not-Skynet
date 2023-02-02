# Project goals:
* Make a system that uses a language model, specified user roles and project goals provided as input to create/generate functioning software systems.

# Roles definition
Each of the roles has a defined "perogative" and "obligation" that they must follow. The perogative is the role's ability to do something, and the obligation is the role's responsibility to do something. The perogative and obligation are defined in the following way for each of the roles:

# Roles
* **Developer** - The developer is responsible for writing the code for the system. The developer's perogative is to write code, and the developer's obligation is to write code that is correct and follows the (tickets) as closely as possible. They are provided (tickets) from the [Lead Developer](#lead-developer). Once the developer is done with the task, they will submit a (pull request) to the [QA](#qa) for review. 

* **Lead Developer** - The lead developer is the person who is responsible for managing the development of the system. The lead developer's perogative is to manage the development of the system, and the lead developer's obligation is to manage the development of the system in a way that is correct and follows the (project goals). They create (tickets) for the [Developer](#developer) to work on. The ticket will contain a description of the task, the expected outcome. It will also contain useful api documentation, design documents, and other information that the developer will need to complete the task. It can also contain a list of API that still need to be implemented. It will specify the language that the [developer](#developer) should use to implement the task. 

* **QA** - The QA is the person who is responsible for testing the new code sent as a (pull request) from the developer. The QA's perogative is to test (pull request) and make sure it works in the system, and the QA's obligation is to provide feedback to the developer if they don't believe the (pull request) fulfills the requirements of the (ticket). They will review the pull requests from the [Developer](#developer) and test the code. They will then either approve the (pull request) or (request changes). If the QA requests changes, the [Developer](#developer) will make the changes and submit another pull request. The QA will then review the changes and either approve the pull request or request more changes. This process will continue until the QA approves the (pull request). Once the QA approves the pull request, the [Lead Developer](#lead-developer) will (merge) the (pull request) into the (master branch).

* **System Architect** - The system architect is responsible for designing the system. The system architect's perogative is to design the system, and the system architect's obligation is to design the system in a way that is correct and follows the (project goals). They will create the design documents for the system. They will also create the API documentation for the system. They only communicate with the [Lead Developer](#lead-developer) and the [QA](#qa). They will not communicate with the [Developer](#developer). The system archtect has the final say on the design of the system. The system architect has the ability to (inspect) the system. It can (read files) within the system.

# System Objects
* **Ticket** - A ticket is a document that contains a description of the task, the expected outcome, as well as useful API documentation, design documents, and other information that the developer will need to complete the task. It can also contain a list of API that still need to be implemented. It will specify the language that the developer should use to implement the task.

* **Design Documents** -  Design documents are documents that contain the system design and other details related to the system. The Lead Developer and System Architect will use these documents to communicate with each other.

* **API Documentation** - API Documentation is information that the Developer will use to implement the task. The System Architect will create the API documentation for the system.

* **Pull Request** - A pull request is a request from the developer to the QA to review their code. The pull request contains the code that the developer has written, and the QA will review the code and either approve the pull request or request changes.

* **Test Suite** - The test suite is a collection of tests that the QA will use to test the code. The QA will run the test suite on the code to make sure that it works in the system. The test suite will be created by the QA. Each time the QA is given a ticket, they will add tests to the test suite that will test the code that the developer will write. Failing the test suite is a sign that the code does not work in the system and thus the QA will request changes from the developer. The Developer does not have access to the test suite, they will only be able to see the results of the test suite as conveyed by the QA in a Change Request.
# System Actions
* **Merge** - A merge is when the Lead Developer takes the code from the pull request and adds it to the master branch.

* **Inspect** - Inspect is when the System Architect looks at the code and design documents to make sure that it meets the project goals. 

* **Read Files** - Read Files is when the System Architect reads the code and design documents to make sure that it meets the project goals.

*  **Pull Request** - A pull request is a request from the developer to the QA to review their code. The pull request contains the code that the developer has written, and the QA will review the code and either approve the pull request or request changes.

* **Request Clarification** -  Request clarification is when either the Developer or the QA requests clarification from the Lead Developer. The Lead Developer will provide clarification to the Developer or the QA through an ammened ticket.

* **Approval/Request Changes** - When a Pull Request is made, the QA will either approve the pull request or request changes. If the QA approves the pull request, the Lead Developer will review the Pull Request and either request changes or approve it. The Lead Developer is making sure the code conforms to both the ticket and the requirements of the project.

# Conclusion
The system described above is a system that uses a language model, specified user roles, and project goals provided as input to create/generate functioning software systems. The system is composed of roles, objects, and actions that are used to create, develop, and maintain the system. The roles have a defined "perogative" and "obligation" that they must follow. The roles, objects, and actions are linked together in order to create a functioning software system.

# Communication Channels
For this system to work, there must be clear lines of communication between the roles. The communication channels are as follows:


* **Developer <-> Lead Developer** - The developer will communicate with the lead developer through the (tickets) that are assigned to them. The lead developer will provide feedback to the developer through the (tickets). Developers can request clarification on a ticket if they are unsure of the requirements, they should be certain that they have all of the tools and specifications needed to design the program.

* **Lead Developer <-> System Architect** - The lead developer and system architect will communicate through design documents and API documentation.

* **QA <-> Developer** - The QA and developer will communicate through (pull requests) and request changes. The QA will provide feedback to the developer through the pull requests.


# System Stages
The system must have a starting point. A genesis. This will be formulated as a file containing project goals named "project_goals.txt". This will contain a plain english description of what the successful completion of the project will look like.

1. The *system architect* will create **API Documentation**. It will detail a) the programming language that will be used for the project, the b) system objects and c) system functions that will be needed to create the functioning software system that fulfills the requirements described in the. The functions must be in terms of the system objects and basic types. Finally, the system architect must describe the system at a high level in terms of the functions and objects. For each of the functions, the *architect* will create a **Ticket**. These tickets will include criteria for successful completion of the ticket and give references to all of objects required to implement the function. Of course, the tickets will also specify the programming language that the code written for the ticket must be in. In order to allow the developer to focus on implementation instead of extraneous details, the ticket will also specify the relative file structure of the function in relation to the rest of the code. The reason for this specificity is that both the *QA* and the *Developer* are reading the same ticket. The QA is writing unit tests for the functions while the developer

2. Once the all of the tickets are created, the tickets will be handled one by 