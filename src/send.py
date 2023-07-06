import socket
import tqdm
import os
import sys

def sendfile(total_hash, Filename):
    SEPARATOR = "<SEPARATOR>"
    BUFFER_SIZE = 4096 # send 4096 bytes each time step
    # the ip address or hostname of the server, the receiver
    # host = "127.0.0.1"
    host = "192.168.126.50"
    # the port, let's use 5001
    port = 5100
    # the name of file we want to send, make sure it exists
    # filename = "ComputerNetworks-Abstract.pdf"
    filename = Filename
    # get the file size
    filesize = os.path.getsize(filename)
    totalhash = total_hash
    # create the client socket
    s = socket.socket()
    print(f"[+] Connecting to {host}:{port}")
    s.connect((host, port))
    print("[+] Connected.")
    # send the filename and filesize
    s.send(f"{filename}{SEPARATOR}{filesize}{SEPARATOR}{totalhash}".encode())
    # start sending the file
    progress = tqdm.tqdm(range(filesize), f"Sending {filename}", unit="B", unit_scale=True, unit_divisor=1024)
    with open(filename, "rb") as f:
        while True:
            # read the bytes from the file
            bytes_read = f.read(BUFFER_SIZE)
            if not bytes_read:
                # file transmitting is done
                break
            # we use sendall to assure transimission in 
            # busy networks
            s.sendall(bytes_read)
            # update the progress bar
            progress.update(len(bytes_read))
    # close the socket
    s.close()

def main():
    # Code for other functionality if any
    print("Executing main()")
    print("error")
    sendfile("43779aafe3d07dcddefa257eb32b9752b2cd5193","ComputerNetworks-Abstract.pdf")

if __name__ == "__main__":
    if len(sys.argv) > 1 and sys.argv[1] == "sendfile":
        total_hash = sys.argv[2]  # Retrieve the total hash from the command-line argument
        filename = sys.argv[3]
        sendfile(total_hash, filename)
    else:
        main()