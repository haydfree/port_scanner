import socket, time
from concurrent.futures import ThreadPoolExecutor

PORT_NUM = 1024

host = "192.168.0.1"

start_time = time.time()

def scan_port(port: int) -> None:
    s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    s.settimeout(0.05)

    result = s.connect_ex((host, port))

    if result == 0:
        print(f"port {port} is open on {host}")

    s.close()

with ThreadPoolExecutor(max_workers=12) as executor:
    executor.map(scan_port, range(PORT_NUM + 1))

end_time = time.time()

print(f"execution time: {end_time - start_time:.5f} s")

