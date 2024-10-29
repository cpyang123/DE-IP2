default_query = """
    WITH mean_room_age as (
        SELECT HouseAge, AVG(AveRooms) as average_age_rooms
        FROM tbl_housing_data
        GROUP BY HouseAge
    )
    SELECT *
    FROM tbl_housing_data t1
    LEFT JOIN mean_room_age t2
    ON t1.HouseAge = t2.HouseAge
    WHERE t1.AveRooms <= average_age_rooms
"""