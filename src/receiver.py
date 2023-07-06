import socket
import tqdm
import os
import threading
import sys
import ipfshttpclient
import asyncio
import sqlite3

SERVER_HOST = "0.0.0.0"
SERVER_PORT = 5100
BUFFER_SIZE = 4096
SEPARATOR = "<SEPARATOR>"
DATABASE_FILE = "credits.db"  # Construct the absolute file path dynamically

async def upload_to_ipfs(file_path):
    client = ipfshttpclient.connect()  # Connect to the local IPFS daemon

    res = client.add(file_path)  # Upload the file to IPFS

    cid = res["Hash"]  # Get the content identifier (CID) of the uploaded file

    return cid

def handle_client(client_socket):
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

    cid = await upload_to_ipfs(filename)
    print(f"File uploaded to IPFS. CID: {cid}")

    # Update the SQLite database with the CID and filename
    conn = sqlite3.connect(DATABASE_FILE)
    c = conn.cursor()

    c.execute(f"UPDATE {totalhash} SET filename = ?, cid = ? WHERE totalhash = ?",
              (filename, cid , totalhash))

    conn.commit()
    conn.close()

def start_server():
    s = socket.socket()
    s.bind((SERVER_HOST, SERVER_PORT))
    s.listen(5)
    print(f"[*] Listening as {SERVER_HOST}:{SERVER_PORT}")

    while True:
        client_socket, address = s.accept()
        print(f"[+] {address} is connected.")

        client_thread = threading.Thread(target=handle_client, args=(client_socket,))
        client_thread.start()

    s.close()
start_server()
# def main():
#     print("Error, Server not started...")


# if __name__ == "__main__":
#     if len(sys.argv) > 1 and sys.argv[1] == "StartServer":
#         start_server()
#     else:
#         main()

