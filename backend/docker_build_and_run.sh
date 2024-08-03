sudo docker build -t dummy-image . 
sudo docker run -v ./.env:/app/.env -it dummy-image