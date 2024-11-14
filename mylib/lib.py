import pandas as pd
from sqlalchemy import create_engine
from sqlalchemy import Column, Integer, Float, create_engine
from sqlalchemy.ext.declarative import declarative_base
from sqlalchemy import text
import sqlite3

# Create an SQLite database (or connect to an existing one)
# engine = create_engine('sqlite:///example.db')

def create_connect_database(db_name: str):
    Base = declarative_base()
    engine = create_engine('sqlite:///./database/' + db_name)
    class HousePrice(Base):
        __tablename__ = 'tbl_house_prices'
        
        # ID column, set to auto-increment
        id = Column(Integer, primary_key=True, autoincrement=True)
        MedInc = Column(Float)
        HouseAge = Column(Float)
        AveRooms = Column(Float)
        AveBedrms = Column(Float)
        Population = Column(Integer)
        AveOccup = Column(Float)
        Latitude = Column(Float)
        Longitude = Column(Float)
        MedHouseVal = Column(Float)
    
    Base.metadata.create_all(engine)
    return engine

def load_dataframe(df_path):
    return pd.read_csv(df_path)

# Write the data into a new SQLite table (replace or append mode)
def load_to_table(dataframe, name, connection):
    dataframe.drop(columns=['id'])
    dataframe.to_sql(name, con=connection, if_exists='replace', index=False)

def select_all(table_name, connection):
    result = connection.execute(text("SELECT * FROM " + table_name))
    # Get the column names
    column_names = result.keys()
    
    # Fetch all rows
    rows = result.fetchall()

    # Print column names
    print(" | ".join(column_names))
    
    return rows

def select_id(table_name, id, connection):
    result = connection.execute(text("SELECT * FROM " + table_name + " WHERE id = {}".format(id)))
    
    # Get the column names
    column_names = result.keys()
    
    # Fetch all rows
    rows = result.fetchall()

    # Print column names
    print(" | ".join(column_names))
    
    return rows

# Function to insert records into the tbl_house_prices table
def insert_house_price_record(MedInc, HouseAge, AveRooms, AveBedrms, Population, AveOccup, Latitude, Longitude, MedHouseVal):
    try:
        # Connect to the SQLite database (replace 'your_database.db' with the actual database name)
        conn = sqlite3.connect('./database/demo.db')
        cursor = conn.cursor()
        
        # Query the current maximum ID in the table
        cursor.execute('SELECT MAX(id) FROM tbl_house_prices')
        max_id_result = cursor.fetchone()[0]
        
        # If no records exist, start from 1, otherwise increment the max_id
        new_id = (max_id_result or 0) + 1
        
        # SQL command to insert a record with the manually incremented ID
        insert_query = '''
            INSERT INTO tbl_house_prices (id, MedInc, HouseAge, AveRooms, AveBedrms, Population, AveOccup, Latitude, Longitude, MedHouseVal)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        '''

        # Data tuple with the new ID
        data_tuple = (new_id, MedInc, HouseAge, AveRooms, AveBedrms, Population, AveOccup, Latitude, Longitude, MedHouseVal)
        
        # Execute the SQL command
        cursor.execute(insert_query, data_tuple)
        
        # Commit the transaction
        conn.commit()
        
        # Return the new ID
        print(f"Record inserted successfully with ID: {new_id}")
        return new_id
        
    except sqlite3.Error as error:
        print("Failed to insert record into SQLite table", error)
    
    finally:
        if conn:
            # Close the connection
            conn.close()
            
def update_house_price_record(id, MedInc=None, HouseAge=None, AveRooms=None, AveBedrms=None, Population=None, AveOccup=None, Latitude=None, Longitude=None, MedHouseVal=None):
    try:
        # Connect to the SQLite database 
        conn = sqlite3.connect('./database/demo.db')
        cursor = conn.cursor()

        # Start building the update query dynamically based on the provided arguments
        update_query = 'UPDATE tbl_house_prices SET '
        params = []

        # Check which columns are provided for update and add them to the query
        if MedInc is not None:
            update_query += 'MedInc = ?, '
            params.append(MedInc)
        if HouseAge is not None:
            update_query += 'HouseAge = ?, '
            params.append(HouseAge)
        if AveRooms is not None:
            update_query += 'AveRooms = ?, '
            params.append(AveRooms)
        if AveBedrms is not None:
            update_query += 'AveBedrms = ?, '
            params.append(AveBedrms)
        if Population is not None:
            update_query += 'Population = ?, '
            params.append(Population)
        if AveOccup is not None:
            update_query += 'AveOccup = ?, '
            params.append(AveOccup)
        if Latitude is not None:
            update_query += 'Latitude = ?, '
            params.append(Latitude)
        if Longitude is not None:
            update_query += 'Longitude = ?, '
            params.append(Longitude)
        if MedHouseVal is not None:
            update_query += 'MedHouseVal = ?, '
            params.append(MedHouseVal)

        # Remove the trailing comma and space
        update_query = update_query.rstrip(', ')

        # Add the WHERE clause to target the record by ID
        update_query += ' WHERE id = ?'
        params.append(id)

        # Execute the SQL command
        cursor.execute(update_query, tuple(params))
        
        # Commit the transaction
        conn.commit()

        # Check if any row was updated
        if cursor.rowcount > 0:
            print(f"Record with ID: {id} updated successfully.")
            return True
        else:
            print(f"No record found with ID: {id}.")
            return False

    except sqlite3.Error as error:
        print("Failed to update record in SQLite table", error)
        return False

    finally:
        if conn:
            # Close the connection
            conn.close()

def delete_record(table_name, record_id, connection):
    try:
        # Use text() to wrap the raw SQL query
        query = text(f"DELETE FROM {table_name} WHERE id = :id")
        
        # Execute the delete query
        result = connection.execute(query, {"id": record_id})
        
        # Check if any rows were affected (i.e., deleted)
        if result.rowcount > 0:
            print(f"Record with ID {record_id} deleted successfully.")
        else:
            print(f"No record found with ID {record_id}.")

    except Exception as e:
        print(f"Error deleting record: {e}")




