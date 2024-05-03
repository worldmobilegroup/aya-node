import subprocess

def run_cardano_cli_command(command, service_container_name):
    try:
        # Construct the docker command to execute cardano-cli in the cardano-cli service
        docker_command = f"docker exec -it {service_container_name} {command}"
        result = subprocess.run(docker_command, shell=True, check=True, text=True, capture_output=True)
        return result.stdout
    except subprocess.CalledProcessError as e:
        print("Failed to execute command:", e)
        return e.output

# Example usage
if __name__ == "__main__":
    service_container_name = "aya_cardano-cli_1"  # Adjust this name based on your actual Docker container name
    cli_command = "cardano-cli query tip --testnet-magic 1097911063 --socket-path /cardano/data/socket/node.socket"
    output = run_cardano_cli_command(cli_command, service_container_name)
    print("Output from cardano-cli:", output)
