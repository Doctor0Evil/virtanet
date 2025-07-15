import os
import requests
import msgpack


model_id = "MODEL_ID" # Replace with your model ID
deployment = "development" # `development`, `production`, or a deployment ID
baseten_api_key = os.environ["BASETEN_API_KEY"]
# Specify the URL to which you want to send the POST request
url = f"https://model-{model_id}.api.baseten.co/{deployment}/predict"
headers={
    "Authorization": f"Api-Key {baseten_api_key}",
    "content-type": "application/octet-stream",
}

with open('Gettysburg.mp3', 'rb') as file:
    response = requests.post(
        url,
        headers=headers,
        data=msgpack.packb({'byte_data': file.read()})
    )

print(response.status_code)
print(response.headers)
To support certain types like numpy and datetime values, you may need to extend client-side msgpack encoding with the same encoder and decoder used by Truss.

​
Parse raw bytes from model output
To use the output of a non-streaming model response, decode the response content.

call_model.py

Copy

Ask AI
# Continues `call_model.py` from above

binary_output = msgpack.unpackb(response.content)

# Change extension if not working with mp3 data
with open('output.mp3', 'wb') as file:
    file.write(binary_output["byte_data"])
​
Streaming binary outputs
You can also stream output as binary. This is useful for sending large files or reading binary output as it is generated.

In the model.py, you must create a streaming output.

model/model.py

Copy

Ask AI
# Replace the predict function in your Truss
def predict(self, model_input):
    import os

    current_dir = os.path.dirname(__file__)
    file_path = os.path.join(current_dir, "tmpfile.txt")
    with open(file_path, mode="wb") as file:
            file.write(bytes(model_input["text"], encoding="utf-8"))

    def iterfile():
        # Get the directory of the current file
        current_dir = os.path.dirname(__file__)
        # Construct the full path to the .wav file
        file_path = os.path.join(current_dir, "tmpfile.txt")
        with open(file_path, mode="rb") as file_like:
            yield from file_like

    return iterfile()
Then, in your client, you can use streaming output directly without decoding.

stream_model.py

import os
import requests
import json

model_id = "MODEL_ID" # Replace with your model ID
deployment = "development" # `development`, `production`, or a deployment ID
baseten_api_key = os.environ["BASETEN_API_KEY"]
# Specify the URL to which you want to send the POST request
url = f"https://model-{model_id}.api.baseten.co/{deployment}/predict"
headers={
    "Authorization": f"Api-Key {baseten_api_key}",
}

s = requests.Session()
with s.post(
    # Endpoint for production deployment, see API reference for more
    f"https://model-{model_id}.api.baseten.co/{deployment}/predict",
    headers={"Authorization": f"Api-Key {baseten_api_key}"},
    data=json.dumps({"text": "Lorem Ipsum"}),
    # Include stream=True as an argument so the requests libray knows to stream
    stream=True,
) as response:
    for token in response.iter_content(1):
        print(token) # Prints bytes
