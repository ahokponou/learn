# Basic Dockerfile

Build a basic Dockerfile to create a Docker image.

## Objective 

Create a basic Dockerfile to create a Docker image.

When this Docker image is run, 
it shoud print `Hello, Captain!` to console before exiting.

## Dockerfile

```dockerfile
FROM alpine:latest

ENV NAME="Captain"

ENTRYPOINT ["sh", "-c", "echo Hello, $NAME!"]
```

## Commands

```shell
docker build -t sayhello .

docker run --rm sayhello
# Output: Hello, Captain!

docker run --rm -e NAME="Flora" sayhello
# Output: Hello, Flora!
```

## Resources

- [Dockerfile Reference](https://docs.docker.com/reference/dockerfile)