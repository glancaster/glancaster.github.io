import asyncio

async def task1():
    print('Task 1: Start')
    await asyncio.sleep(2)
    print('Task 1: Complete')

async def task2():
    print('Task 2: Start')
    await asyncio.sleep(1)
    print('Task 2: Complete')

async def main():
    tasks = [task1(), task2()]
    await asyncio.gather(*tasks)

asyncio.run(main())
