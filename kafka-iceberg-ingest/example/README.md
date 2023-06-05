wget https://raw.githubusercontent.com/debezium/debezium-examples/main/tutorial/docker-compose-mysql-apicurio.yaml

wget https://raw.githubusercontent.com/debezium/debezium-examples/main/tutorial/register-mysql-apicurio-converter-avro.json

# Start the topology as defined in https://debezium.io/documentation/reference/stable/tutorial.html
export DEBEZIUM_VERSION=2.1
docker-compose -f docker-compose-mysql-apicurio.yaml up

# Start MySQL connector
curl -i -X POST -H "Accept:application/json" -H  "Content-Type:application/json" http://localhost:8083/connectors/ -d @register-mysql-apicurio-converter-avro.json

# Consume messages from a Debezium topic
docker-compose -f docker-compose-mysql-apicurio.yaml exec kafka /kafka/bin/kafka-console-consumer.sh \
    --bootstrap-server kafka:9092 \
    --from-beginning \
    --property print.key=true \
    --topic dbserver1.inventory.customers

# Modify records in the database via MySQL client
docker-compose -f docker-compose-mysql-apicurio.yaml exec mysql bash -c 'mysql -u $MYSQL_USER -p$MYSQL_PASSWORD inventory'

# Shut down the cluster
docker-compose -f docker-compose-mysql-apicurio.yaml down
