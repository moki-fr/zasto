
import requests
import json
import sys

SYSTEM_PROMPT = """You're in a tool called Zasto, it's an open source tool to free space, that scans a disks and uses AI to tell if heavy folders are worth keeping or not.

You will be given about a 100 paths sorted from heaviest to lightest with their sizes and you will have to output only the folders that you think are worth deleting, and a description of what it is.

## Important
You have 2 rules:

- Respond only with the paths, nothing else before or after, no 'Sure, here are the folders that are worth deleting..'
- You **HAVE** to use the correct formating, here are a, example:

"/home/user/.minecraft/"@Minecraft directory@6.2GB|"/home/user/download/"@Heavy files in download@23GB


Here's an explaination:
first you link the path of the folder between quotes, then you add a @ to separate the path to the comment and the size, and if you want to add another path with a comment and size, separate it with |
it can be represented as this:

"path"@comment@size|"path"@comment@size|"path"@comment@size

It's better not to set a comment which contains more than 7 words but it's not really important
Remember that you HAVE to set a comment, even if you don't know where that file is pointing to, just say "Unknown path" or something like that
And also you HAVE to set a size, even if you're not sure just say "? GB"

If you have any problem (like not having the paths, or missunderstood something) please replace your whole reply with "ai_thinking_error" """


def ai(api_key, model, userPrompt):
    try:
        ### Basic template of OpenRouter's Docs
        response = requests.post(
            url="https://openrouter.ai/api/v1/chat/completions",
            headers={
                "Authorization": f"Bearer {api_key}",
            },
            data=json.dumps({
                "model": f"{model}",
                "messages": [
                    {
                        "role": "system",
                        "content": f"{SYSTEM_PROMPT}"
                    },  
                    {  
                        "role": "user",
                        "content": f"{userPrompt}"
                    }
                ]
            })
        )

        # Get response
        data = response.json()

        # Get only the ai's response
        reply = data["choices"][0]["message"]["content"]

        if "@" in reply:
            return reply # Correct reply, returning

        elif "ai_thinking_error" in reply: # if ai misunderstood smth
            print("Error occured while AI was thinking of your request")
            print("You can retry, if it won't stop you can open an issue on our Github: https://github.com/moki-fr/zasto/issues")
            sys.exit(1)

        else: # Other error
            print("Error occured while contacting AI")
            print("Reply's format isn't correct:")
            print(reply)
            sys.exit(1)

    except Exception as e: # Py error
        print("Error occured while contacting AI")
        print(f"Py error: {e}")
        sys.exit(1)
