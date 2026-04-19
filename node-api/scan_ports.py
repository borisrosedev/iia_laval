import socket

target = "127.0.0.1"

print(f"Scan des ports sur {target}...\n")

for port in range(2990, 3010):
    s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    socket.setdefaulttimeout(0.5)

    result = s.connect_ex((target, port))

    if result == 0:
        print(f"Port {port} ouvert")

    s.close()
