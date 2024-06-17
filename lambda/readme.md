# IoT Data Ingestion Lambda Function
This project contains an AWS Lambda function written in Python, designed to ingest IoT data and store it in an Amazon RDS PostgreSQL database.

# Overview
This Lambda function is triggered by an SNS message, extracts data from the message, and inserts it into a PostgreSQL database running on Amazon RDS.

# Prerequisites
1. Python 3.7+
2. PostgreSQL database (Amazon RDS instance)
3. AWS account with access to Lambda, SNS, and RDS
4. psycopg2-binary library for PostgreSQL database connection

# Install dependencies:
pip install psycopg2-binary


## Configuration
1. Database Configuration:
# Ensure that your PostgreSQL database is running and accessible. Update the connection parameters in the script

1. host = "your-database-endpoint"
2. username = "your-username"
3. password = "your-password"
4. database = "your-database"

2. AWS Configuration:
# Ensure you have the necessary AWS permissions to deploy and invoke Lambda functions, publish SNS messages, and access the RDS database.



## Running the Script Locally
1. Prepare a dummy event for local testing. The event should mimic the structure of the actual SNS message:
2. Run the script: "python3 lambda_function.py"






