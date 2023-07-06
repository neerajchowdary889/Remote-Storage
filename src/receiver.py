import socket
import tqdm
import os
import threading
import sys
import asyncio
import sqlite3

SERVER_HOST = "0.0.0.0"
SERVER_PORT = 5100
BUFFER_SIZE = 4096
SEPARATOR = "<SEPARATOR>"
DATABASE_FILE = "credits.db"  # Construct the absolute file path dynamically

async def run_ipfs_command(command):
    process = await asyncio.create_subprocess_shell(
        command,
        stdout=asyncio.subprocess.PIPE,
        stderr=asyncio.subprocess.PIPE
    )
    stdout, stderr = await process.communicate()
    return stdout.decode().strip()

async def upload_to_ipfs(file_path):
    command = f"ipfs add -Q {file_path}"
    cid = await run_ipfs_command(command)
    return cid

async def handle_client(client_socket):
    received = client_socket.recv(BUFFER_SIZE).decode()
    filename, filesize, totalhash = received.split(SEPARATOR)
    filename = os.path.basename(filename)
    filesize = int(filesize)

    directory = f"UserFolders/{totalhash}"
    os.makedirs(directory, exist_ok=True)
    filepath = os.path.join(directory, filename)

    progress = tqdm.tqdm(range(filesize), f"Receiving {filename}", unit="B", unit_scale=True, unit_divisor=1024)
    with open(filepath, "wb") as f:
        while True:
            bytes_read = client_socket.recv(BUFFER_SIZE)
            if not bytes_read:
                break
            f.write(bytes_read)
            progress.update(len(bytes_read))

    client_socket.close()

    cid = await upload_to_ipfs(filepath)
    print(f"File uploaded to IPFS. CID: {cid}")

    # Update the SQLite database with the CID and filename
    conn = sqlite3.connect(DATABASE_FILE)
    c = conn.cursor()

    # Check if table exists
    c.execute("SELECT name FROM sqlite_master WHERE type='table' AND name=?", (totalhash,))
    result = c.fetchone()

    if result is None:
        # Create the table if it doesn't exist
        create_table_query = f"CREATE TABLE IF NOT EXISTS '{totalhash}' (filename TEXT, cid TEXT, totalhash TEXT)"
        c.execute(create_table_query)
    # Update the table with filename and cid
    insert_query = f"INSERT INTO '{totalhash}' (filename, cid) VALUES (?, ?)"
    c.execute(insert_query, (filename, cid))

    conn.commit()
    conn.close()

async def start_server():
    s = socket.socket()
    s.bind((SERVER_HOST, SERVER_PORT))
    s.listen(5)
    print(f"[*] Listening as {SERVER_HOST}:{SERVER_PORT}")

    while True:
        client_socket, address = await loop.run_in_executor(None, s.accept)
        print(f"[+] {address} is connected.")

        asyncio.create_task(handle_client(client_socket))

    s.close()

loop = asyncio.get_event_loop()
loop.run_until_complete(start_server())

