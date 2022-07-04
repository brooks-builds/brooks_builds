FROM rust:latest
ADD . /code
EXPOSE 3000
ENV PORT=3000
HEALTHCHECK --interval=30s --timeout=30s --start-period=5s --retries=3 CMD curl -f http://localhost:3000/healthcheck
VOLUME [ "/code" ]
WORKDIR /code
RUN cargo build
STOPSIGNAL SIGTERM
CMD [ "cargo", "run" ]
