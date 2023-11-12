# Define default target
.PHONY: all
all: containers-up

# Path to the directory containing docker-compose file
COMPOSE_PATH = ./devs

# Define a variable for the docker-compose command with the file path
COMPOSE = docker-compose -f $(COMPOSE_PATH)/docker-compose.yml

# Build the project
.PHONY: containers-build
containers-build:
	$(COMPOSE) build

# Start up the project
.PHONY: containers-up
containers-up:
	$(COMPOSE) up

# Stop the project
.PHONY: containers-down
containers-down:
	$(COMPOSE) down

# View logs
.PHONY: containers-logs
containers-logs:
	$(COMPOSE) logs

# Stop and remove volumes
.PHONY: containers-clean
containers-clean:
	$(COMPOSE) down -v

# Rebuild and start the project
.PHONY: containers-rebuild
containers-rebuild: containers-clean containers-build containers-up

# Target to enter a service's bash
# Usage: make shell service=postgres
.PHONY: containers-shell
containers-shell:
	$(COMPOSE) exec $(service) /bin/bash
