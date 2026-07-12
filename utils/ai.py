
import requests
import json
import sys




def ai(api_key, model, systemPrompt, userPrompt):
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
                        "content": f"{systemPrompt}"
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
            return reply

        elif "ai_thinking_error" in reply:
            print("Error occured while AI was thinking of your request")
            print("You can retry, if it won't stop you can open an issue on our Github: https://github.com/moki-fr/zasto/issues")
            sys.exit(1)

        else:
            print("Error occured while contacting AI")
            print("Reply's format isn't correct:")
            print(reply)
            sys.exit(1)

    except Exception as e: 
        print("Error occured while contacting AI")
        print(f"Py error: {e}")
        sys.exit(1)
