all:
	;

init-venv:
	virtualenv --python=python3 venv;\
	. venv/bin/activate;\
	pip install -r requirements.txt
