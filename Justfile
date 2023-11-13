dev: 
    (cd applications/web && trunk serve)

build:
    docker build -t todolist . 
    
start:
    docker run -p 8080:8080 todolist
