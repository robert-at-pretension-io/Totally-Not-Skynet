from pymongo import MongoClient

def update_mongodb_collection():
    # Create a MongoClient to the running MongoDB instance
    # Replace 'mongodb://localhost:27017/' with your MongoDB connection URI
    client = MongoClient('mongodb+srv://[username]:[password]@cluster0.tkon6.mongodb.net/')

    # Access the 'your_database' database. Replace 'your_database' with your database name
    db = client['skynet']

    # Access the 'your_collection' collection. Replace 'your_collection' with your collection name
    collection = db['nodes']

    # Fetch all the documents from the collection
    cursor = collection.find({})

    # Iterate over each document
    for document in cursor:
        type_name = document["type_name"]

        if type_name.lower() == "prompt":
            # change type_name to "Prompt"
            document["type_name"] = "Prompt"
            
            # Update node_content field
            node_content = document["node_content"]

            document["node_content"] = {"Prompt": node_content}
        
        elif type_name.lower() == "process":
            document["type_name"] = "Process"

            node_content = document["node_content"]

            document["node_content"] = {"Process": node_content}

        # Save the updated document back to the database
        collection.replace_one({"_id": document["_id"]}, document)

if __name__ == "__main__":
    update_mongodb_collection()
