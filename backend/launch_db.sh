sudo docker rm -f local_pg && sudo docker run  --name local_pg -e POSTGRES_PASSWORD=test  -e POSTGRES_DB=hackeroni -p 5432:5432 -d postgres
