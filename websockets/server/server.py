import asyncio
import websockets
import json


async def echo(websocket):
    async for message in websocket:
        response = {
            'error': None,
            'result': message
        }
        print("Got: ", message)
        await websocket.send(json.dumps(response))


async def main():
    async with websockets.serve(echo, "localhost", 8765):
        await asyncio.Future()  # run forever


asyncio.run(main())
