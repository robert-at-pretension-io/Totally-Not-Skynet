# Project goals:
* Make a system that uses a language model, specified user roles and project goals provided as input to create/generate functioning software systems.

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

1. The *system architect* will create **API Documentation**. It will detail a) the programming language that will be used for the project, the b) system objects and c) system functions that will be needed to create the functioning software system that fulfills the requirements described in the project_goals.txt file. The functions must be in terms of the system objects and basic types of the chosen programming language. If external libraries are going to be used in the function, they must be specified in the ticket. Finally, the system architect must describe the system at a high level in terms of the functions and objects. 

2. For each of the functions, the *architect* will create a **Ticket**. These tickets will include a) criteria for successful completion of the ticket, b) concise and unambiguous descriptions of all of objects required to implement the function, c) the programming language that the code written, d) the relative file structure of the function in relation to the rest of the code, e) what the function should be named. The reason for this specificity is that both the *QA* and the *Developer* are reading the same ticket while having different objectives. The tickets will have the following file names: ticket_number_[ticket_number]_[function_name|object_name].txt . The ticket_number will be the order that they are implemented.

3. Once the all of the tickets are created, the tickets will be handled one by one. "Handling" a ticket goes like this: The QA will read the ticket and create unit tests that import the function as it is described in the ticket. Independently, the developer will write code that implements the function described in the ticket. The developer will notify the qa when the function is completed. The QA will then run the tests on the code and report their results to the developer. If the code passes the tests, the developer will submit a pull request to the QA.

4. The *QA* will review the pull request sent by the *developer*. If the code passes the tests, the QA will approve the pull request and the *lead developer* will merge it into the master branch. If the code fails the tests, the QA will request changes from the developer and the process will start again.

5. Once all of the functions have been implemented, the *system architect* will inspect the system. They will make sure that the code follows the project goals and the code is well-structured and commented. If there are any issues, the system architect will re-open the tickets and the process will start again.

# System implementation:
* In order to program this meta-system, the following functions must exist:
  * **parse_ticket()** - This function will parse the ticket into a structured data object that contains the criteria for successful completion of the ticket, a concise and unambiguous description of the objects required to implement the function, a programming language for the code to be written in and the relative file structure of the function in relation to the rest of the code.

* **write_code()** - This function will take the structured data object from the parse_ticket() function and use it to write a program that implements the function described in the ticket.

* **run_tests()** - This function will take the code from the write_code() function and run it against a test suite created by the QA. It will then return the results of the tests.

* **submit_pull_request()** - This function will take the code from the write_code() function and submit it to the QA in the form of a pull request.

* **review_pull_request()** - This function will take the code from the pull request and review it. If the code passes the tests, the QA will approve the pull request and the Lead Developer will merge it into the master branch. If the

* **create_ticket()**  - This function will take the project goals from the project_goals.txt file and use them to create a ticket for the developer. It will contain a description of the task, the expected outcome, as well as useful API documentation, design documents, and other information that the developer will need to complete the task. It can also contain a list of API that still need to be implemented. It will specify the language that the developer should use to implement the task.

* **inspect_system()** - This function will take the system and inspect it. It will make sure that the code follows the project goals and the code is well-structured and commented. If there are any issues, the system architect will re-open the tickets and the process will start again.

# Event loop:
* Write the event loop using the functions above, write it in python:

def event_loop():
    while True:
        ticket = create_ticket() # create a ticket
        code = write_code(ticket) # write the code
        tests = run_tests(code) # run the tests
        pull_request = submit_pull_request(code) # submit the pull request
        review_result = review_pull_request(pull_request) # review the pull request
        if review_result == "approved": # if the pull request is approved
            merge_result = merge_pull_request(pull_request) # merge the pull request
            inspect_result = inspect_system() # inspect the system
            if inspect_result == "passed": # if the system passes inspection
                break # break out of the loop
            else: # if the system does not pass inspection
                continue # continue the loop
        else: # if the pull request is not approved
            continue # continue the loop

# Final Thoughts
This system is a meta-system that uses language models, specified user roles, and project goals provided as input to create/generate functioning software systems. By using this system, developers, lead developers, system architects, and QA can create software systems quickly and efficiently. It will reduce the amount of time needed for the creation of software systems and make the process more streamlined.