Demo: https://alicebob.ru/

Алгоритм Price-Time Priority  
Один товар, один ордер за одну итерацию  
todo: улучшить тесты  

## Сборка ##
```
docker build -t matching_engine .
docker save -o matching_engine.tar matching_engine
gzip matching_engine.tar
scp -P port matching_engine.tar.gz user@ip:.
rm matching_engine.tar.gz
```

## Деплой ##
```
ssh user@ip -p port
docker stop matching_engine
docker rmi matching_engine
gunzip matching_engine.tar.gz
docker load -i matching_engine.tar
rm matching_engine.tar
docker run --rm -d --network exch --name matching_engine matching_engine
```


## То что не реализовано ##

рыночные ордера(лимитные с лучшей ценой), стоп-ордера(рыночные под капотом), айсберг-ордера(лимитные с отложенным исполнением), отмена ордеров  
пакетная обработка ордеров(здесь перейти на красно-чёрное дерево?)  

добавить больше тестов
