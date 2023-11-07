import os
import openai
import json  # Safer alternative to eval for loading and storing data

# Load the OpenAI API key from an environment variable
openai.api_key = os.getenv('OPENAI_API_KEY')
if not openai.api_key:
    raise ValueError("The OpenAI API key has not been set in environment variables.")

# Function to append a user message, talk to the OpenAI API, and get back a response
def talk_to_openai(user_message, conversation_history):
    conversation_history.append({"role": "user", "content": user_message})
    response = openai.chat.completions.create(
        model="gpt-4-1106-preview",
        messages=conversation_history
    )
    
    completion = response.choices[0].message.content
    conversation_history.append({"role": "assistant", "content": completion})
    
    return completion, conversation_history

# Paths to the conversation log
conversation_log_path = "conversation_log.txt"

# Load conversation history log
if os.path.exists(conversation_log_path):
    with open(conversation_log_path, 'r') as f:
        conversation_history = json.load(f)
else:
    conversation_history = []

# Main loop for the conversation
while True:
    # Ask the user for their message
    user_message = input("You: ")

    if user_message.lower() in ['exit', 'quit']:
        break

    # Get the response from OpenAI and update conversation history
    ai_response, conversation_history = talk_to_openai(user_message, conversation_history)
    
    # Print the conversation history (only print last user and assistant messages)
    print(f"You: {user_message}")
    print(f"Assistant: {ai_response}")
    
    # Save updated conversation history
    with open(conversation_log_path, 'w') as f:
        json.dump(conversation_history, f)

# Exit message
print("Exiting conversation.")