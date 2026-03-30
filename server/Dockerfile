FROM scratch

COPY target/x86_64-unknown-linux-musl/release/cpu_monitor /cpu_monitor
COPY index.html /index.html

WORKDIR /
EXPOSE 3000
CMD ["/cpu_monitor"]
