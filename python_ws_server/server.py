#!/usr/bin/env python
import asyncio
import websockets

async def echo(websocket):
    remote_ip = websocket.remote_address[0]
    # Create File for this address and log out to file
    file = open('logs/' + remote_ip + '.txt', 'a+')
    async for message in websocket:
        print("From IP Address:", remote_ip)
        print("Message: ", message)
        file.write(message)
        # await websocket.send(message)

async def main():
    print("Starting Websocket Server...")
    async with websockets.serve(echo, "localhost", 8765):
        await asyncio.Future()

asyncio.run(main())