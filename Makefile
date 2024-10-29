install: 
	pip install --upgrade pip && pip install -r requirements.txt

format:
	black *.py

lint:
	pylint --disable=R,C --ignore-patterns=test_.*?py *.py

test:
	python -m pytest -cov=main test_main.py -v

extract:
	python main.py extract

transform_load: 
	python main.py transform_load

query:
	python main.py general_query

	
all: install format lint test extract transform_load query
