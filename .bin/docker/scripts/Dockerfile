FROM postgres:latest

ENV READONLY_ROLE readonly
ENV DATABASE_USER "morgaine"

WORKDIR /usr/app/morgaine/scripts

COPY ../../scripts .

RUN chmod +x *.sh

CMD [ "./init.sh" ]