#!/usr/bin/env python
import asyncio
import websockets

async def hello():
    async with websockets.connect("ws://localhost:8765") as websocet:
            await websocket.send("Hello")
            await websocket.recv()


            