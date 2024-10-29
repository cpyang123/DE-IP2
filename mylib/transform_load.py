import os
from databricks import sql
import pandas as pd
from dotenv import load_dotenv

def load(dataset="data/housing_data.csv"):
    """Transforms and Loads data into the local databricks database"""
    df = pd.read_csv(dataset, delimiter=",")
    load_dotenv()
    server_h = os.getenv("SERVER_HOST")
    access_token = os.getenv("DATABRICKS_ACCESS_TOKEN")
    http_path = os.getenv("HTTP_PATH")
    # print(df)
    with sql.connect(
        server_hostname=server_h,
        http_path=http_path,
        access_token=access_token,
    ) as connection:
        c = connection.cursor()
        # INSERT TAKES TOO LONG
        #c.execute("DROP TABLE IF EXISTS tbl_housing_data")
        c.execute("SHOW TABLES FROM default LIKE 'tbl_housing*'")
        result = c.fetchall()
        # takes too long so not dropping anymore
        # c.execute("DROP TABLE IF EXISTS EventTimesDB")
        if not result:
            
            c.execute(
                """
                CREATE TABLE tbl_housing_data (
                    id INT,
                    MedInc DOUBLE,
                    HouseAge DOUBLE,
                    AveRooms DOUBLE,
                    AveBedrms DOUBLE,
                    Population DOUBLE,
                    AveOccup DOUBLE,
                    Latitude DOUBLE,
                    Longitude DOUBLE,
                    MedHouseVal DOUBLE
                )
            """
            )
            # insert
            for _, row in df.iterrows():
                convert = (_,) + tuple(row[1:])
                print(convert)
                c.execute(f"INSERT INTO tbl_housing_data VALUES {convert}")
        c.execute("SHOW TABLES FROM default LIKE 'tbl_housing*'")
        result = c.fetchall()