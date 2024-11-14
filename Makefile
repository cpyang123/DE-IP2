pyinstall: 
	pip install --upgrade pip && pip install -r requirements.txt

pyformat:
	black *.py

pylint:
	pylint --disable=R,C --ignore-patterns=test_.*?py *.py

pytest:
	python -m pytest -cov=main test_main.py -v

pyextract:
	python main.py extract

pytransform_load: 
	python main.py transform_load

pyquery:
	python main.py general_query

	
pyall: install format lint test extract transform_load query


format:
	cargo fmt --quiet

lint:
	cargo clippy --quiet

test:
	cargo test --quiet

run:
	cargo run
release:
	cargo build --release

install:
	# Install if needed
	# @echo "Updating rust toolchain"
	# rustup update stable
	# rustup default stable 

all: format lint test run

# Custom tasks

extract: 
	cargo run extract

transform_load:
	cargo run transform_load

create:
	cargo run query "INSERT INTO ServeTimesDB (server, seconds_before_next_point, day, opponent, game_score, sets, game) VALUES ('John Doe', 40, '2023-09-05', 'Jane Doe', '30-40', 1, '0-0');"

read:
	cargo run query "SELECT * FROM ServeTimesDB WHERE server = 'John Doe';"

update:
	cargo run query "UPDATE ServeTimesDB SET server='John Doe', seconds_before_next_point=40, day='2023-09-05', opponent='Jane Doe', game_score='30-40', sets=1, game='0-0' WHERE id=1;"

delete:
	cargo run query "DELETE FROM ServeTimesDB WHERE id=3;"