#!/usr/bin/python3

import requests

URL = "http://localhost:3030/pushEvent"

headers = {
    'Accept': 'application/json',
    'Content-type': 'application/json'
}

def send(message):
    return requests.post(URL, headers=headers, json={
        "level": "info",
        "message": message,
    }).json()

if __name__ == "__main__":
    while True:
        try:
            message = input()
            send(message)
        except:
            break
