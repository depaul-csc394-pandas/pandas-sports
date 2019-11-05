import dotenv
import os
import requests
import yaml

METHODS = ['post', 'get', 'delete']

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

def validate_request(req):
    if req['method'] is None or req['method'] not in METHODS:
        print('Request has invalid method or method is None:\n{}'.format(req))
        return False

def check_status(req, resp):
    if resp.status_code != req['response']['status']:
        print('Unexpected status code for request (expected {}, got {}): {}'
              .format(req['response']['status'], resp.status_code, req))
        exit(1)

with open('test-data.yaml') as yaml_file:
    test_data = yaml.load(yaml_file)

reqs = test_data['requests']
for req in reqs:
    validate_request(req)
    method = req['method'].lower()
    full_url = URL + req['route']
    if method == 'post':
        resp = requests.post(full_url, json=req['body'])
    elif method == 'get':
        resp = requests.get(full_url)
    elif method == 'delete':
        resp = requests.delete(full_url)
    else:
        print('Method ({}) not recognized!'.format(method))
        exit(1)
    check_status(req, resp)

