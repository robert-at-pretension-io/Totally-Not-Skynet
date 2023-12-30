# Project Overview
The purpose of this project is to allow someone with no programming experience to compose their job's (either professional or personal) processes and workflows and let the ai figure out how to execute them.

Processes are built up using a visual interface that is shown to the user in a browser. The interface is designed to be intuitive (and always open to constructive criticism).

This project is designed to encapsulate even complex workflows by having the ability to "nest" processes. That is, a sub-process can be defined as detailed and nuanced as required for the task. Then that sub-process can be called from a higher level process. Thus, **YOUR** standards are met -- whether it's for a personal project or a large enterprise.

# Recommendation
* Do not read the text below until you watch the following youtube video demoing the system: (put the video here)

# How this works:
1. Within the browser, the user defines basic tasks in "plain english" that contain "variables". (input/output variables are just a way of saying that these will be replaced with other values that are defined when the process is executed)
   - **For example**, a task coule be defined as: `"Go to the website {{website}} and click on the {{button}} button."` In this example, `{{website}}` and `{{button}}` are input variables that are given specific values by the user when the task is _executed_ later on. For instance, website = google, button = search.

   - When a process is defined, the user will explain what the output variables should be. These should things that the ai should be able to understand by _executing_ the task. For instance, in the example above, the output variable might be `"{{html}}"` which would be the html of the page that the user is taken to after clicking the button. The `{{html}}` variable can then be used as an input variable for another task when the _process_ is executed.

# Current Implementation
- Currently, nodes can be created and processes can be executed.

## What is a process?
- Processes are exactly what they sound like, they are a series of steps (nodes) that lead to an outcome. This sounds vague and general and that's because it is. Processes can accomplish almost anything that can be done by a human on a computer -- using plain english alone.

## What are nodes?
- Node are the "steps" of a process. That is, they are the what make up an execution.

### Node Types 
- **Prompt**: this is like chatting with a language model. The only difference is that you define "variables" that will change depending on initial state of the execution.
- **Conditional**: This node creates decision points within your process. When these nodes are run, they will only end up coming up with definitions for some of the output variables. These will determine how the rest of the process is executed.
- **Loop**: This is *like* a process except that it MUST contain a conditional node. A loop will run all of the nodes within the loop, ending with the conditional node that decides if the loop should be exited.
- **Command**: This node controls a computer and will attempt to complete a goal provided to it. This is a powerful node because the composition of command nodes can perform arbitrary computer tasks (sending emails, setting up minecraft servers, hosting irc servers and more creative things!)
- **Process**: As mentioned previously, processes are both made up of nodes AND nodes themselves. What does this mean? Well, let's say that you define a process that dependably sends emails from your address to a recipient email. This could be used as a step within another process that looks on the internet for the tech support for a certain company (the company could be used as a variable). 

# Project installation/setup guide (local edition)
- system requirements: **no idea**. But, for reference, I developed this on an asus e210m with ubuntu installed ($210 laptop with a celeron cpu)
- Download/install node, npm, cargo, podman ( or docker if you're into that 😒... I guess 💀 )
- Setup podman or docker ([Follow these instructions and modify them as needed](https://chat.openai.com/share/419b50ef-ee4a-4f57-a13e-96927f1fd24e) ) -- this isn't  necessary if you don't need to want to use the command node.
- Clone this git repository
- Set the environmental variables in the backend/req_env_vars.txt text file. Please either ask chatgpt or google how to do this if you don't know how
   - In particular, make sure environment is set to "DEVELOPMENT" 
- Add your email address to the backend/allowed_emails.txt file (this will allow you to make a login for the application)
   - **NOTE**: The first time you login, whatever password you put in the box will be your password... So like don't mess that up, you got this!! If you DO mess that up, find someone who knows sqlite and sql and ask them kindly to remove your username from backend/auth.db (or just delete backend/auth.db if you're the only user, like in the case of local builds).
- open two terminals. 

- In the first terminal (the order IS important as the cargo build steps make sure the project has requirements and env variables defined), go to the backend folder and type in:
   - `cargo run`
   - **NOTE**: do not be afraid of the terminal. If something goes wrong, read what it says. For most problems, I have put instructive error messages. If it's not instructive, still write it down somewhere so someone can help you figure out how to solve your problem.

- In the second termal, open the frontend folder and type in:
   - `npm install`
   ( wait until its done installing )
   - `npm run dev`
   ( DON'T CLOSE THIS TERMINAL )

- The browser should automatically open when `npm run dev` finishes. If not, type in `127.0.0.1:5000` in your browser url.

- If anything goes wrong, please send an email to robert+help@pretension.io and I'll get back to you. 

# License:
GNU General Public License (read more here: https://www.gnu.org/licenses/gpl-3.0.html )... Go wild 🍾🥂. 


# Roadmap:
- Improve loop nodes
- Improve conditional nodes
- Add process scheduling / resuming
- Add persistent memory feature for processes to write important global context as needed
- Add teams feature (multiple people have access to the same processes/nodes)
- Add versioning feature to protobuf schemas for easier data-structure evolution


# Continual Initiatives:
- Improve code readability
- Add video resources explaining design choices of architecture
- Improve on-boarding for new project members (haha, more people will join this project... right?)
- Improve visual appearance of app (this requires designers with a lot more experience than me!)