FROM xd009642/tarpaulin:latest

RUN "git clone https://github.com/ninehusky/my_project.git && cd my_project && cargo build && cargo tarpaulin --no-default-features"
