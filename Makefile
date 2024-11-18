pyinstall: 
	pip install --upgrade pip && pip install -r requirements.txt

pyformat:
	black *.py

pylint:
	pylint --disable=R,C --ignore-patterns=test_.*?py *.py

pytest:
	python -m pytest -cov=main test_main.py -v

	
pyall: pyinstall pyformat pylint pytest 


format:
	cargo fmt --quiet

lint:
	cargo clippy --quiet

test:
	cargo test --quiet

init:
	cargo run init train.csv
	
release:
	cargo build --release

install:
	# Install if needed
	# @echo "Updating rust toolchain"
	# rustup update stable
	# rustup default stable 

all: format lint test run release

# Custom tasks

read: 
	cargo run read

transform_load:
	cargo run transform_load

query:
	cargo run query "WITH mean_room_age as (SELECT HouseAge, AVG(AveRooms) as average_age_rooms FROM tbl_housing_data GROUP BY HouseAge ) SELECT * FROM tbl_housing_data t1 LEFT JOIN mean_room_age t2 ON t1.HouseAge = t2.HouseAge WHERE t1.AveRooms <= average_age_rooms"
