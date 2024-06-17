

# # from dataclasses import dataclass
# # import psycopg2
# # from psycopg2.extras import RealDictCursor
# # import json

# # host = "database.cfsg8aggwxm9.us-east-1.rds.amazonaws.com"
# # username = "postgres"
# # password = "Sbikrc21916"
# # database = "postgres"

# # conn = psycopg2.connect(
# #     host=host,
# #     database=database,
# #     user=username,
# #     password=password
# # )

# # def lambda_handler(event, context):
# #     cur = conn.cursor(cursor_factory=RealDictCursor)
# #     cur.execute("SELECT * FROM people")
# #     results = cur.fetchall()
# #     json_result = json.dumps(results)
# #     print(json_result)
# #     return json_result

# # # Dummy event and context for local testing
# # dummy_event = {}
# # dummy_context = {}

# # lambda_handler(dummy_event, dummy_context)




# # import json
# # import psycopg2
# # import os

# # # Environment variables for RDS connection
# # RDS_HOST = "database.cfsg8aggwxm9.us-east-1.rds.amazonaws.com"
# # RDS_DB_NAME = "postgres"
# # RDS_USER = "postgres"
# # RDS_PASSWORD = "Sbikrc21916"


# # # Establish a connection to the PostgreSQL database
# # def get_db_connection():
# #     conn = psycopg2.connect(
# #         host=RDS_HOST,
# #         database=RDS_DB_NAME,
# #         user=RDS_USER,
# #         password=RDS_PASSWORD
# #     )
# #     return conn

# # def lambda_handler(event, context):
# #     # Parse the incoming IoT message
# #     iot_message = event['Records'][0]['Sns']['Message']
# #     data = json.loads(iot_message)

# #     # Extract data from IoT message
# #     # Adjust this part based on your IoT message structure
# #     state = data.get('state')
# #     timestamp = data.get('timestamp')

# #     # Connect to the database
# #     conn = get_db_connection()
# #     cur = conn.cursor()

# #     try:
# #         # Insert data into the PostgreSQL table
# #         insert_query = """
# #         INSERT INTO state_data (state, timestamp) VALUES (%s, %s);
# #         """
# #         cur.execute(insert_query, (state, timestamp))
# #         conn.commit()
# #     except Exception as e:
# #         print(f"Error: {e}")
# #         conn.rollback()
# #     finally:
# #         cur.close()
# #         conn.close()

# #     return {
# #         'statusCode': 200,
# #         'body': json.dumps('Data inserted successfully')
# #     }


# # dummy_event = {}
# # dummy_context = {}

# # lambda_handler(dummy_event, dummy_context)





# from dataclasses import dataclass
# import psycopg2
# from psycopg2.extras import RealDictCursor
# import json

# host = "database.cfsg8aggwxm9.us-east-1.rds.amazonaws.com"
# username = "postgres"
# password = "Sbikrc21916"
# database = "postgresdatabase"

# try:
#     conn = psycopg2.connect(
#         host=host,
#         database=database,
#         user=username,
#         password=password
#     )
#     print("Connection to the database was successful.")
# except Exception as e:
#     print(f"Failed to connect to the database: {e}")

# def lambda_handler(event, context):
#     try:
#         # Parse the SNS message
#         iot_message = event['Records'][0]['Sns']['Message']
#         print(f"IoT Message: {iot_message}")
        
#         # Proceed with the database operations
#         cur = conn.cursor(cursor_factory=RealDictCursor)
#         print("Cursor created successfully.")
        
#         cur.execute("SELECT * FROM public.people")
#         results = cur.fetchall()
#         print("Query executed successfully.")
        
#         json_result = json.dumps(results)
#         print(f"Query results: {json_result}")
        
#         return {
#             'statusCode': 200,
#             'body': json_result
#         }
#     except KeyError as e:
#         print(f"KeyError: {e}")
#         return {
#             'statusCode': 400,
#             'body': f"Missing key in event: {e}"
#         }
#     except Exception as e:
#         print(f"Failed to execute query: {e}")
#         return {
#             'statusCode': 500,
#             'body': 'Failed to execute query.'
#         }

# # Dummy event and context for local testing
# dummy_event = {
#   "Records": [
#     {
#       "Sns": {
#         "Message": "{\"key1\":\"value1\",\"key2\":\"value2\",\"key3\":\"value3\"}"
#       }
#     }
#   ]
# }
# dummy_context = {}

# lambda_handler(dummy_event, dummy_context)






import json
import psycopg2
from psycopg2.extras import RealDictCursor
import os

# RDS PostgreSQL connection parameters
host = "database.cfsg8aggwxm9.us-east-1.rds.amazonaws.com"
username = "postgres"
password = "Sbikrc21916"
database = "postgres"


try:
    conn = psycopg2.connect(
        host=host,
        database=database,
        user=username,
        password=password
    )
    print("Connection to the database was successful.")
except Exception as e:
    print(f"Failed to connect to the database: {e}")

def lambda_handler(event, context):
    try:
        # Print event for debugging purposes
        print(f"Received event: {json.dumps(event)}")
        
        # Extract the message from the event
        iot_message = json.loads(event['Records'][0]['Sns']['Message'])
        print(f"IoT Message: {iot_message}")
        
        # Extract values from the IoT message
        id = iot_message.get('id')
        name = iot_message.get('name')
        location = iot_message.get('location')
        data = iot_message.get('data')
        
        if id is None or name is None or location is None or data is None:
            raise ValueError("Missing required fields in the IoT message.")
        
        # Proceed with the database operations
        cur = conn.cursor(cursor_factory=RealDictCursor)
        print("Cursor created successfully.")
        
        # Insert the data into the people table
        insert_query = """
        INSERT INTO public.people (id, name, location, data) 
        VALUES (%s, %s, %s, %s)
        """
        cur.execute(insert_query, (id, name, location, data))
        conn.commit()
        print("Data inserted successfully.")
        
        return {
            'statusCode': 200,
            'body': json.dumps({'message': 'Data inserted successfully.'})
        }
    except KeyError as e:
        print(f"KeyError: {e}")
        return {
            'statusCode': 400,
            'body': f"Missing key in event: {e}"
        }
    except ValueError as e:
        print(f"ValueError: {e}")
        return {
            'statusCode': 400,
            'body': str(e)
        }
    except Exception as e:
        print(f"Failed to execute query: {e}")
        return {
            'statusCode': 500,
            'body': 'Failed to execute query.'
        }

# For local testing
if __name__ == "__main__":
    dummy_event = {
        "Records": [
            {
                "Sns": {
                    "Message": json.dumps({
                        "id": 4,
                        "name": "Humidity sensor",
                        "location": "Bangalore office 147 block",
                        "data": "45 PPM"
                    })
                }
            }
        ]
    }
    dummy_context = {}
    lambda_handler(dummy_event, dummy_context)
