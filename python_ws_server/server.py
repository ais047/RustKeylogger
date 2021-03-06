#!/usr/bin/env python
import asyncio
import websockets

async def echo(websocket):
    remote_ip = websocket.remote_address[0]
    print('open')
    # Create File for this address and log out to file
    file = open('logs/' + remote_ip + '_ws.txt', 'a+')
    async for message in websocket:
        print("From IP Address:", remote_ip)
        print("Message: ", message)
        # Currently Writes to file on Client Close and Backwards
        file.write(message)
        file.close()

        # await websocket.send(message)

async def main():
    print("Starting Websocket Server...")
    async with websockets.serve(echo, "localhost", 8765):
        await asyncio.Future()

asyncio.run(main())