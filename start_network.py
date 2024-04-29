import subprocess
import time
import os
import re

def run_command(command, capture=False):
    """Execute a command in the shell and optionally capture the output."""
    print(f"Executing command: {command}")
    if capture:
        result = subprocess.run(command, shell=True, text=True, capture_output=True)
        return result.stdout.strip()
    else:
        subprocess.run(command, shell=True)

def fetch_logs_for_node_key():
    """Fetch logs from the Alice container and extract the node key."""
    print("Fetching logs and attempting to extract node key...")
    time.sleep(5)  # Ensure Alice is fully started and logs are generated
    command = "docker-compose logs alice"
    logs = run_command(command, capture=True)
    pattern = re.compile(r"Local node identity is: (\w+)")
    match = pattern.search(logs)
    if match:
        node_key = match.group(1)
        print(f"Found node key: {node_key}")
        return node_key
    else:
        print("Node key not found in the logs.")
        return None

def fetch_ip_address():
    """Fetch IP address for Alice from the Docker network."""
    print("Fetching IP address for Alice...")
    command = "docker inspect -f '{{range .NetworkSettings.Networks}}{{.IPAddress}}{{end}}' aya-alice-1"
    ip_address = run_command(command, capture=True)
    if ip_address:
        print(f"Found IP address: {ip_address}")
        return ip_address
    else:
        print("IP address not found.")
        return None

def start_network():
    """Main function to start network components including all dependencies."""
    print("Cleaning up old containers and networks...")
    run_command("docker-compose down -v")

    print("Starting all services...")
    run_command("docker-compose up -d --remove-orphans")  # Start all services at once

    time.sleep(20)  # Allow time for all services to initialize properly

    node_key = fetch_logs_for_node_key()
    alice_ip = fetch_ip_address()
    
    if node_key and alice_ip:
        print(f"Restarting Bob with Alice's node key: {node_key} and Alice's IP: {alice_ip}")
        os.environ['NODE_KEY'] = node_key
        os.environ['ALICE_IP'] = alice_ip
        # Restart Bob to apply the environment variables properly
        run_command("docker-compose stop bob")
        run_command("docker-compose up -d bob")
        print("Bob restarted successfully with updated settings.")
    else:
        print("Failed to start network components due to missing information.")

if __name__ == "__main__":
    start_network()
