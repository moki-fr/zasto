
import requests
import json

def ai(api_key, model, systemPrompt, userPrompt):
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
    
    return reply