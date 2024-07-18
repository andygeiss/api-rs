FROM scratch
ADD assets /assets
ADD data /data
ADD templates /templates
COPY server /server
EXPOSE 8080
ENTRYPOINT [ "/server" ]
