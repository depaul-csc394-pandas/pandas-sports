import dotenv
import os
import requests
import yaml

dotenv.load_dotenv()

d = os.getenv("DOMAIN")
if d is not None:
    DOMAIN = d
else:
    DOMAIN = 'localhost'

p = os.getenv("PORT")
if p is not None:
    PORT = int(p)
else:
    PORT = 8080

URL = 'http://{}:{}/api'.format(DOMAIN, PORT)

with open('test-data.yaml') as yaml_file:
    test_data = yaml.load(yaml_file)

print(test_data)

teams = test_data['teams']
for t in teams:
    print('creating team: {}'.format(t))
    r = requests.post(URL + '/teams', json=t)
    r.raise_for_status()

matches = test_data['matches']
for m in matches:
    print('creating match: {}'.format(m))
    r = requests.post(URL + '/matches', json=m)
    r.raise_for_status()
