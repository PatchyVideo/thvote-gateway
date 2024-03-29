FROM scratch

# These commands copy your files into the specified directory in the image
# and set that as the working location
COPY target/x86_64-unknown-linux-musl/release/thvote-gateway /webapp/app
WORKDIR /webapp

EXPOSE 80

# This command runs your application, comment out this line to compile only
CMD ["./app"]
