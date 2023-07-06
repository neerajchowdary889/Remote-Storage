# # # import socket
# # # import tqdm

# # # server = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
# # # server.bind(('localhost', 5100))
# # # server.listen()

# # # print("Server is listening on Port 5100")

# # # client, address = server.accept()

# # # file_name = client.recv(1024).decode()
# # # print(file_name)
# # # file_size = client.recv(1024).decode()
# # # print(file_size)

# # # file = open(file_name, "wb")
# # # file_bytes = b""

# # # done = False

# # # while not done:
# # #     data = client.recv(1024)
# # #     if file_bytes[-5:] == b"<EOF>":
# # #         done = True
# # #     else:
# # #         file_bytes += data

# # # file.write(file_bytes)
# # # file.close()
# # # client.close()
# # # server.close()

# # import socket
# # import os

# # BUFFER_SIZE = 4096

# # server = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
# # server.bind(('localhost', 5100))
# # server.listen(1)

# # print("Server started. Listening for connections...")

# # while True:
# #     conn, addr = server.accept()
# #     print(f"Connection established from {addr}")

# #     # Receive the file name and size from the client
# #     file_name = conn.recv(BUFFER_SIZE).decode()
# #     print(file_name)
# #     file_size = int(conn.recv(BUFFER_SIZE).decode())
# #     print(file_size)
# #     totalhash = conn.recv(BUFFER_SIZE).decode()

# #     print(f"Received file: {file_name} ({file_size} bytes)")

# #     # Send a string back to the client
# #     string_data = "String data from server"
# #     conn.send(string_data.encode())

# #     # Create the folder to save the file
# #     folder_name = f"UserFolder/{string_data}"
# #     if not os.path.exists(folder_name):
# #         os.makedirs(folder_name)

# #     # Save the file in the folder
# #     file_path = os.path.join(folder_name, file_name)
# #     with open(file_path, "wb") as file:
# #         progress = 0
# #         print("69 line")
# #         while progress < file_size:
# #             data = conn.recv(BUFFER_SIZE)
# #             print("line 72")
# #             if not data:
# #                 break
# #             file.write(data)
# #             progress += len(data)
# #             print(f"Received {progress} bytes of {file_size} bytes ({progress/file_size*100:.2f}%)")

# #     print(f"File received: {file_path}")

# #     conn.close()



# import socket
# import tqdm
# import os
# # device's IP address
# SERVER_HOST = "0.0.0.0"
# SERVER_PORT = 5100
# # receive 4096 bytes each time
# BUFFER_SIZE = 4096
# SEPARATOR = "<SEPARATOR>"
# # create the server socket
# # TCP socket
# s = socket.socket()
# # bind the socket to our local address
# s.bind((SERVER_HOST, SERVER_PORT))
# # enabling our server to accept connections
# # 5 here is the number of unaccepted connections that
# # the system will allow before refusing new connections
# s.listen(5)
# print(f"[*] Listening as {SERVER_HOST}:{SERVER_PORT}")
# # accept connection if there is any
# client_socket, address = s.accept() 
# # if below code is executed, that means the sender is connected
# print(f"[+] {address} is connected.")
# # receive the file infos
# # receive using client socket, not server socket
# received = client_socket.recv(BUFFER_SIZE).decode()
# filename, filesize, totalhash = received.split(SEPARATOR)
# # remove absolute path if there is
# filename = os.path.basename(filename)
# # convert to integer
# filesize = int(filesize)
# # Create the directory if it doesn't exist
# directory = f"UserFolders/{totalhash}"
# os.makedirs(directory, exist_ok=True)

# # Define the file path
# filepath = os.path.join(directory, filename)

# # start receiving the file from the socket
# # and writing to the file stream
# progress = tqdm.tqdm(range(filesize), f"Receiving {filename}", unit="B", unit_scale=True, unit_divisor=1024)
# with open(filepath, "wb") as f:
#     while True:
#         # read 1024 bytes from the socket (receive)
#         bytes_read = client_socket.recv(BUFFER_SIZE)
#         if not bytes_read:    
#             # nothing is received
#             # file transmitting is done
#             break
#         # write to the file the bytes we just received
#         f.write(bytes_read)
#         # update the progress bar
#         progress.update(len(bytes_read))

# # close the client socket
# client_socket.close()
# # close the server socket
# s.close()



import socket
import tqdm
import os
import threading
import sys
SERVER_HOST = "0.0.0.0"
SERVER_PORT = 5100
BUFFER_SIZE = 4096
SEPARATOR = "<SEPARATOR>"

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

