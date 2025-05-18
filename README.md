Demo: https://alicebob.ru/

Price-Time Priority algorithm 

## Build ##
```
docker build -t matching_engine .
docker save -o matching_engine.tar matching_engine
gzip matching_engine.tar
scp -P port matching_engine.tar.gz user@ip:.
rm matching_engine.tar.gz
```

## Deploy ##
```
ssh user@ip -p port
docker stop matching_engine
docker rmi matching_engine
gunzip matching_engine.tar.gz
docker load -i matching_engine.tar
rm matching_engine.tar
docker run --rm -d --network exch --name matching_engine matching_engine
```


todo: improve tests  

