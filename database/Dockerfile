FROM mysql:latest

ENV MYSQL_ROOT_PASSWORD=123456
ENV MYSQL_DATABASE=study_vault
ENV MYSQL_USER=sv_user
ENV MYSQL_PASSWORD=123456

COPY database/db_script.sql /docker-entrypoint-initdb.d/
COPY database/my.cnf /etc/mysql/conf.d/

EXPOSE 3306
